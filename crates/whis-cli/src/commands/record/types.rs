//! Record Command Types
//!
//! This module defines the core data structures used throughout the record command
//! pipeline. Microphone input uses progressive transcription.
//!
//! # Type Flow
//!
//! ```text
//! RecordConfig
//!     ↓
//! ┌─────────────────┐
//! │  Record Phase   │  → Vec<f32> samples (16kHz mono)
//! └─────────────────┘
//!     ↓
//! ┌─────────────────┐
//! │  Progressive    │  → TranscriptionResult { text }
//! │  Transcription  │
//! └─────────────────┘
//!     ↓
//! ┌─────────────────┐
//! │  Process Phase  │  → ProcessedResult { text }
//! └─────────────────┘
//!     ↓
//! ┌─────────────────┐
//! │  Output Phase   │  → Final output (clipboard/stdout)
//! └─────────────────┘
//! ```
//!
//! # Key Types
//!
//! - `RecordConfig`: User-provided configuration (flags, presets, output mode)
//! - `TranscriptionResult`: Raw transcript text from provider
//! - `ProcessedResult`: Final processed text after LLM cleanup/preset transform

use anyhow::Result;
use std::time::Duration;
use whis_core::Preset;

/// Configuration for the record command
#[derive(Debug, Clone)]
pub struct RecordConfig {
    /// Whether to enable post-processing
    pub post_process: bool,
    /// Preset to apply to output
    pub preset: Option<Preset>,
    /// Whether to print to stdout instead of clipboard
    pub print: bool,
    /// Recording duration (None = until silence/manual stop)
    pub duration: Option<Duration>,
    /// Disable Voice Activity Detection
    pub no_vad: bool,
}

impl RecordConfig {
    /// Create a new record configuration
    pub fn new(
        post_process: bool,
        preset_name: Option<String>,
        print: bool,
        duration: Option<Duration>,
        no_vad: bool,
    ) -> Result<Self> {
        // Load preset if provided
        let preset = if let Some(name) = preset_name {
            let (p, _source) = Preset::load(&name).map_err(|e| anyhow::anyhow!("{}", e))?;
            Some(p)
        } else {
            None
        };

        Ok(Self {
            post_process,
            preset,
            print,
            duration,
            no_vad,
        })
    }

    /// Check if output should be quiet (for clean stdout)
    pub fn is_quiet(&self) -> bool {
        self.print
    }
}

/// Result of transcription phase
#[derive(Debug)]
pub struct TranscriptionResult {
    /// The transcribed text
    pub text: String,
}

/// Result of post-processing phase
#[derive(Debug)]
pub struct ProcessedResult {
    /// The processed text
    pub text: String,
}
