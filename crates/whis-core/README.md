<div align="center">
<img src="https://raw.githubusercontent.com/frankdierolf/whis/main/crates/whis-desktop/icons/128x128.png" alt="whis" width="80" height="80" />

<h3>whis-core</h3>
<p>
  Core library for whis voice-to-text functionality.
  <br />
  <a href="https://whis.ink">Website</a>
  ·
  <a href="https://github.com/frankdierolf/whis/tree/main/crates/whis-cli">CLI</a>
  ·
  <a href="https://github.com/frankdierolf/whis/tree/main/crates/whis-desktop">Desktop</a>
</p>
</div>

## Features

- **Audio recording** — capture microphone input via cpal
- **Multi-provider transcription** — OpenAI, Mistral, Groq, Deepgram, ElevenLabs, or local Whisper
- **Parallel processing** — split long recordings into chunks for parallel transcription
- **LLM post-processing** — clean up transcriptions using Ollama
- **Clipboard** — copy results to system clipboard (X11, Wayland, Flatpak)
- **Config management** — persistent settings in `~/.config/whis/`

## Usage

```rust
use whis_core::{
    AudioRecorder, TranscriptionProvider, RecordingOutput,
    transcribe_audio, copy_to_clipboard, ClipboardMethod,
};

// Configure provider and API key
let provider = TranscriptionProvider::OpenAI;
let api_key = std::env::var("OPENAI_API_KEY")?;

// Record audio
let mut recorder = AudioRecorder::new()?;
recorder.start_recording()?;
// ... wait for user input ...
let output = recorder.finalize_recording()?;

// Extract audio data from RecordingOutput
let audio_data = match output {
    RecordingOutput::Single(data) => data,
    RecordingOutput::Chunked(chunks) => {
        // For chunked audio, use parallel_transcribe instead
        chunks.into_iter().next().unwrap().data
    }
};

// Transcribe
let text = transcribe_audio(&provider, &api_key, None, audio_data)?;

// Copy to clipboard
copy_to_clipboard(&text, ClipboardMethod::Auto)?;
```

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `ffmpeg` | Yes | Desktop audio encoding via FFmpeg subprocess |
| `clipboard` | Yes | Clipboard support via arboard/xclip/wl-copy |
| `local-whisper` | Yes | Local whisper.cpp transcription (requires model) |
| `embedded-encoder` | No | Mobile MP3 encoding via mp3lame (no FFmpeg) |

## Modules

| Module | Description |
|--------|-------------|
| `audio` | `AudioRecorder`, `AudioChunk`, `RecordingOutput`, recording utilities |
| `transcribe` | Single-file and parallel chunked transcription |
| `provider` | Provider registry and `TranscriptionBackend` trait |
| `config` | `TranscriptionProvider` enum (OpenAI, Mistral, Groq, etc.) |
| `settings` | User preferences (provider, API keys, language, hotkeys) |
| `preset` | Named configuration presets |
| `post_processing` | LLM-based transcription cleanup |
| `ollama` | Ollama client for local LLM post-processing |
| `clipboard` | System clipboard operations with multiple backends |
| `model` | Whisper model management |
| `state` | Recording state machine |
| `verbose` | Debug logging utilities |

## License

MIT
