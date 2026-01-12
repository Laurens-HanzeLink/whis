//! External service configuration (Ollama, etc.).

use serde::{Deserialize, Serialize};

/// Settings for external services.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServicesSettings {
    /// Ollama configuration for local LLM post-processing
    #[serde(default)]
    pub ollama: OllamaConfig,
}

/// Configuration for Ollama local LLM service.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaConfig {
    /// Ollama server URL (default: http://localhost:11434)
    #[serde(default)]
    pub url: Option<String>,

    /// Ollama model name for post-processing (default: qwen2.5:1.5b)
    #[serde(default)]
    pub model: Option<String>,

    /// How long Ollama keeps the model loaded in VRAM after a request.
    ///
    /// - "0": Unload immediately after each request
    /// - "5m", "10m", "30m": Unload after idle timeout
    /// - "-1": Keep loaded forever (until Ollama restarts)
    ///
    /// Default: "5m" (Ollama's native default)
    #[serde(default)]
    pub keep_alive: Option<String>,
}

impl Default for OllamaConfig {
    fn default() -> Self {
        Self {
            url: Some(crate::configuration::DEFAULT_OLLAMA_URL.to_string()),
            model: Some(crate::configuration::DEFAULT_OLLAMA_MODEL.to_string()),
            keep_alive: Some(crate::configuration::DEFAULT_OLLAMA_KEEP_ALIVE.to_string()),
        }
    }
}

impl OllamaConfig {
    /// Get the Ollama server URL, falling back to environment variable.
    pub fn url(&self) -> Option<String> {
        self.url
            .clone()
            .or_else(|| std::env::var("OLLAMA_URL").ok())
    }

    /// Get the Ollama model name, falling back to environment variable.
    pub fn model(&self) -> Option<String> {
        self.model
            .clone()
            .or_else(|| std::env::var("OLLAMA_MODEL").ok())
    }

    /// Get the Ollama keep_alive duration, falling back to default.
    pub fn keep_alive(&self) -> String {
        self.keep_alive
            .clone()
            .unwrap_or_else(|| crate::configuration::DEFAULT_OLLAMA_KEEP_ALIVE.to_string())
    }

    /// Preload Ollama model using this config's settings.
    ///
    /// Spawns a background thread that warms up the model by sending
    /// a minimal request to Ollama. This reduces latency for the first
    /// actual post-processing request.
    ///
    /// No-op if url or model are not configured.
    pub fn preload(&self) {
        if let (Some(url), Some(model)) = (self.url(), self.model()) {
            crate::preload_ollama(&url, &model, &self.keep_alive());
        }
    }
}
