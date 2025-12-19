# Contributing to Whis

Thanks for your interest in contributing! This guide will help you get started.

## Quick Start

```bash
# Clone the repository
git clone https://github.com/frankdierolf/whis.git
cd whis

# Show required system dependencies
just setup-info

# Build everything
just build-all
```

## Prerequisites

- **Rust** (latest stable)
- **Node.js 20+** (for desktop frontend)
- **FFmpeg** (`sudo apt install ffmpeg` or `brew install ffmpeg`)
- **just** task runner (`cargo install just`)

### Linux Dependencies

```bash
sudo apt-get install -y \
  libasound2-dev libx11-dev libxtst-dev \
  libwebkit2gtk-4.1-dev libappindicator3-dev \
  librsvg2-dev patchelf ffmpeg
```

### macOS Dependencies

```bash
brew install ffmpeg
```

## Development Workflow

We use [just](https://github.com/casey/just) for task automation. Run `just` to see all available commands.

### Common Commands

| Command | Description |
|---------|-------------|
| `just build` | Build CLI (debug) |
| `just build-release` | Build CLI (release) |
| `just desktop-dev` | Run desktop app with hot reload |
| `just lint` | Run clippy |
| `just fmt` | Format code |
| `just ci` | Pre-commit check (format + lint) |
| `just clean-all` | Clean all build artifacts |

### Project Structure

```
whis/
├── crates/
│   ├── whis-core/      # Core library (providers, audio, config)
│   ├── whis-cli/       # CLI application (package: whis)
│   ├── whis-desktop/   # Tauri desktop app + Vue frontend
│   └── whis-mobile/    # Tauri mobile app (Android)
├── book/               # mdBook documentation (experimental)
└── justfile            # Task automation
```

For deeper architecture details, see the [book](./book/).

## Making Changes

1. **Fork and clone** the repository
2. **Create a branch** for your changes: `git checkout -b feature/my-feature`
3. **Make your changes**
4. **Run checks**: `just ci`
5. **Commit** with a clear message
6. **Open a Pull Request** with a description of your changes

## Code Style

- Run `just fmt` before committing
- Follow existing patterns in the codebase
- Keep changes focused - one feature/fix per PR
- Add comments for non-obvious logic

## Getting Help

- Open an [issue](https://github.com/frankdierolf/whis/issues) for bugs or questions
- Check existing issues before creating new ones

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
