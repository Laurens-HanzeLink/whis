<div align="center">
<img src="https://raw.githubusercontent.com/frankdierolf/whis/main/crates/whis-desktop/icons/128x128.png" alt="whis" width="80" height="80" />

<h3>whis</h3>
<p>
  Your voice, piped to clipboard.
  <br />
  <a href="https://whis.ink">Website</a>
  ·
  <a href="https://github.com/frankdierolf/whis/tree/main/crates/whis-desktop">Desktop</a>
  ·
  <a href="https://github.com/frankdierolf/whis/tree/main/crates/whis-mobile">Mobile</a>
  ·
  <a href="https://github.com/frankdierolf/whis/releases">Releases</a>
</p>
</div>

## Introduction

The terminal-native voice-to-text tool. Record, transcribe, paste — all from your shell. Supports hotkey mode, presets, and pipes nicely with AI assistants.

## Quick Start

```bash
cargo install whis
whis setup cloud   # or: whis setup local
whis
```

## Usage

```bash
# Record once
whis                           # Press Enter to stop — text copied!

# Hotkey mode (background)
whis listen                    # Ctrl+Alt+W toggles recording
whis listen -k "super+space"   # Custom hotkey

# From file
whis -f audio.mp3

# Presets
whis --as email                # Use preset
whis presets                   # List all

# Post-process with LLM
whis --post-process            # Clean up with Ollama
```

## Requirements

- API key from [OpenAI](https://platform.openai.com/api-keys) (or Groq, Deepgram, ...) — or use local Whisper (no API key needed)
- FFmpeg (`sudo apt install ffmpeg` or `brew install ffmpeg`)
- Linux (X11/Wayland) or macOS

**For hotkey mode** (one-time setup on Linux):
```bash
sudo usermod -aG input $USER
echo 'KERNEL=="uinput", GROUP="input", MODE="0660"' | sudo tee /etc/udev/rules.d/99-uinput.rules
sudo udevadm control --reload-rules && sudo udevadm trigger
# Logout and login again
```

## Prefer a GUI?

See [whis-desktop](https://github.com/frankdierolf/whis/tree/main/crates/whis-desktop) — same functionality, with system tray.

## License

MIT
