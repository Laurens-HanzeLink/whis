//! Stdin input mode - read audio from standard input

use anyhow::Result;
use whis_core::load_audio_stdin;

use super::super::types::RecordResult;

/// Stdin recording mode
pub struct StdinMode {
    format: String,
}

impl StdinMode {
    /// Create a new stdin mode
    pub fn new(format: impl Into<String>) -> Self {
        Self {
            format: format.into(),
        }
    }

    /// Load audio from stdin
    pub fn execute(&self, quiet: bool) -> Result<RecordResult> {
        if !quiet {
            println!("Reading audio from stdin ({} format)...", self.format);
        }

        let audio = load_audio_stdin(&self.format)?;

        Ok(RecordResult {
            audio,
            raw_samples: None, // Stdin mode doesn't expose raw samples
        })
    }
}
