# VoxType

> Speak, don't type. — AI voice input for desktop, open-source.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Tauri v2](https://img.shields.io/badge/Tauri-v2-FFC131?logo=tauri)](https://v2.tauri.app)
[![Rust](https://img.shields.io/badge/Rust-edition2021-000000?logo=rust)](https://www.rust-lang.org)
[![React](https://img.shields.io/badge/React-19-61DAFB?logo=react)](https://react.dev)

VoxType is an open-source AI voice input tool — a [Typeless](https://typeless.com) alternative. Press a hotkey, speak naturally, and get polished text at your cursor in any app.

```
🎙 Press Fn → Speak → Release → ⚡ AI Polish → 📋 Text at cursor
```

---

## Table of Contents

- [Quick Start](#quick-start)
- [STT Providers](#stt-providers)
- [LLM Providers](#llm-providers)
- [Development](#development)
  - [Prerequisites](#prerequisites)
  - [Project Setup](#project-setup)
  - [Dev Mode](#dev-mode)
  - [Project Structure](#project-structure)
  - [Architecture](#architecture)
- [Testing](#testing)
  - [Rust Unit Tests](#rust-unit-tests)
  - [Frontend Tests](#frontend-tests)
  - [Mock/Integration Tests](#mockintegration-tests)
  - [CI/CD](#cicd)
- [Building & Release](#building--release)
  - [Development Build](#development-build)
  - [Production Build](#production-build)
  - [Platform-Specific](#platform-specific)
  - [Code Signing](#code-signing)
  - [Auto-Update](#auto-update)
- [Configuration](#configuration)
  - [Config File Location](#config-file-location)
  - [Config Schema](#config-schema)
  - [Pre-configured Setup](#pre-configured-setup)
- [Features](#features)
- [Documentation](#documentation)
- [Troubleshooting](#troubleshooting)
- [Performance](#performance)
- [License](#license)

---

## Quick Start

```bash
# 1. Clone
git clone https://github.com/adamcjm/VoxType.git
cd VoxType

# 2. Install dependencies
pnpm install

# 3. Configure API keys (create config directory)
mkdir -p ~/.VoxType
```

Create `~/.VoxType/config.json`:

```json
{
  "stt": {
    "provider": "deepgram",
    "api_key": "YOUR_DEEPGRAM_API_KEY",
    "model": "nova-2",
    "language": "zh"
  },
  "llm": {
    "provider": "deepseek",
    "api_key": "YOUR_DEEPSEEK_API_KEY",
    "model": "deepseek-chat",
    "temperature": 0.3
  },
  "translate": { "enabled": false }
}
```

```bash
# 4. Start development
pnpm tauri dev
```

**Hotkey**: Press **Fn** (macOS 🌐 key) to start/stop recording.

---

## STT Providers

VoxType supports 4 speech-to-text backends + custom endpoints. All configurations are hot-swappable in Settings.

### Groq (Recommended — Free Tier)

| Item | Value |
|------|-------|
| **Cost** | Free (rate-limited) |
| **Latency** | ~200ms |
| **Models** | `whisper-large-v3-turbo`, `whisper-large-v3` |
| **Signup** | [console.groq.com](https://console.groq.com/keys) |

```json
{
  "stt": {
    "provider": "groq",
    "base_url": "https://api.groq.com/openai/v1",
    "api_key": "gsk_...",
    "model": "whisper-large-v3-turbo"
  }
}
```

### Deepgram (Free $200 Credit)

| Item | Value |
|------|-------|
| **Cost** | $200 free credit, then pay-as-you-go |
| **Latency** | ~300ms (real-time streaming) |
| **Models** | `nova-2`, `nova-3`, `whisper` |
| **Signup** | [console.deepgram.com](https://console.deepgram.com) |

```json
{
  "stt": {
    "provider": "deepgram",
    "base_url": "https://api.deepgram.com/v1",
    "api_key": "your-deepgram-key",
    "model": "nova-2"
  }
}
```

### OpenAI Whisper

```json
{
  "stt": {
    "provider": "openai",
    "base_url": "https://api.openai.com/v1",
    "api_key": "sk-...",
    "model": "whisper-1"
  }
}
```

### Local Whisper (Offline)

No API key required. Downloads a model file to your machine.

```json
{
  "stt": {
    "provider": "local",
    "base_url": "http://localhost:8080",
    "model": "ggml-small.bin"
  }
}
```

**Prerequisites:**
```bash
# Install whisper.cpp
brew install whisper-cpp          # macOS
sudo apt install whisper-cpp      # Linux

# Or download model via VoxType Settings → STT → Local Whisper → Download
```

### Custom Endpoint

Any OpenAI-compatible `/audio/transcriptions` endpoint:

```json
{
  "stt": {
    "provider": "custom",
    "base_url": "https://your-server.com/v1",
    "api_key": "...",
    "model": "whisper-v3"
  }
}
```

---

## LLM Providers

VoxType uses LLMs for text polishing: remove filler words, fix homophone errors, add punctuation, translate.

| Provider | Default Model | Cost | Setup |
|----------|--------------|------|-------|
| **DeepSeek** | `deepseek-chat` | Low ($0.14/M tokens) | [platform.deepseek.com](https://platform.deepseek.com) |
| **OpenAI** | `gpt-4o-mini` | $0.15/M tokens | [platform.openai.com](https://platform.openai.com) |
| **Groq** | `llama-3.3-70b` | Free tier | [console.groq.com](https://console.groq.com) |
| **Gemini** | `gemini-2.0-flash` | Free tier | [aistudio.google.com](https://aistudio.google.com) |
| **Ollama** | `llama3` | Free (local) | [ollama.com](https://ollama.com) |
| **Custom** | any | varies | OpenAI-compatible API |

### Recommended Setup (Free / Low Cost)

```
STT:  Groq (whisper-large-v3-turbo)    →  Free
LLM:  DeepSeek (deepseek-chat)          →  ~$0.14/M tokens
```

---

## Development

### Prerequisites

| Tool | Version | Check |
|------|---------|-------|
| **Rust** | 1.80+ | `rustc --version` |
| **Node.js** | 20+ | `node --version` |
| **pnpm** | 10+ | `pnpm --version` |
| **Tauri CLI** | 2.x | `cargo install tauri-cli` |

**Platform-specific dependencies:**

```bash
# macOS
xcode-select --install

# Windows
# Install Microsoft Visual Studio C++ Build Tools

# Linux (Ubuntu/Debian)
sudo apt install libwebkit2gtk-4.1-dev libappindicator3-dev \
  librsvg2-dev patchelf libasound2-dev
```

See [Tauri Prerequisites](https://v2.tauri.app/start/prerequisites/) for details.

### Project Setup

```bash
git clone https://github.com/adamcjm/VoxType.git
cd VoxType

# Install all dependencies
pnpm install
```

### Dev Mode

```bash
# Start Tauri development (frontend hot-reload + Rust rebuild)
pnpm tauri dev
```

This launches:
1. **Vite dev server** (`http://localhost:1420`) — React HMR
2. **Capsule window** — always-on-top transparent overlay (hidden until recording)
3. **Main window** — Settings panel (hidden, open via system tray)
4. **System tray icon** — right-click for menu

**Dev tips:**
- Frontend changes hot-reload instantly
- Rust changes trigger `cargo build` and app restart
- DevTools available for main window (debug mode only)
- Set `RUST_LOG=debug` for verbose logging:
  ```bash
  RUST_LOG=debug pnpm tauri dev
  ```

### Project Structure

```
voxtype/
├── src/                          # React 19 + TypeScript frontend
│   ├── components/
│   │   ├── capsule/              # Floating overlay UI
│   │   │   ├── Capsule.tsx       #   Main capsule (state-driven animations)
│   │   │   ├── Waveform.tsx      #   Animated audio bars
│   │   │   └── StatusBadge.tsx   #   Color-coded state indicator
│   │   ├── settings/
│   │   │   └── Settings.tsx      # Full settings panel
│   │   └── history/
│   │       └── History.tsx       # Transcription history list
│   ├── hooks/
│   │   └── useRecording.ts       # Tauri invoke/listen bridge
│   ├── stores/                   # Zustand state management
│   │   ├── recordingStore.ts     #   Recording state machine
│   │   ├── settingsStore.ts      #   Provider config, hotkeys
│   │   └── historyStore.ts       #   History items
│   └── styles/
│       └── global.css            # Tailwind CSS 4 + @theme tokens
│
├── src-tauri/                    # Rust backend
│   ├── tauri.conf.json           # Tauri config (windows, bundle, plugins)
│   ├── Cargo.toml                # Rust dependencies
│   └── src/
│       ├── main.rs               # Binary entry
│       ├── lib.rs                # App setup, plugin registration
│       ├── state.rs              # AppState (config + audio + history)
│       ├── error.rs              # VoxTypeError enum
│       ├── audio/                # Audio capture (cpal), VAD, preprocessing
│       ├── stt/                  # STT providers (Groq, OpenAI, Deepgram, Local, Custom)
│       ├── llm/                  # LLM providers + prompts (cleanup, translate, format)
│       ├── output/               # Keyboard simulation + clipboard paste + IME bypass
│       ├── pipeline/             # STT → LLM → Output orchestration
│       ├── hotkey/               # Platform-specific hotkey descriptions
│       ├── config/               # AppConfig load/save + defaults
│       ├── history/              # SQLite transcription history (CRUD + search)
│       ├── model_manager/        # Whisper model download (HuggingFace)
│       └── commands/             # Tauri command handlers
│
├── docs/                         # Full documentation
│   ├── architecture.md           # System architecture
│   ├── api-contract.md           # Rust ↔ Frontend API contract
│   ├── development-plan.md       # 8-phase development roadmap
│   ├── progress.md               # Real-time progress tracking
│   ├── design/                   # UI/UX design specs
│   └── phases/                   # Per-phase deliverables & reviews
│
├── AGENTS.md                     # AI agent development guide
├── package.json
├── pnpm-lock.yaml
└── README.md
```

### Architecture

```
┌─────────────────────────────────────────────────────────┐
│                     VoxType Desktop App                  │
│                                                          │
│  ┌─────────────────────┐   ┌───────────────────────────┐│
│  │  React 19 / TS      │   │     Rust Backend          ││
│  │                     │   │                           ││
│  │  Capsule (overlay)  │◄──┤  Hotkey → Audio Capture  ││
│  │  Settings Panel     │   │  VAD → Preprocess → WAV  ││
│  │  History Panel      │   │  STT Provider → API call ││
│  │  System Tray        │   │  LLM Provider → Polish   ││
│  │                     │   │  Clipboard Paste → Cursor││
│  └─────────────────────┘   └───────────────────────────┘│
└─────────────────────────────────────────────────────────┘
```

**Data Flow:**
```
Fn key → Audio Capture (cpal, 16kHz PCM) → VAD Gate → Preprocess
  → STT (Groq/Deepgram/OpenAI/Local) → Raw Text
  → LLM Polish (DeepSeek/OpenAI/Gemini/Ollama) → Final Text
  → Clipboard Paste (Cmd+V) → Restore Clipboard → Save to History (SQLite)
```

---

## Testing

### Rust Unit Tests

```bash
# Run all Rust tests
cargo test --manifest-path src-tauri/Cargo.toml

# Run specific module tests
cargo test --manifest-path src-tauri/Cargo.toml stt          # STT providers
cargo test --manifest-path src-tauri/Cargo.toml audio         # Audio/VAD/preprocess
cargo test --manifest-path src-tauri/Cargo.toml llm           # LLM polish
cargo test --manifest-path src-tauri/Cargo.toml history       # History SQLite
cargo test --manifest-path src-tauri/Cargo.toml output        # Keyboard/clipboard

# Run with output
cargo test --manifest-path src-tauri/Cargo.toml -- --nocapture

# Run a single test
cargo test --manifest-path src-tauri/Cargo.toml test_polish_success

# With backtrace on failure
RUST_BACKTRACE=1 cargo test --manifest-path src-tauri/Cargo.toml
```

**Current test coverage:** 28 tests, 28 passed, 0 failed.

**Test modules:**
| Module | Tests | Description |
|--------|-------|-------------|
| `audio::capture` | 3 | WAV encoding, audio capture creation |
| `audio::vad` | 5 | Voice activity detection (speech/silence/segments) |
| `audio::preprocess` | 4 | Noise gate, gain normalization |
| `stt::common` | 3 | OpenAI-compatible API calls (wiremock) |
| `llm::openai_compat` | 5 | LLM polish, auth errors, empty text, translate |
| `output::keyboard` | 1 | Shift character detection |
| `output::clipboard` | 1 | Empty text handling |
| `output::ime` | 1 | IME detection safety |
| `history` | 3 | CRUD operations, search, delete |
| `hotkey` | 2 | Platform key descriptions |

### Frontend Tests

```bash
# Setup (when we add vitest)
pnpm add -D vitest @testing-library/react @testing-library/jest-dom jsdom

# Run frontend tests
pnpm test

# Watch mode
pnpm test --watch
```

### Mock/Integration Tests

STT and LLM tests use [wiremock](https://docs.rs/wiremock) for HTTP mocking:

```rust
// Example: STT API mock test
#[tokio::test]
async fn test_transcribe_openai_compat_success() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/audio/transcriptions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(
            serde_json::json!({"text": "Hello world"})
        ))
        .mount(&server)
        .await;

    let result = transcribe_openai_compat(/* ... */).await;
    assert_eq!(result.unwrap(), "Hello world");
}
```

### CI/CD

For GitHub Actions, create `.github/workflows/test.yml`:

```yaml
name: Test
on: [push, pull_request]
jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: pnpm/action-setup@v4
      - run: pnpm install
      - run: cargo test --manifest-path src-tauri/Cargo.toml
      - run: pnpm build
```

---

## Building & Release

### Development Build

```bash
# Fast build for testing (no optimization, debug symbols)
pnpm tauri dev

# Or build Rust and frontend separately
pnpm build                              # Frontend only
cargo build --manifest-path src-tauri/Cargo.toml   # Rust only
```

### Production Build

```bash
# Full production build for current platform
pnpm tauri build

# Build for specific target
pnpm tauri build --target aarch64-apple-darwin   # Apple Silicon
pnpm tauri build --target x86_64-apple-darwin    # Intel Mac
pnpm tauri build --target x86_64-pc-windows-msvc # Windows
pnpm tauri build --target x86_64-unknown-linux-gnu  # Linux
```

**Build outputs:**

| Platform | Output | Location |
|----------|--------|----------|
| macOS | `.dmg` | `src-tauri/target/release/bundle/dmg/` |
| macOS | `.app` | `src-tauri/target/release/bundle/macos/` |
| Windows | `.msi` | `src-tauri/target/release/bundle/msi/` |
| Windows | `.exe` | `src-tauri/target/release/bundle/nsis/` |
| Linux | `.AppImage` | `src-tauri/target/release/bundle/appimage/` |
| Linux | `.deb` | `src-tauri/target/release/bundle/deb/` |

### Platform-Specific

#### macOS

```bash
# Build for both architectures (Universal Binary)
pnpm tauri build --target universal-apple-darwin

# The .app bundle requires these permissions:
# - Microphone (audio capture)
# - Accessibility (keyboard simulation)

# To add entitlements, create src-tauri/entitlements.plist:
# <?xml version="1.0" encoding="UTF-8"?>
# <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "...">
# <plist version="1.0">
# <dict>
#     <key>com.apple.security.device.audio-input</key>
#     <true/>
# </dict>
# </plist>
```

#### Windows

```bash
# NSIS installer (requires NSIS installed)
pnpm tauri build

# MSI installer (requires WiX Toolset)
pnpm tauri build --bundles msi
```

#### Linux

```bash
# AppImage (recommended for distribution)
pnpm tauri build --bundles appimage

# Debian package
pnpm tauri build --bundles deb

# RPM package
pnpm tauri build --bundles rpm
```

### Code Signing

#### macOS Code Signing & Notarization

```bash
# Set environment variables
export APPLE_SIGNING_IDENTITY="Developer ID Application: Your Name (TEAMID)"
export APPLE_CERTIFICATE="base64-encoded .p12 file"
export APPLE_CERTIFICATE_PASSWORD="keychain-password"
export APPLE_ID="your@email.com"
export APPLE_PASSWORD="app-specific-password"
export APPLE_TEAM_ID="TEAMID"

# Build with signing
pnpm tauri build

# Notarize (after build)
xcrun notarytool submit src-tauri/target/release/bundle/dmg/VoxType_*.dmg \
  --apple-id "$APPLE_ID" --team-id "$APPLE_TEAM_ID" --password "$APPLE_PASSWORD" --wait

# Staple notarization ticket
xcrun stapler staple src-tauri/target/release/bundle/dmg/VoxType_*.dmg
```

#### Windows Code Signing

```bash
# With OV/EV certificate
export WINDOWS_PFX_FILE="/path/to/cert.pfx"
export WINDOWS_PFX_PASSWORD="password"

pnpm tauri build
```

### Auto-Update

VoxType uses Tauri's built-in updater with GitHub Releases:

```json
// src-tauri/tauri.conf.json
{
  "plugins": {
    "updater": {
      "endpoints": [
        "https://github.com/adamcjm/VoxType/releases/latest/download/latest.json"
      ]
    }
  }
}
```

**Publishing an update:**
1. Bump version in `tauri.conf.json` and `package.json`
2. Run `pnpm tauri build`
3. Create GitHub Release with generated artifacts
4. The updater signature is auto-generated

**Generating updater key pair (one-time):**
```bash
pnpm tauri signer generate -- -w ~/.vox-type-updater-key
```

---

## Configuration

### Data Directory

All VoxType data lives in a single directory:

| Platform | Path |
|----------|------|
| macOS | `~/.VoxType/` |
| Windows | `C:\Users\<user>\.VoxType\` |
| Linux | `~/.VoxType/` |

```
~/.VoxType/
├── config.json     # User settings (STT/LLM providers, hotkeys, theme)
├── history.db      # SQLite transcription history
└── models/          # Downloaded Whisper models
    └── ggml-small.bin
```

### Config Options

There are **two ways** to set up VoxType:

#### Option A: `.env` file (developers, fastest)

```bash
cp .env.example .env
# Edit .env with your API keys
pnpm tauri dev
```

`.env` takes priority over saved settings. Ideal for development.

#### Option B: In-app Settings (recommended for users)

On first launch, VoxType detects no API keys are configured and **auto-opens the Settings panel**. Fill in your provider + API key + model, click Save.

Settings are persisted to `~/.VoxType/config.json` and reloaded on next launch.

### Config File Path

| Platform | Path |
|----------|------|
| macOS | `~/.VoxType/config.json` |
| Windows | `C:\Users\<user>\.VoxType\config.json` |
| Linux | `~/.VoxType/config.json` |

### Config Schema

```json
{
  "stt": {
    "provider": "groq | openai | deepgram | local | custom",
    "base_url": "API endpoint",
    "api_key": "Your API key",
    "model": "Model name",
    "language": "zh | en | ja | ko | ..."
  },
  "llm": {
    "provider": "openai | deepseek | anthropic | gemini | groq | ollama | custom",
    "base_url": "API endpoint",
    "api_key": "Your API key",
    "model": "Model name",
    "temperature": 0.3,
    "max_tokens": 4096,
    "custom_prompt": null
  },
  "translate": {
    "enabled": false,
    "source_lang": "auto | zh | en | ja | ko",
    "target_lang": "en | zh | ja | ko | de | fr | es"
  },
  "hotkey": {
    "macos": "Fn",
    "other": "RightAlt"
  },
  "theme": "system | light | dark"
}
```

### Pre-configured Setup

For team deployment, create a config template:

```bash
mkdir -p ~/.VoxType

cat > ~/.VoxType/config.json << 'EOF'
{
  "stt": {
    "provider": "deepgram",
    "base_url": "https://api.deepgram.com/v1",
    "api_key": "PASTE_YOUR_KEY_HERE",
    "model": "nova-2",
    "language": "zh"
  },
  "llm": {
    "provider": "deepseek",
    "base_url": "https://api.deepseek.com/v1",
    "api_key": "PASTE_YOUR_KEY_HERE",
    "model": "deepseek-chat",
    "temperature": 0.3,
    "max_tokens": 4096,
    "custom_prompt": null
  },
  "translate": { "enabled": false },
  "hotkey": { "macos": "Fn", "other": "RightAlt" },
  "theme": "system"
}
EOF
```

---

## Features

- **Global Hotkey**: Fn (macOS) / Right Alt (Windows/Linux), press to toggle recording
- **AI Transcription**: 4 STT providers + custom endpoint + local offline Whisper
- **Text Polishing**: LLM cleans filler words, fixes homophones, adds punctuation, structures text
- **Translation Mode**: Speak in one language, output in another
- **Custom Providers**: BYOK model with configurable endpoints
- **Floating Capsule**: Always-on-top overlay with waveform, timer, real-time transcript
- **Cross-Platform**: macOS, Windows, Linux via Tauri v2
- **Privacy-First**: BYOK, local STT option, clipboard restore, no audio retention
- **History**: SQLite-backed transcription history with full-text search
- **Hot-Swappable Settings**: Change providers and models without restarting

---

## Documentation

Full documentation in `docs/`:

| Document | Description |
|----------|-------------|
| [architecture.md](docs/architecture.md) | System architecture and design |
| [api-contract.md](docs/api-contract.md) | Rust ↔ Frontend API contract |
| [development-plan.md](docs/development-plan.md) | Phased development roadmap |
| [progress.md](docs/progress.md) | Real-time development progress |
| [design/ui-design.md](docs/design/ui-design.md) | UI/UX specifications |
| [design/capsule-design.md](docs/design/capsule-design.md) | Floating capsule design |
| [phases/](docs/phases/) | Per-phase delivery reports & reviews |

---

## Troubleshooting

### Microphone not working

```bash
# macOS: Check System Settings → Privacy & Security → Microphone
# Ensure your terminal (or VoxType.app) has permission

# Windows: Settings → Privacy → Microphone
# Linux: Check PulseAudio/PipeWire permissions
```

### "Accessibility permission required"

On macOS, VoxType needs Accessibility permission to simulate keyboard input:

```
System Settings → Privacy & Security → Accessibility
→ Enable your terminal app (or VoxType.app)
```

### Fn key doesn't trigger recording

macOS Fn key is the 🌐 (globe) key. Configure in System Settings:
```
System Settings → Keyboard → Press 🌐 key to → Do Nothing
```

If that doesn't work, use the Settings panel → Hotkey to configure a custom key.

### "No speech detected" error

- Check your microphone is connected and enabled
- Speak at a normal volume (VAD threshold: 0.02 RMS)
- Avoid noisy environments
- Set `RUST_LOG=debug` to see VAD energy levels

### STT API errors

```
# Check API key is valid
curl -H "Authorization: Bearer YOUR_KEY" \
  https://api.groq.com/openai/v1/models

# Check Deepgram API key
curl -H "Authorization: Token YOUR_KEY" \
  https://api.deepgram.com/v1/projects
```

### Build fails on macOS

```bash
# Install Xcode Command Line Tools
xcode-select --install

# Ensure Rust is up to date
rustup update stable
```

### Build fails on Linux

```bash
# Install WebKit2GTK and other Tauri dependencies
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev build-essential \
  curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev \
  librsvg2-dev libasound2-dev
```

### Debug logging

```bash
# Enable debug logs
RUST_LOG=debug pnpm tauri dev
RUST_LOG=voxtype=debug,trace pnpm tauri dev  # Very verbose
```

---

## Performance

| Metric | Value |
|--------|-------|
| Binary size (Rust) | ~5MB |
| Bundle size (macOS .dmg) | ~15MB |
| Memory (idle) | ~25MB |
| Memory (recording) | ~40MB |
| CPU (idle) | 0% |
| CPU (recording) | ~2% |
| STT latency (Groq) | ~200ms |
| LLM polish (DeepSeek) | ~500ms |
| End-to-end (voice → text) | ~1-2s |

---

## License

MIT

## Credits

Inspired by [Typeless](https://typeless.com) and the open-source community, especially:
- [tover0314-w/opentypeless](https://github.com/tover0314-w/opentypeless)
- [kuleka/OpenTypeless](https://github.com/kuleka/OpenTypeless)

Built with [Tauri](https://tauri.app), [React](https://react.dev), [Tailwind CSS](https://tailwindcss.com).
