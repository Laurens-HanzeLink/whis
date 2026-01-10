//! Manual Shortcut Setup Instructions
//!
//! Provides compositor-specific instructions for users to manually configure
//! global shortcuts when automatic methods (Tauri plugin, Portal) are unavailable.

use whis_core::Compositor;

/// Print manual setup instructions for the user based on their compositor
pub fn print_manual_setup_instructions(compositor: &Compositor, shortcut: &str) {
    println!();
    println!("=== Global Shortcuts Not Available ===");
    println!("Compositor: {}", compositor.display_name());
    println!();
    println!("To use a keyboard shortcut, configure your compositor:");
    println!();

    match compositor {
        Compositor::Gnome => {
            println!("GNOME: Settings → Keyboard → Custom Shortcuts");
            println!("  Name: Whis Toggle Recording");
            println!("  Command: whis-desktop --toggle");
            println!("  Shortcut: {shortcut}");
        }
        Compositor::KdePlasma => {
            println!("KDE: System Settings → Shortcuts → Custom Shortcuts");
            println!("  Command: whis-desktop --toggle");
        }
        Compositor::Sway => {
            println!("Sway: Add to ~/.config/sway/config:");
            println!(
                "  bindsym {} exec whis-desktop --toggle",
                shortcut.to_lowercase()
            );
        }
        Compositor::Hyprland => {
            println!("Hyprland: Add to ~/.config/hypr/hyprland.conf:");
            println!(
                "  bind = {}, exec, whis-desktop --toggle",
                shortcut.replace('+', ", ")
            );
        }
        Compositor::Wlroots | Compositor::Unknown(_) => {
            println!("Configure your compositor to run: whis-desktop --toggle");
        }
        Compositor::Native | Compositor::X11 => {
            // These should use TauriPlugin, not manual setup
            println!("Configure your system to run: whis-desktop --toggle");
        }
    }
    println!();
}
