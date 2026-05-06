# Phase S0: Project Scaffold

## Date
2026-05-06

## Goal
Initialize the VoxType project with a solid foundation: Tauri v2, React 19 frontend, Rust backend, comprehensive documentation.

## Deliverables

### 1. Tauri v2 Project
- [x] Scaffolded with `create-tauri-app`, React+TypeScript template
- [x] Configured two windows: "capsule" (floating overlay) and "main" (settings)
- [x] Tauri plugins: store, window, opener, shell

### 2. Frontend Setup
- [x] React 19 + TypeScript 5.8 + Vite 7
- [x] Tailwind CSS 4 with custom brand tokens (oklch color system)
- [x] Zustand 5 for state management (recording, settings, history stores)
- [x] Framer Motion 12 for animations
- [x] Lucide React for icons
- [x] date-fns for date formatting
- [x] Component structure: Capsule, Settings, History, Tray

### 3. Rust Backend
- [x] Cargo.toml with all core dependencies (cpal, enigo, reqwest, arboard, hound, tokio)
- [x] Module structure: audio, stt, llm, output, pipeline, hotkey, config, history, commands
- [x] Error types: `VoxTypeError` with thiserror derive
- [x] Configuration types: `AppConfig`, `SttConfig`, `LlmConfig`, `HotkeyConfig`, `TranslateConfig`
- [x] STT provider trait + 3 provider stubs (Groq, OpenAI, Deepgram) + Local provider
- [x] LLM provider trait + OpenAI-compatible implementation
- [x] Pipeline orchestration (STT → LLM Polish → Output)
- [x] Output module: clipboard paste with original content restore
- [x] Tauri command handlers: recording, settings, history, clipboard
- [x] State management: `AppState` with `Arc<Mutex<>>` for thread safety

### 4. Documentation
- [x] `docs/README.md` - Documentation index
- [x] `docs/architecture.md` - Full system architecture
- [x] `docs/development-plan.md` - 8-phase development roadmap
- [x] `docs/progress.md` - Real-time progress tracker
- [x] `docs/api-contract.md` - Rust ↔ Frontend API contract
- [x] `docs/design/ui-design.md` - UI/UX specification
- [x] `docs/design/capsule-design.md` - Capsule widget design
- [x] `docs/adr/001-choose-tauri-rust.md` - Framework decision
- [x] `docs/adr/002-use-cpal-for-audio.md` - Audio library decision
- [x] `docs/adr/003-stt-provider-abstraction.md` - STT abstraction decision
- [x] `docs/phases/01-scaffold.md` - This document

### 5. Git Setup
- [ ] Repository initialization
- [ ] Push to GitHub

## Reference Sources

- **Typeless** (typeless.com) - User experience reference: floating UI, hotkey workflow, AI polish features
- **tover0314-w/opentypeless** (GitHub, ⭐175) - Architecture reference: Tauri v2 + React, STT/LLM abstraction, BYOK model
- **kuleka/OpenTypeless** (GitHub, ⭐104) - macOS-native reference: WhisperKit integration, scene detection
- **wkwunju/openTypeless** (GitHub, ⭐3) - Simplicity reference: single-file Python, Fn key approach

## Improvements Over References

VoxType aims to be better than existing OpenTypeless implementations:
1. **Better IME handling** - Proper CJK input method bypass
2. **Better capsule UI** - More polished floating widget with streaming transcript
3. **More provider options** - Custom endpoints for both STT and LLM
4. **Better docs** - Comprehensive architecture docs and ADRs from day one
5. **Better test coverage** - Mandatory tests for every phase

## Review Notes
- All provider interfaces use `async_trait` for clean async abstraction
- Configuration types mirror between Rust and TypeScript
- Capsule uses a separate Tauri window for true always-on-top behavior
- Clipboard paste is the primary output method (bypasses IME entirely)
- Rust compilation pending (need to verify cargo build succeeds)

## Next Phase
S1: Audio Capture + Hotkey - Implement microphone recording and global hotkey registration.
