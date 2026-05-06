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

## S3: Keyboard Output

### Completed
- [x] enigo keyboard simulation: `type_text()` character-by-character
- [x] `simulate_paste_keystroke()` — Cmd+V (macOS) / Ctrl+V (Win/Linux)
- [x] Clipboard paste with save/restore (primary output, bypasses IME)
- [x] Output Manager: `write_text()` with clipbord-first fallback strategy
- [x] IME detection: macOS env var, Linux env vars, Windows placeholder
- [x] Pipeline integrated with `output::write_text()`
- [x] **3 new tests — 20 total, all passed**
- [x] Rust: 0 errors, 0 warnings

### Next Phase
S4: Capsule UI — Floating overlay with waveform animation, real-time transcript, error states

---

## S2: STT Integration

### Completed
- [x] Shared HTTP helper: `transcribe_openai_compat()` for OpenAI-compatible APIs
- [x] Groq Provider: real Whisper API call via multipart/form-data
- [x] OpenAI Whisper Provider: real API call + Custom endpoint support
- [x] Deepgram Provider: native API format with smart_format
- [x] AudioFormat: mime_type() and extension() methods
- [x] Error handling: HTTP errors, JSON parse errors, auth errors
- [x] **3 new mock HTTP tests (wiremock) — 17 total, all passed**
- [x] Rust: 0 errors, 0 warnings
- [x] Frontend: builds successfully

### Next Phase
S3: Keyboard Output — Implement enigo keyboard simulation with IME bypass for real text input

---

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

---

## S0: Project Scaffold
