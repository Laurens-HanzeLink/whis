<div align="center">
<img src="https://raw.githubusercontent.com/frankdierolf/whis/main/crates/whis-desktop/icons/128x128.png" alt="whis" width="80" height="80" />

<h3>whis-mobile</h3>
<p>
  Whis voice-to-text mobile application for Android.
  <br />
  <a href="https://whis.ink">Website</a>
  ·
  <a href="https://github.com/frankdierolf/whis/tree/main/crates/whis-cli">CLI</a>
  ·
  <a href="https://github.com/frankdierolf/whis/tree/main/crates/whis-desktop">Desktop</a>
</p>
</div>

## Features

- **Voice-to-text** — record and transcribe speech on Android
- **Multi-provider** — OpenAI, Mistral, Groq, Deepgram, or ElevenLabs
- **Lightweight** — uses embedded MP3 encoder (no FFmpeg dependency)
- **Clipboard** — copy transcriptions directly to clipboard

## Platform Support

- **Android** — primary mobile platform

## Differences from Desktop

| Feature | Desktop | Mobile |
|---------|---------|--------|
| Audio encoding | FFmpeg subprocess | Embedded mp3lame |
| Local Whisper | Supported | Not available |
| Binary size | Larger | Optimized for mobile |

## Building

Requires Android SDK and NDK. See [Tauri mobile prerequisites](https://v2.tauri.app/start/prerequisites/).

```bash
# Install Android targets
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi

# Initialize Android project (first time only)
just android-init

# Development build (emulator or connected device)
just android-dev

# Release build
just android-release
```

## Project Structure

```
whis-mobile/
├── src/           # Rust Tauri commands
├── ui/            # Vue.js frontend (shared with desktop)
├── icons/         # App icons
├── capabilities/  # Android permissions
└── gen/           # Generated Android project
```

## License

MIT
