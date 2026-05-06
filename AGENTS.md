# VoxType Development Guide

## Project Overview

VoxType is an open-source AI voice input tool for desktop — a Typeless alternative. Speak naturally, get polished text at your cursor in any app.

## Tech Stack

- **Framework**: Tauri v2 (Rust backend + webview frontend)
- **Backend**: Rust (edition 2021) — `src-tauri/src/`
- **Frontend**: React 19 + TypeScript 5.8 + Vite 7 — `src/`
- **Styling**: Tailwind CSS 4
- **State**: Zustand 5
- **Package Manager**: pnpm

## Commands

```bash
# Development
pnpm tauri dev          # Start Tauri dev mode (frontend + Rust)

# Build
pnpm build              # Build frontend only
cargo build             # Build Rust backend only (in src-tauri/)
pnpm tauri build        # Full production build

# Testing (when we add tests)
cargo test              # Rust tests
pnpm test               # Frontend tests (vitest)

# Lint
cargo clippy            # Rust lint
pnpm lint               # Frontend lint (when configured)
```

## Architecture

```
src-tauri/src/          # Rust backend
├── lib.rs              # App entry, plugin registration, command handler registration
├── main.rs             # Binary entry
├── state.rs            # Global shared state (AppState with Arc<Mutex<>>)
├── error.rs            # VoxTypeError enum
├── config/mod.rs       # Configuration types (SttConfig, LlmConfig, AppConfig)
├── audio/              # Audio capture (cpal), VAD, preprocessing
├── stt/                # STT provider trait + implementations (Groq, OpenAI, Deepgram, Local)
├── llm/                # LLM provider trait + OpenAI-compatible implementation
├── output/             # Keyboard simulation, clipboard, IME bypass
├── pipeline/           # Recording → STT → LLM → Output orchestration
├── hotkey/             # Global hotkey registration per platform
├── history/            # SQLite-based transcription history
└── commands/           # Tauri command handlers (Rust ↔ JS bridge)
    ├── recording.rs    # start_recording, stop_recording
    ├── settings.rs     # get_settings, save_settings
    ├── history.rs      # get_history, add_history_item, remove_history_item
    └── clipboard.rs    # copy_to_clipboard, paste_text

src/                    # React/TS frontend
├── App.tsx             # Root component (router for capsule/main windows)
├── main.tsx            # React entry point
├── stores/             # Zustand state management
│   ├── recordingStore  # Recording state machine (idle→recording→transcribing→polishing→done)
│   ├── settingsStore   # STT/LLM config, theme, hotkey
│   └── historyStore    # Transcription history list
├── components/
│   ├── capsule/        # Floating overlay (recording indicator, transcript, waveform)
│   ├── settings/       # Full settings panel with STT/LLM/hotkey config
│   └── history/        # Past transcriptions with search
└── styles/
    └── global.css      # Tailwind directives + @theme tokens
```

## Key Design Decisions

1. **Multiple windows**: Two Tauri windows — "capsule" (always-on-top overlay) and "main" (settings window)
2. **STT/LLM abstraction**: Provider traits allow custom endpoints alongside built-in providers
3. **Clipboard-first output**: Cmd+V paste with clipboard save/restore to bypass IME issues
4. **BYOK model**: No cloud dependency — user brings their own API keys. Optional cloud proxy for future.
5. **Platform hotkeys**: macOS uses Fn key (kVK_Function), Windows/Linux uses Right Alt

## Configuration

- Config stored via `tauri-plugin-store` (JSON files in app config dir)
- API keys stored in platform keychain (planned)
- Default config in `config::AppConfig::default()`

## Reference

- https://github.com/tover0314-w/opentypeless (main reference, Tauri v2 architecture)
- https://typeless.com (UX reference)
- https://v2.tauri.app (Tauri v2 docs)
