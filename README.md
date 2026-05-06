# VoxType

> Speak, don't type. — AI voice input for desktop, open-source.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Tauri v2](https://img.shields.io/badge/Tauri-v2-FFC131?logo=tauri)](https://v2.tauri.app)
[![Rust](https://img.shields.io/badge/Rust-edition2021-000000?logo=rust)](https://www.rust-lang.org)
[![React](https://img.shields.io/badge/React-19-61DAFB?logo=react)](https://react.dev)

VoxType is an open-source AI voice input tool — a [Typeless](https://typeless.com) alternative. Press a hotkey, speak naturally, and get polished text at your cursor in any app.

## Features

- **Global Hotkey**: Fn (macOS) / Right Alt (Windows/Linux), press to toggle recording
- **AI Transcription**: Multiple STT providers (Groq, OpenAI Whisper, Deepgram, local Whisper)
- **Text Polishing**: LLM-powered cleanup — remove filler words, fix punctuation, add structure
- **Translation Mode**: Speak in one language, output in another
- **Custom Providers**: Bring your own API keys or use custom endpoints
- **Floating Capsule**: Non-intrusive overlay showing recording status and transcript
- **Cross-Platform**: macOS, Windows, Linux via Tauri
- **Privacy-First**: BYOK model, local STT option, API keys in system keychain

## Architecture

```
Hotkey → Audio Capture → STT Transcription → LLM Polish → Text Output at Cursor
```

- **Backend**: Rust (Tauri v2) — audio, STT, LLM, keyboard simulation, system integration
- **Frontend**: React 19 + TypeScript + Tailwind CSS 4 — capsule UI, settings, history

## Development

### Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Node.js](https://nodejs.org/) 20+
- [pnpm](https://pnpm.io/) 10+
- Platform-specific Tauri prerequisites: [Tauri Prerequisites](https://v2.tauri.app/start/prerequisites/)

### Setup

```bash
# Clone
git clone https://github.com/YOUR_USERNAME/VoxType.git
cd VoxType

# Install dependencies
pnpm install

# Run in development
pnpm tauri dev

# Build for production
pnpm tauri build
```

## Configuration

All settings are accessible from the in-app Settings panel:

- **Speech Recognition** — choose STT provider and enter API key
- **AI Polish** — choose LLM provider, model, and API key
- **General** — hotkey, theme, language
- **Translation** — source/target language

API keys are stored in the platform keychain (macOS Keychain / Windows Credential Manager).

### Recommended Free Tier Setup

| Provider | Model | Tier |
|----------|-------|------|
| STT | Groq | `whisper-large-v3-turbo` | Free |
| LLM | DeepSeek | `deepseek-chat` | Free |

## Documentation

See [docs/](./docs) for full architecture, API contracts, design specs, and ADRs.

## License

MIT

## Credits

Inspired by [Typeless](https://typeless.com) and the open-source community, especially [tover0314-w/opentypeless](https://github.com/tover0314-w/opentypeless).

Built with [Tauri](https://tauri.app) and [React](https://react.dev).
