# Whis Deep Dive (Book)

> **Note**: This book is experimental learning documentation. It may be reorganized or removed in the future. Its primary purpose is to help maintain deep understanding of the codebase — to not lose grip on the coding craft through over-reliance on AI assistance.

## What This Is

A 28-chapter deep dive into building a voice-to-text application in Rust. Covers:

- Rust fundamentals refresher
- Audio capture and encoding
- Transcription provider integration
- Desktop app development with Tauri
- Cross-platform considerations
- Testing and release workflows

## Building the Book

```bash
just book          # Build the book
just book-serve    # Build and serve with live reload
```

Or manually:

```bash
cd book
mdbook build       # Output in book/book/
mdbook serve       # Serve at http://localhost:3000
```

## Status

This is a living document that may change significantly. For canonical project information, refer to:

- [README.md](../README.md) — Project overview
- [CONTRIBUTING.md](../CONTRIBUTING.md) — Development setup
