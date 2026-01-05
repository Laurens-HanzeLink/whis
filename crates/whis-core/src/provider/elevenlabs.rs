//! ElevenLabs Scribe transcription provider
//!
//! ElevenLabs Scribe claims the highest accuracy in the market with ~3.3% English WER.
//! Uses multipart form upload with a different response structure.

use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::Deserialize;

use super::base::retry::{RetryConfig, is_rate_limited, is_retryable_error, is_retryable_status};
use super::{
    DEFAULT_TIMEOUT_SECS, TranscriptionBackend, TranscriptionRequest, TranscriptionResult,
    TranscriptionStage,
};

const API_URL: &str = "https://api.elevenlabs.io/v1/speech-to-text";
const MODEL: &str = "scribe_v1";

#[derive(Deserialize)]
struct Response {
    text: String,
}

/// ElevenLabs Scribe transcription provider
///
/// Uses ElevenLabs' Scribe model for high-accuracy transcription.
/// Supports 99 languages with speaker diarization for up to 32 speakers.
/// Priced at $0.40/hour.
#[derive(Debug, Default, Clone)]
pub struct ElevenLabsProvider;

#[async_trait]
impl TranscriptionBackend for ElevenLabsProvider {
    fn name(&self) -> &'static str {
        "elevenlabs"
    }

    fn display_name(&self) -> &'static str {
        "ElevenLabs Scribe"
    }

    fn transcribe_sync(
        &self,
        api_key: &str,
        request: TranscriptionRequest,
    ) -> Result<TranscriptionResult> {
        // Report uploading stage
        request.report(TranscriptionStage::Uploading);

        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(DEFAULT_TIMEOUT_SECS))
            .build()
            .context("Failed to create HTTP client")?;

        let config = RetryConfig::default();
        let mut attempt = 0;

        loop {
            let mut form = reqwest::blocking::multipart::Form::new()
                .text("model_id", MODEL)
                .part(
                    "file",
                    reqwest::blocking::multipart::Part::bytes(request.audio_data.clone())
                        .file_name(request.filename.clone())
                        .mime_str(&request.mime_type)?,
                );

            if let Some(lang) = request.language.clone() {
                form = form.text("language_code", lang);
            }

            // Report transcribing stage
            request.report(TranscriptionStage::Transcribing);

            let result = client
                .post(API_URL)
                .header("xi-api-key", api_key)
                .multipart(form)
                .send();

            match result {
                Ok(response) => {
                    let status = response.status();

                    if status.is_success() {
                        let text = response.text().context("Failed to get response text")?;
                        let resp: Response = serde_json::from_str(&text)
                            .context("Failed to parse ElevenLabs API response")?;
                        return Ok(TranscriptionResult { text: resp.text });
                    }

                    // Check if error is retryable
                    if is_retryable_status(status) && attempt < config.max_retries {
                        let delay = config.delay_for_attempt(attempt, is_rate_limited(status));
                        crate::verbose!(
                            "ElevenLabs request failed with {} (attempt {}/{}), retrying in {:?}",
                            status,
                            attempt + 1,
                            config.max_retries,
                            delay
                        );
                        std::thread::sleep(delay);
                        attempt += 1;
                        continue;
                    }

                    // Non-retryable error or max retries exceeded
                    let error_text = response
                        .text()
                        .unwrap_or_else(|_| "Unknown error".to_string());
                    anyhow::bail!("ElevenLabs API error ({status}): {error_text}");
                }
                Err(err) => {
                    // Check if network error is retryable
                    if is_retryable_error(&err) && attempt < config.max_retries {
                        let delay = config.delay_for_attempt(attempt, false);
                        crate::verbose!(
                            "ElevenLabs request failed with network error (attempt {}/{}), retrying in {:?}: {}",
                            attempt + 1,
                            config.max_retries,
                            delay,
                            err
                        );
                        std::thread::sleep(delay);
                        attempt += 1;
                        continue;
                    }

                    return Err(err).context("Failed to send request to ElevenLabs API");
                }
            }
        }
    }

    async fn transcribe_async(
        &self,
        client: &reqwest::Client,
        api_key: &str,
        request: TranscriptionRequest,
    ) -> Result<TranscriptionResult> {
        // Report uploading stage
        request.report(TranscriptionStage::Uploading);

        let config = RetryConfig::default();
        let mut attempt = 0;

        loop {
            let mut form = reqwest::multipart::Form::new()
                .text("model_id", MODEL)
                .part(
                    "file",
                    reqwest::multipart::Part::bytes(request.audio_data.clone())
                        .file_name(request.filename.clone())
                        .mime_str(&request.mime_type)?,
                );

            if let Some(lang) = request.language.clone() {
                form = form.text("language_code", lang);
            }

            // Report transcribing stage
            request.report(TranscriptionStage::Transcribing);

            let result = client
                .post(API_URL)
                .header("xi-api-key", api_key)
                .multipart(form)
                .send()
                .await;

            match result {
                Ok(response) => {
                    let status = response.status();

                    if status.is_success() {
                        let text = response
                            .text()
                            .await
                            .context("Failed to get response text")?;
                        let resp: Response = serde_json::from_str(&text)
                            .context("Failed to parse ElevenLabs API response")?;
                        return Ok(TranscriptionResult { text: resp.text });
                    }

                    // Check if error is retryable
                    if is_retryable_status(status) && attempt < config.max_retries {
                        let delay = config.delay_for_attempt(attempt, is_rate_limited(status));
                        crate::verbose!(
                            "ElevenLabs request failed with {} (attempt {}/{}), retrying in {:?}",
                            status,
                            attempt + 1,
                            config.max_retries,
                            delay
                        );
                        tokio::time::sleep(delay).await;
                        attempt += 1;
                        continue;
                    }

                    // Non-retryable error or max retries exceeded
                    let error_text = response
                        .text()
                        .await
                        .unwrap_or_else(|_| "Unknown error".to_string());
                    anyhow::bail!("ElevenLabs API error ({status}): {error_text}");
                }
                Err(err) => {
                    // Check if network error is retryable
                    if is_retryable_error(&err) && attempt < config.max_retries {
                        let delay = config.delay_for_attempt(attempt, false);
                        crate::verbose!(
                            "ElevenLabs request failed with network error (attempt {}/{}), retrying in {:?}: {}",
                            attempt + 1,
                            config.max_retries,
                            delay,
                            err
                        );
                        tokio::time::sleep(delay).await;
                        attempt += 1;
                        continue;
                    }

                    return Err(err).context("Failed to send request to ElevenLabs API");
                }
            }
        }
    }
}
