//! Pipeline phases for the record command
//!
//! The record command follows a clear pipeline:
//! 1. Record/Load - Get audio from source (returns f32 samples)
//! 2. Progressive Transcribe - Convert audio chunks to text as they arrive
//! 3. Process - Apply post-processing and presets
//! 4. Output - Display or copy to clipboard

pub mod output;
pub mod process;

pub use output::{OutputMode, output};
pub use process::{ProcessingConfig, process};
