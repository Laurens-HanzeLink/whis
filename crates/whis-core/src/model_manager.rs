//! Model caching for local transcription
//!
//! This module provides a global cache for the WhisperContext to avoid
//! reloading the model on every transcription (saves 200ms-2s per call).
//!
//! By default, models are unloaded immediately after transcription.
//! In listen mode, `set_keep_loaded(true)` keeps the model in memory
//! for faster subsequent transcriptions.

use anyhow::{Context, Result};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{OnceLock, RwLock};
use whisper_rs::{WhisperContext, WhisperContextParameters, WhisperState};

/// Global model cache
static MODEL_CACHE: OnceLock<RwLock<Option<CachedModel>>> = OnceLock::new();

/// Whether to keep the model loaded after transcription
static KEEP_LOADED: AtomicBool = AtomicBool::new(false);

/// Cached whisper model with its path for validation
struct CachedModel {
    context: WhisperContext,
    path: String,
}

fn get_cache() -> &'static RwLock<Option<CachedModel>> {
    MODEL_CACHE.get_or_init(|| RwLock::new(None))
}

/// Get or load the whisper model and create a state for transcription.
///
/// If the model is already cached (and path matches), uses the cached context.
/// Otherwise, loads the model from disk and caches it.
///
/// # Arguments
/// * `path` - Path to the whisper model file (.bin)
///
/// # Returns
/// A WhisperState ready for transcription
pub fn get_model(path: &str) -> Result<ModelGuard> {
    // First, try to use the cached model
    {
        let cache = get_cache().read().unwrap();
        if let Some(ref cached) = *cache
            && cached.path == path
        {
            // Model is already loaded and path matches
            let state = cached
                .context
                .create_state()
                .context("Failed to create whisper state")?;
            return Ok(ModelGuard { state });
        }
    }

    // Need to load or reload the model
    let state = {
        let mut cache = get_cache().write().unwrap();

        // Double-check after acquiring write lock (another thread may have loaded)
        if let Some(ref cached) = *cache
            && cached.path == path
        {
            let state = cached
                .context
                .create_state()
                .context("Failed to create whisper state")?;
            return Ok(ModelGuard { state });
        }

        // Validate model path
        if path.is_empty() {
            anyhow::bail!(
                "Whisper model path not configured. Set LOCAL_WHISPER_MODEL_PATH or use: whis config --whisper-model-path <path>"
            );
        }

        if !std::path::Path::new(path).exists() {
            anyhow::bail!(
                "Whisper model not found at: {}\n\
                 Download a model from: https://huggingface.co/ggerganov/whisper.cpp/tree/main",
                path
            );
        }

        // Suppress verbose whisper.cpp logging
        whisper_rs::install_logging_hooks();

        crate::verbose!("Loading whisper model from: {}", path);

        // Load the model
        let context = WhisperContext::new_with_params(path, WhisperContextParameters::default())
            .context("Failed to load whisper model")?;

        crate::verbose!("Whisper model loaded successfully");

        // Create state before caching (state holds Arc to context internally)
        let state = context
            .create_state()
            .context("Failed to create whisper state")?;

        // Cache the context
        *cache = Some(CachedModel {
            context,
            path: path.to_string(),
        });

        state
    };

    Ok(ModelGuard { state })
}

/// Unload the cached model (if any).
///
/// This frees the memory used by the model. Call this when you're done
/// with transcription and don't expect more requests soon.
pub fn unload_model() {
    let mut cache = get_cache().write().unwrap();
    if cache.is_some() {
        crate::verbose!("Unloading whisper model from cache");
        *cache = None;
    }
}

/// Set whether to keep the model loaded after transcription.
///
/// When `true`, the model stays in memory for faster subsequent transcriptions.
/// When `false` (default), the model is unloaded after each use.
///
/// # Arguments
/// * `keep` - Whether to keep the model loaded
pub fn set_keep_loaded(keep: bool) {
    KEEP_LOADED.store(keep, Ordering::SeqCst);
    crate::verbose!("Model cache keep_loaded set to: {}", keep);
}

/// Check if models should be kept loaded.
pub fn should_keep_loaded() -> bool {
    KEEP_LOADED.load(Ordering::SeqCst)
}

/// Called after transcription to conditionally unload the model.
pub fn maybe_unload() {
    if !should_keep_loaded() {
        unload_model();
    }
}

/// Guard that holds a WhisperState for transcription.
///
/// The state internally holds an Arc to the context, so it's safe to use
/// even after the cache lock is released.
pub struct ModelGuard {
    state: WhisperState,
}

impl ModelGuard {
    /// Get a mutable reference to the whisper state for transcription.
    pub fn state_mut(&mut self) -> &mut WhisperState {
        &mut self.state
    }

    /// Consume the guard and return the state.
    pub fn into_state(self) -> WhisperState {
        self.state
    }
}
