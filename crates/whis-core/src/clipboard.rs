use anyhow::{Context, Result};
use arboard::Clipboard;

/// Copy text to clipboard
///
/// Uses arboard with wayland-data-control feature enabled, which uses the
/// wlr-data-control protocol on Wayland. This works in Flatpak without
/// requiring flatpak-spawn or org.freedesktop.Flatpak permission.
pub fn copy_to_clipboard(text: &str) -> Result<()> {
    let mut clipboard = Clipboard::new().context("Failed to access clipboard")?;
    clipboard
        .set_text(text)
        .context("Failed to copy text to clipboard")?;

    Ok(())
}
