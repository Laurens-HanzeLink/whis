<div align="center">
<img src="https://raw.githubusercontent.com/frankdierolf/whis/main/crates/whis-desktop/icons/128x128.png" alt="whis" width="80" height="80" />

<h3>whis-desktop</h3>
<p>
  Your voice, piped to clipboard. With a GUI.
  <br />
  <a href="https://whis.ink">Website</a>
  ·
  <a href="https://github.com/frankdierolf/whis/tree/main/crates/whis-cli">CLI</a>
  ·
  <a href="https://github.com/frankdierolf/whis/tree/main/crates/whis-mobile">Mobile</a>
  ·
  <a href="https://github.com/frankdierolf/whis/releases">Releases</a>
</p>
</div>

## Introduction

A desktop app that lives in your system tray. Hit a global shortcut, speak, and your words land in the clipboard — ready to paste into Claude, Copilot, or anywhere else.

## Quick Start

```bash
flatpak install flathub ink.whis.Whis
```

## Screenshot

![Presets](https://raw.githubusercontent.com/frankdierolf/whis/main/crates/whis-desktop/screenshots/5-presets.png)

## Features

- **System tray** — lives in your taskbar, out of the way
- **Global shortcut** — Ctrl+Alt+W by default (configurable)
- **Settings UI** — configure provider, API key, and shortcuts
- **X11 & Wayland** — works on both

## Installation

**AppImage**:
```bash
wget https://github.com/frankdierolf/whis/releases/latest/download/Whis_amd64.AppImage
chmod +x Whis_amd64.AppImage
./Whis_amd64.AppImage --install
```

**Debian/Ubuntu**:
```bash
wget https://github.com/frankdierolf/whis/releases/latest/download/Whis_amd64.deb
sudo dpkg -i Whis_amd64.deb
```

**Flatpak**:
```bash
flatpak install flathub ink.whis.Whis
```

## Requirements

- API key from [OpenAI](https://platform.openai.com/api-keys) (or Groq, Deepgram, ...) — or use local Whisper (no API key needed)
- Linux (X11/Wayland)

## Prefer the terminal?

See [whis CLI](https://github.com/frankdierolf/whis) — same functionality, no GUI.

## License

MIT
