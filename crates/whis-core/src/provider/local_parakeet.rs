//! Local transcription using NVIDIA Parakeet via ONNX
//!
//! This provider enables offline transcription using Parakeet models.
//! Requires a Parakeet model directory containing ONNX files.
//!
//! Parakeet models offer high accuracy and speed for speech-to-text.

use anyhow::{Context, Result};
use async_trait::async_trait;

use super::{TranscriptionBackend, TranscriptionRequest, TranscriptionResult, TranscriptionStage};

/// Local Parakeet transcription provider
#[derive(Debug, Default, Clone)]
pub struct LocalParakeetProvider;

#[async_trait]
impl TranscriptionBackend for LocalParakeetProvider {
    fn name(&self) -> &'static str {
        "local-parakeet"
    }

    fn display_name(&self) -> &'static str {
        "Local Parakeet"
    }

    fn transcribe_sync(
        &self,
        model_path: &str, // Path to Parakeet model directory
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

/// Perform local transcription using parakeet-rs
fn transcribe_local(
    model_path: &str,
    request: TranscriptionRequest,
) -> Result<TranscriptionResult> {
    // Report transcribing stage
    request.report(TranscriptionStage::Transcribing);

    // Decode MP3 to PCM samples
    let pcm_samples = decode_mp3_to_samples(&request.audio_data)?;

    // Transcribe the samples
    transcribe_samples(model_path, pcm_samples)
}

/// Transcribe raw f32 samples directly.
///
/// Use this for local recordings where samples are already 16kHz mono.
///
/// # Arguments
/// * `model_path` - Path to the Parakeet model directory
/// * `samples` - Raw f32 audio samples (must be 16kHz mono)
pub fn transcribe_raw(model_path: &str, samples: Vec<f32>) -> Result<TranscriptionResult> {
    transcribe_samples(model_path, samples)
}

/// Internal function to transcribe PCM samples using Parakeet
fn transcribe_samples(model_path: &str, samples: Vec<f32>) -> Result<TranscriptionResult> {
    use parakeet_rs::{ParakeetTDT, Transcriber};

    // Load the Parakeet TDT model (multilingual, uses vocab.txt)
    let mut parakeet =
        ParakeetTDT::from_pretrained(model_path, None).context("Failed to load Parakeet model")?;

    // Transcribe the audio samples
    // samples should be 16kHz mono
    let result = parakeet
        .transcribe_samples(samples, 16000, 1, None)
        .context("Parakeet transcription failed")?;

    Ok(TranscriptionResult {
        text: result.text.trim().to_string(),
    })
}

/// Decode MP3 audio data to f32 samples at 16kHz mono
fn decode_mp3_to_samples(mp3_data: &[u8]) -> Result<Vec<f32>> {
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

    // Resample to 16kHz mono if needed
    crate::resample::resample_to_16k(&samples, sample_rate, channels)
}
