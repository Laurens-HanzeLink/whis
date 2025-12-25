//! Local transcription using whisper.cpp via whisper-rs
//!
//! This provider enables offline transcription without API calls.
//! Requires a whisper.cpp model file (e.g., ggml-small.bin).
//!
//! Uses the model_manager for caching the WhisperContext to avoid
//! reloading the model on every transcription.

use anyhow::{Context, Result};
use async_trait::async_trait;

use super::{TranscriptionBackend, TranscriptionRequest, TranscriptionResult};
use crate::model_manager;

/// Local whisper.cpp transcription provider
#[derive(Debug, Default, Clone)]
pub struct LocalWhisperProvider;

#[async_trait]
impl TranscriptionBackend for LocalWhisperProvider {
    fn name(&self) -> &'static str {
        "local-whisper"
    }

    fn display_name(&self) -> &'static str {
        "Local Whisper"
    }

    fn transcribe_sync(
        &self,
        model_path: &str, // Repurposed: path to .bin model file
        request: TranscriptionRequest,
    ) -> Result<TranscriptionResult> {
        transcribe_local(model_path, request)
    }

    async fn transcribe_async(
        &self,
        _client: &reqwest::Client, // Not used for local transcription
        model_path: &str,
        request: TranscriptionRequest,
    ) -> Result<TranscriptionResult> {
        // Run CPU-bound transcription in blocking task
        let model_path = model_path.to_string();
        tokio::task::spawn_blocking(move || transcribe_local(&model_path, request))
            .await
            .context("Task join failed")?
    }
}

/// Perform local transcription using whisper-rs
fn transcribe_local(
    model_path: &str,
    request: TranscriptionRequest,
) -> Result<TranscriptionResult> {
    use super::TranscriptionStage;

    // Report transcribing stage (local transcription)
    request.report(TranscriptionStage::Transcribing);

    // Decode MP3 to PCM and resample to 16kHz mono
    let pcm_samples = decode_and_resample(&request.audio_data)?;

    // Transcribe the samples
    transcribe_samples(model_path, &pcm_samples, request.language.as_deref())
}

/// Transcribe raw f32 samples directly (skips MP3 decoding).
///
/// Use this for local recordings where samples are already 16kHz mono.
/// This is faster than going through MP3 encoding/decoding.
///
/// # Arguments
/// * `model_path` - Path to the whisper.cpp model file (.bin)
/// * `samples` - Raw f32 audio samples (must be 16kHz mono)
/// * `language` - Optional language code (e.g., "en", "de")
pub fn transcribe_raw(
    model_path: &str,
    samples: &[f32],
    language: Option<&str>,
) -> Result<TranscriptionResult> {
    transcribe_samples(model_path, samples, language)
}

/// Internal function to transcribe PCM samples
fn transcribe_samples(
    model_path: &str,
    samples: &[f32],
    language: Option<&str>,
) -> Result<TranscriptionResult> {
    use whisper_rs::{FullParams, SamplingStrategy};

    // Get or load the cached model and create a state
    let mut model_guard = model_manager::get_model(model_path)?;
    let state = model_guard.state_mut();

    // Configure parameters
    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });

    // Set language if provided
    if let Some(lang) = language {
        params.set_language(Some(lang));
    }

    // Disable printing to stdout
    params.set_print_special(false);
    params.set_print_progress(false);
    params.set_print_realtime(false);
    params.set_print_timestamps(false);

    // Run transcription
    state
        .full(params, samples)
        .context("Transcription failed")?;

    // Extract text from segments
    let num_segments = state.full_n_segments();

    let mut text = String::new();
    for i in 0..num_segments {
        if let Some(segment) = state.get_segment(i)
            && let Ok(segment_text) = segment.to_str()
        {
            text.push_str(segment_text);
        }
    }

    // Drop guard before potentially unloading model
    drop(model_guard);

    // Conditionally unload model based on keep_loaded setting
    model_manager::maybe_unload();

    Ok(TranscriptionResult {
        text: text.trim().to_string(),
    })
}

/// Decode MP3 audio data and resample to 16kHz mono for whisper
fn decode_and_resample(mp3_data: &[u8]) -> Result<Vec<f32>> {
    use minimp3::{Decoder, Frame};

    let mut decoder = Decoder::new(mp3_data);
    let mut samples = Vec::new();
    let mut sample_rate = 0u32;
    let mut channels = 0u16;

    // Decode all MP3 frames
    loop {
        match decoder.next_frame() {
            Ok(Frame {
                data,
                sample_rate: sr,
                channels: ch,
                ..
            }) => {
                sample_rate = sr as u32;
                channels = ch as u16;
                // Convert i16 samples to f32 normalized to [-1.0, 1.0]
                samples.extend(data.iter().map(|&s| s as f32 / i16::MAX as f32));
            }
            Err(minimp3::Error::Eof) => break,
            Err(e) => anyhow::bail!("MP3 decode error: {:?}", e),
        }
    }

    if samples.is_empty() {
        anyhow::bail!("No audio data decoded from MP3");
    }

    // Resample to 16kHz mono
    crate::resample::resample_to_16k(&samples, sample_rate, channels)
}
