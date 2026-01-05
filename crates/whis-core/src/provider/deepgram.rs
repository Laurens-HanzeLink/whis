//! Deepgram Nova transcription provider
//!
//! Deepgram uses a different API format than OpenAI-style providers:
//! - Raw audio bytes in request body (not multipart form)
//! - Options passed as query parameters
//! - Different response JSON structure

use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::Deserialize;

use super::base::retry::{RetryConfig, is_rate_limited, is_retryable_error, is_retryable_status};
use super::{
    DEFAULT_TIMEOUT_SECS, TranscriptionBackend, TranscriptionRequest, TranscriptionResult,
    TranscriptionStage,
};

const API_URL: &str = "https://api.deepgram.com/v1/listen";
const MODEL: &str = "nova-2";

#[derive(Deserialize)]
struct Response {
    results: Results,
}

#[derive(Deserialize)]
struct Results {
    channels: Vec<Channel>,
}

#[derive(Deserialize)]
struct Channel {
    alternatives: Vec<Alternative>,
}

#[derive(Deserialize)]
struct Alternative {
    transcript: String,
}

/// Deepgram Nova transcription provider
///
/// Uses Deepgram's REST API with Nova-2 model.
/// Offers fast transcription at $0.26/hour with good accuracy.
#[derive(Debug, Default, Clone)]
pub struct DeepgramProvider;

#[async_trait]
impl TranscriptionBackend for DeepgramProvider {
    fn name(&self) -> &'static str {
        "deepgram"
    }

    fn display_name(&self) -> &'static str {
        "Deepgram Nova"
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

        let mut url = reqwest::Url::parse(API_URL).context("Failed to parse Deepgram URL")?;
        url.query_pairs_mut()
            .append_pair("model", MODEL)
            .append_pair("smart_format", "true");

        if let Some(lang) = &request.language {
            url.query_pairs_mut().append_pair("language", lang);
        }

        let config = RetryConfig::default();
        let mut attempt = 0;

        loop {
            // Report transcribing stage
            request.report(TranscriptionStage::Transcribing);

            let result = client
                .post(url.clone())
                .header("Authorization", format!("Token {api_key}"))
                .header("Content-Type", &request.mime_type)
                .body(request.audio_data.clone())
                .send();

            match result {
                Ok(response) => {
                    let status = response.status();

                    if status.is_success() {
                        let text = response.text().context("Failed to get response text")?;
                        let resp: Response = serde_json::from_str(&text)
                            .context("Failed to parse Deepgram API response")?;

                        let transcript = resp
                            .results
                            .channels
                            .first()
                            .and_then(|c| c.alternatives.first())
                            .map(|a| a.transcript.clone())
                            .ok_or_else(|| {
                                anyhow::anyhow!(
                                    "Deepgram API returned unexpected response format: no transcript found"
                                )
                            })?;

                        return Ok(TranscriptionResult { text: transcript });
                    }

                    // Check if error is retryable
                    if is_retryable_status(status) && attempt < config.max_retries {
                        let delay = config.delay_for_attempt(attempt, is_rate_limited(status));
                        crate::verbose!(
                            "Deepgram request failed with {} (attempt {}/{}), retrying in {:?}",
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
                    anyhow::bail!("Deepgram API error ({status}): {error_text}");
                }
                Err(err) => {
                    // Check if network error is retryable
                    if is_retryable_error(&err) && attempt < config.max_retries {
                        let delay = config.delay_for_attempt(attempt, false);
                        crate::verbose!(
                            "Deepgram request failed with network error (attempt {}/{}), retrying in {:?}: {}",
                            attempt + 1,
                            config.max_retries,
                            delay,
                            err
                        );
                        std::thread::sleep(delay);
                        attempt += 1;
                        continue;
                    }

                    return Err(err).context("Failed to send request to Deepgram API");
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

        let mut url = reqwest::Url::parse(API_URL).context("Failed to parse Deepgram URL")?;
        url.query_pairs_mut()
            .append_pair("model", MODEL)
            .append_pair("smart_format", "true");

        if let Some(lang) = &request.language {
            url.query_pairs_mut().append_pair("language", lang);
        }

        let config = RetryConfig::default();
        let mut attempt = 0;

        loop {
            // Report transcribing stage
            request.report(TranscriptionStage::Transcribing);

            let result = client
                .post(url.clone())
                .header("Authorization", format!("Token {api_key}"))
                .header("Content-Type", &request.mime_type)
                .body(request.audio_data.clone())
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
                            .context("Failed to parse Deepgram API response")?;

                        let transcript = resp
                            .results
                            .channels
                            .first()
                            .and_then(|c| c.alternatives.first())
                            .map(|a| a.transcript.clone())
                            .ok_or_else(|| {
                                anyhow::anyhow!(
                                    "Deepgram API returned unexpected response format: no transcript found"
                                )
                            })?;

                        return Ok(TranscriptionResult { text: transcript });
                    }

                    // Check if error is retryable
                    if is_retryable_status(status) && attempt < config.max_retries {
                        let delay = config.delay_for_attempt(attempt, is_rate_limited(status));
                        crate::verbose!(
                            "Deepgram request failed with {} (attempt {}/{}), retrying in {:?}",
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
                    anyhow::bail!("Deepgram API error ({status}): {error_text}");
                }
                Err(err) => {
                    // Check if network error is retryable
                    if is_retryable_error(&err) && attempt < config.max_retries {
                        let delay = config.delay_for_attempt(attempt, false);
                        crate::verbose!(
                            "Deepgram request failed with network error (attempt {}/{}), retrying in {:?}: {}",
                            attempt + 1,
                            config.max_retries,
                            delay,
                            err
                        );
                        tokio::time::sleep(delay).await;
                        attempt += 1;
                        continue;
                    }

                    return Err(err).context("Failed to send request to Deepgram API");
                }
            }
        }
    }
}
