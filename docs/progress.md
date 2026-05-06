# Development Progress

> Last updated: 2026-05-06

## Summary

| Phase | Status | Start Date | End Date | Notes |
|-------|--------|------------|----------|-------|
| S0: Scaffold | In Progress | 2026-05-06 | - | Project initialization |
| S1: Audio + Hotkey | Not Started | - | - | |
| S2: STT Integration | Not Started | - | - | |
| S3: Keyboard Output | Not Started | - | - | |
| S4: Capsule UI | Not Started | - | - | |
| S5: LLM Polish | Not Started | - | - | |
| S6: Settings + History | Not Started | - | - | |
| S7: Local Whisper + Release | Not Started | - | - | |

## S0: Project Scaffold

### Completed
- [x] Created Tauri v2 project with React+TS template
- [x] Installed frontend dependencies: Zustand, Framer Motion, Lucide React, date-fns
- [x] Installed Tailwind CSS v4 with @tailwindcss/vite
- [x] Installed Tauri plugins: store, opener, shell
- [x] Configured Rust Cargo.toml: cpal, enigo, reqwest, arboard, hound, async-trait, tokio
- [x] Created complete directory structure (Rust modules + React components)
- [x] Defined core types: AppConfig, SttConfig, LlmConfig, HotkeyConfig, TranslateConfig
- [x] Implemented error types: VoxTypeError with thiserror
- [x] Implemented STP provider trait + Groq, OpenAI, Deepgram, Local stubs
- [x] Implemented LLM provider trait + OpenAI-compatible implementation with prompts
- [x] Implemented pipeline orchestration (STT → LLM → Output flow)
- [x] Implemented output module: clipboard paste with restore, IME bypass strategy
- [x] Created React stores: recordingStore, settingsStore, historyStore
- [x] Created React components: Capsule (floating), Settings (full panel), History (list)
- [x] Configured Tailwind CSS with VoxType brand colors and animations
- [x] Created comprehensive docs directory structure (12 documents)
- [x] Created AGENTS.md for AI development context
- [x] Rust backend: 0 errors, 0 warnings
- [x] Frontend: builds successfully
- [x] Git repository initialized with 2 commits
- [ ] Push to GitHub (requires `gh auth login`)

## S1: Audio Capture + Hotkey

### Completed
- [x] AudioCapture: cpal-based microphone capture at 16kHz mono PCM
- [x] Dedicated audio thread to isolate non-Send cpal::Stream (macOS CoreAudio)
- [x] Channel-based start/stop commands (mpsc)
- [x] WAV encoding: f32 → i16 PCM via hound crate
- [x] VAD: energy-based speech detection with configurable threshold
- [x] VAD: speech segment detection with min speech/silence filtering
- [x] Audio preprocessing: noise gate + gain normalization
- [x] Device enumeration: list_input_devices() with AudioDeviceInfo
- [x] Hotkey infrastructure: platform-specific key descriptions
- [x] Pipeline refactored to free function (Send-safe async)
- [x] AppState simplified: config + audio_capture only
- [x] All async Tauri commands are Send-safe
- [x] Recording commands: start_recording, stop_recording, list_audio_devices
- [x] Integrated VAD gate before STT pipeline
- [x] Integrated audio preprocessing before STT
- [x] **14 tests, 14 passed, 0 failed**
- [x] Rust: 0 errors, 0 warnings
- [x] Frontend: builds successfully

### Pending (deferred to later)
- [ ] OS-level global hotkey (CGEventTap/RegisterHotKey/XGrabKey)
- [ ] Hotkey Tauri event emission to frontend

### Next Phase
S2: STT Integration - Implement actual Groq/OpenAI/Deepgram API calls
