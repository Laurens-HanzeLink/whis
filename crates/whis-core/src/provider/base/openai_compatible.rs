//! Shared implementation for OpenAI-compatible transcription APIs.
//!
//! This module provides helper functions for transcription providers that use
//! the OpenAI Whisper API format:
//! - OpenAI Whisper API
//! - Groq Whisper API
//! - Mistral Voxtral API
//!
//! All three providers use identical request/response formats:
//! - Multipart form upload with `model` and `file` fields
//! - Authorization via `Bearer` token
//! - JSON response with `text` field

use anyhow::{Context, Result};
use serde::Deserialize;

use super::super::{
    DEFAULT_TIMEOUT_SECS, TranscriptionRequest, TranscriptionResult, TranscriptionStage,
};
use super::retry::{RetryConfig, is_rate_limited, is_retryable_error, is_retryable_status};

/// Response structure for OpenAI-compatible APIs
#[derive(Deserialize)]
struct OpenAICompatibleResponse {
    text: String,
}

/// Transcribe audio using an OpenAI-compatible API (synchronous).
///
/// # Parameters
/// - `api_url`: The API endpoint URL (e.g., "https://api.openai.com/v1/audio/transcriptions")
/// - `model`: The model name to use (e.g., "whisper-1")
/// - `api_key`: Bearer token for authentication
/// - `request`: Transcription request with audio data and options
///
/// # Returns
/// Transcription result containing the text transcript
pub(crate) fn openai_compatible_transcribe_sync(
    api_url: &str,
    model: &str,
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
            .text("model", model.to_string())
            .part(
                "file",
                reqwest::blocking::multipart::Part::bytes(request.audio_data.clone())
                    .file_name(request.filename.clone())
                    .mime_str(&request.mime_type)?,
            );

        if let Some(lang) = request.language.clone() {
            form = form.text("language", lang);
        }

        // Report transcribing stage (request sent, waiting for response)
        request.report(TranscriptionStage::Transcribing);

        let result = client
            .post(api_url)
            .header("Authorization", format!("Bearer {api_key}"))
            .multipart(form)
            .send();

        match result {
            Ok(response) => {
                let status = response.status();

                if status.is_success() {
                    let text = response.text().context("Failed to get response text")?;
                    let resp: OpenAICompatibleResponse =
                        serde_json::from_str(&text).context("Failed to parse API response")?;
                    return Ok(TranscriptionResult { text: resp.text });
                }

                // Check if error is retryable
                if is_retryable_status(status) && attempt < config.max_retries {
                    let delay = config.delay_for_attempt(attempt, is_rate_limited(status));
                    crate::verbose!(
                        "Request failed with {} (attempt {}/{}), retrying in {:?}",
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
                anyhow::bail!("API error ({status}): {error_text}");
            }
            Err(err) => {
                // Check if network error is retryable
                if is_retryable_error(&err) && attempt < config.max_retries {
                    let delay = config.delay_for_attempt(attempt, false);
                    crate::verbose!(
                        "Request failed with network error (attempt {}/{}), retrying in {:?}: {}",
                        attempt + 1,
                        config.max_retries,
                        delay,
                        err
                    );
                    std::thread::sleep(delay);
                    attempt += 1;
                    continue;
                }

                return Err(err).context("Failed to send request");
            }
        }
    }
}

/// Transcribe audio using an OpenAI-compatible API (asynchronous).
///
/// # Parameters
/// - `client`: Shared reqwest client for connection pooling
/// - `api_url`: The API endpoint URL
/// - `model`: The model name to use
/// - `api_key`: Bearer token for authentication
/// - `request`: Transcription request with audio data and options
///
/// # Returns
/// Transcription result containing the text transcript
pub(crate) async fn openai_compatible_transcribe_async(
    client: &reqwest::Client,
    api_url: &str,
    model: &str,
    api_key: &str,
    request: TranscriptionRequest,
) -> Result<TranscriptionResult> {
    // Report uploading stage
    request.report(TranscriptionStage::Uploading);

    let config = RetryConfig::default();
    let mut attempt = 0;

    loop {
        let mut form = reqwest::multipart::Form::new()
            .text("model", model.to_string())
            .part(
                "file",
                reqwest::multipart::Part::bytes(request.audio_data.clone())
                    .file_name(request.filename.clone())
                    .mime_str(&request.mime_type)?,
            );

        if let Some(lang) = request.language.clone() {
            form = form.text("language", lang);
        }

        // Report transcribing stage
        request.report(TranscriptionStage::Transcribing);

        let result = client
            .post(api_url)
            .header("Authorization", format!("Bearer {api_key}"))
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
                    let resp: OpenAICompatibleResponse =
                        serde_json::from_str(&text).context("Failed to parse API response")?;
                    return Ok(TranscriptionResult { text: resp.text });
                }

                // Check if error is retryable
                if is_retryable_status(status) && attempt < config.max_retries {
                    let delay = config.delay_for_attempt(attempt, is_rate_limited(status));
                    crate::verbose!(
                        "Request failed with {} (attempt {}/{}), retrying in {:?}",
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
                anyhow::bail!("API error ({status}): {error_text}");
            }
            Err(err) => {
                // Check if network error is retryable
                if is_retryable_error(&err) && attempt < config.max_retries {
                    let delay = config.delay_for_attempt(attempt, false);
                    crate::verbose!(
                        "Request failed with network error (attempt {}/{}), retrying in {:?}: {}",
                        attempt + 1,
                        config.max_retries,
                        delay,
                        err
                    );
                    tokio::time::sleep(delay).await;
                    attempt += 1;
                    continue;
                }

                return Err(err).context("Failed to send request");
            }
        }
    }
}
