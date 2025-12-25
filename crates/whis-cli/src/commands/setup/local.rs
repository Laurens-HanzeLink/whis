//! Local transcription setup

use anyhow::{Result, anyhow};
use std::io::{self, Write};
use whis_core::{PostProcessor, Settings, TranscriptionProvider, model, ollama};

use super::post_processing::select_ollama_model;
use crate::ui::{prompt_choice, prompt_choice_with_default};

/// Setup for fully local (on-device) transcription (full interactive wizard)
pub fn setup_local() -> Result<()> {
    println!("Local Setup");
    println!("===========");
    println!();
    println!("This will set up fully local transcription:");
    println!("  - Transcription model (runs on CPU)");
    println!("  - Ollama for transcript post-processing (runs locally)");
    println!();

    // Step 1: Choose transcription engine
    println!("Step 1: Choose Transcription Engine");
    println!("------------------------------------");
    println!("  1. Parakeet - RECOMMENDED (NVIDIA model, fast & accurate)");
    println!("  2. Whisper - OpenAI model (multiple sizes available)");
    println!();

    let engine_choice = prompt_choice("Select engine (1-2)", 1, 2)?;
    println!();

    let (provider, model_path) = match engine_choice {
        1 => {
            // Parakeet setup - show available models
            println!("Step 2: Choose Parakeet Model");
            println!("-----------------------------");
            for (i, (name, _, desc, _)) in model::PARAKEET_MODELS.iter().enumerate() {
                let path = model::default_parakeet_model_path(name);
                let status = if model::parakeet_model_exists(&path) {
                    " [installed]"
                } else {
                    ""
                };
                println!("  {}. {} - {}{}", i + 1, name, desc, status);
            }
            println!();

            let model_choice = prompt_choice(
                &format!("Select model (1-{})", model::PARAKEET_MODELS.len()),
                1,
                model::PARAKEET_MODELS.len(),
            )?;
            let (model_name, _, _, _) = model::PARAKEET_MODELS[model_choice - 1];
            println!();

            println!("Setting up {}...", model_name);
            let path = model::default_parakeet_model_path(model_name);
            if model::parakeet_model_exists(&path) {
                println!("Model '{}' already installed at:", model_name);
                println!("  {}", path.display());
            } else {
                println!("Downloading '{}' model...", model_name);
                model::download_parakeet_model(model_name, &path)?;
            }
            (TranscriptionProvider::LocalParakeet, path)
        }
        2 => {
            // Whisper setup - show available models
            println!("Step 2: Choose Whisper Model");
            println!("----------------------------");
            for (i, (name, _, desc)) in model::WHISPER_MODELS.iter().enumerate() {
                let path = model::default_model_path(name);
                let status = if model::model_exists(&path) {
                    " [installed]"
                } else {
                    ""
                };
                println!("  {}. {} - {}{}", i + 1, name, desc, status);
            }
            println!();

            let model_choice = prompt_choice(
                &format!("Select model (1-{})", model::WHISPER_MODELS.len()),
                1,
                model::WHISPER_MODELS.len(),
            )?;
            let (model_name, _, _) = model::WHISPER_MODELS[model_choice - 1];
            println!();

            println!("Setting up {}...", model_name);
            let path = model::default_model_path(model_name);
            if model::model_exists(&path) {
                println!("Model '{}' already installed at:", model_name);
                println!("  {}", path.display());
            } else {
                println!("Downloading '{}' model...", model_name);
                model::download_model(model_name, &path)?;
            }
            (TranscriptionProvider::LocalWhisper, path)
        }
        _ => unreachable!(),
    };
    println!();

    // Step 3: Setup Ollama
    println!("Step 3: Ollama (for post-processing)");
    println!("------------------------------------");

    let ollama_url = ollama::DEFAULT_OLLAMA_URL;

    // Check if Ollama is installed
    if !ollama::is_ollama_installed() {
        println!("Ollama is not installed.");
        println!();
        println!("Install Ollama:");
        println!("  Linux:  curl -fsSL https://ollama.com/install.sh | sh");
        println!("  macOS:  brew install ollama");
        println!("  Website: https://ollama.com/download");
        println!();
        return Err(anyhow!("Please install Ollama and run setup again"));
    }

    // Start Ollama if not running
    ollama::ensure_ollama_running(ollama_url)?;

    // Let user select model (shows installed + recommended options)
    let ollama_model = select_ollama_model(ollama_url, None)?;
    println!();

    // Step 4: Save configuration
    println!("Step 4: Saving Configuration");
    println!("----------------------------");

    let mut settings = Settings::load();
    settings.provider = provider.clone();
    match &provider {
        TranscriptionProvider::LocalParakeet => {
            settings.parakeet_model_path = Some(model_path.to_string_lossy().to_string());
        }
        TranscriptionProvider::LocalWhisper => {
            settings.whisper_model_path = Some(model_path.to_string_lossy().to_string());
        }
        _ => {}
    }
    settings.post_processor = PostProcessor::Ollama;
    settings.ollama_url = Some(ollama_url.to_string());
    settings.ollama_model = Some(ollama_model.clone());
    settings.save()?;

    println!("Configuration saved to: {}", Settings::path().display());
    println!();
    println!("Setup complete!");
    println!();
    println!("Your setup:");
    println!("  Transcription:    {}", provider.display_name());
    println!("  Post-processing:  Ollama ({})", ollama_model);
    println!();
    println!("Try it out:");
    println!("  whis                # Record and transcribe locally");
    println!("  whis --post-process # Record, transcribe, and post-process locally");
    println!();
    println!("Note: Ollama will auto-start when needed.");

    Ok(())
}

/// Streamlined local transcription setup (no post-processing config)
/// Used by the unified wizard
pub fn setup_transcription_local() -> Result<()> {
    let mut settings = Settings::load();

    // Determine current engine and show with [current] marker
    let current_engine = match settings.provider {
        TranscriptionProvider::LocalParakeet => Some(1),
        TranscriptionProvider::LocalWhisper => Some(2),
        _ => None,
    };

    println!("Model:");
    let parakeet_marker = if current_engine == Some(1) { " [current]" } else { "" };
    let whisper_marker = if current_engine == Some(2) { " [current]" } else { "" };
    println!("  1. Parakeet (recommended) - Fast, accurate{}", parakeet_marker);
    println!("  2. Whisper - OpenAI model, multiple sizes{}", whisper_marker);
    println!();

    // Default to current engine if local, otherwise Parakeet
    let default = current_engine.unwrap_or(1);
    let engine_choice = prompt_choice_with_default("Select", 1, 2, Some(default))?;
    println!();

    let (provider, model_path) = match engine_choice {
        1 => {
            // Parakeet - use recommended model directly
            let (model_name, _, _, _) = model::PARAKEET_MODELS[0]; // First is recommended
            let path = model::default_parakeet_model_path(model_name);

            if model::parakeet_model_exists(&path) {
                println!("Model ready: {}", model_name);
            } else {
                print!("Downloading {}... ", model_name);
                io::stdout().flush()?;
                model::download_parakeet_model(model_name, &path)?;
                println!("Done!");
            }

            (TranscriptionProvider::LocalParakeet, path)
        }
        2 => {
            // Whisper - show model options
            println!("Whisper model:");
            for (i, (name, _, desc)) in model::WHISPER_MODELS.iter().enumerate() {
                let path = model::default_model_path(name);
                let status = if model::model_exists(&path) { " [installed]" } else { "" };
                println!("  {}. {} - {}{}", i + 1, name, desc, status);
            }
            println!();

            let model_choice = prompt_choice_with_default(
                "Select",
                1,
                model::WHISPER_MODELS.len(),
                Some(2), // Default to "base" which is usually index 2
            )?;
            let (model_name, _, _) = model::WHISPER_MODELS[model_choice - 1];
            println!();

            let path = model::default_model_path(model_name);
            if model::model_exists(&path) {
                println!("Model ready: {}", model_name);
            } else {
                print!("Downloading {}... ", model_name);
                io::stdout().flush()?;
                model::download_model(model_name, &path)?;
                println!("Done!");
            }

            (TranscriptionProvider::LocalWhisper, path)
        }
        _ => unreachable!(),
    };

    // Save transcription config
    settings.provider = provider.clone();
    match &provider {
        TranscriptionProvider::LocalParakeet => {
            settings.parakeet_model_path = Some(model_path.to_string_lossy().to_string());
        }
        TranscriptionProvider::LocalWhisper => {
            settings.whisper_model_path = Some(model_path.to_string_lossy().to_string());
        }
        _ => {}
    }
    settings.save()?;

    Ok(())
}
