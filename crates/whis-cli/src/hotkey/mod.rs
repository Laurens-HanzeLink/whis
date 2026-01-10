//! Cross-platform hotkey support
//!
//! - Linux/macOS: Uses rdev for keyboard grab (supports X11, Wayland, and macOS)
//! - Windows: Uses global-hotkey crate (Tauri-maintained)

use anyhow::Result;
use std::sync::mpsc::Receiver;

#[cfg(any(target_os = "linux", target_os = "macos"))]
mod unix_like;
#[cfg(any(target_os = "linux", target_os = "macos"))]
use unix_like as platform;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use windows as platform;

/// Opaque guard that keeps the hotkey listener alive
#[allow(dead_code)]
pub struct HotkeyGuard(platform::HotkeyGuard);

/// Setup the hotkey listener.
/// Returns a receiver for hotkey events and a guard that must be kept alive.
pub fn setup(hotkey_str: &str) -> Result<(Receiver<()>, HotkeyGuard)> {
    let (rx, guard) = platform::setup(hotkey_str)?;
    Ok((rx, HotkeyGuard(guard)))
}

/// Validate a hotkey string and return normalized form if valid
///
/// Examples of valid hotkeys: "ctrl+alt+w", "super+shift+r", "cmd+option+w"
pub fn validate(hotkey_str: &str) -> Result<String> {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        use whis_core::hotkey::Hotkey;
        let hotkey = Hotkey::parse(hotkey_str).map_err(|e| anyhow::anyhow!(e))?;
        Ok(hotkey.to_normalized_string())
    }
    #[cfg(target_os = "windows")]
    {
        // Windows validation via global-hotkey
        windows::validate(hotkey_str)
    }
}
