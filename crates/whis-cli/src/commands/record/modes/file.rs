//! File input mode - load and transcribe audio from file

use anyhow::Result;
use std::path::PathBuf;
use whis_core::load_audio_file;

use super::super::types::RecordResult;

/// File recording mode
pub struct FileMode {
    path: PathBuf,
}

impl FileMode {
    /// Create a new file mode
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    /// Load audio from file
    pub fn execute(&self, quiet: bool) -> Result<RecordResult> {
        if !quiet {
            println!("Loading audio file: {}", self.path.display());
        }

        let audio = load_audio_file(&self.path)?;

        Ok(RecordResult {
            audio,
            raw_samples: None, // File mode doesn't expose raw samples
        })
    }
}
