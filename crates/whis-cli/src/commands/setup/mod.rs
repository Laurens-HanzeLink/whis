//! Setup wizard for different usage modes
//!
//! Provides a streamlined setup experience for:
//! - Cloud users (API key setup)
//! - Local users (on-device transcription)

mod cloud;
mod interactive;
mod local;
mod post_processing;
mod provider_helpers;

use anyhow::Result;

use crate::args::SetupMode;

pub fn run(mode: Option<SetupMode>) -> Result<()> {
    match mode {
        None => setup_wizard(),
        Some(SetupMode::Cloud) => cloud::setup_cloud(),
        Some(SetupMode::Local) => local::setup_local(),
        Some(SetupMode::PostProcessing) => post_processing::setup_post_processing(),
    }
}

/// Unified setup wizard - guides user through all configuration
fn setup_wizard() -> Result<()> {
    let items = vec!["Cloud", "Local"];
    let choice = interactive::select("How do you want to transcribe?", &items, Some(0))?;

    let is_cloud = match choice {
        0 => {
            cloud::setup_transcription_cloud()?;
            true
        }
        1 => {
            local::setup_transcription_local()?;
            false
        }
        _ => unreachable!(),
    };

    post_processing::setup_post_processing_step(is_cloud)?;

    interactive::info("Configuration saved! Run 'whis' to record and transcribe.");

    Ok(())
}
