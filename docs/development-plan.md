# VoxType Development Plan

## Overview

| Item | Detail |
|------|--------|
| **Target Platforms** | macOS, Windows, Linux |
| **Tech Stack** | Tauri v2, Rust, React 19, TypeScript, Tailwind CSS |
| **License** | MIT |
| **Repository** | TBD |

## Development Phases

### S0: Project Scaffold (current)

| Task | Status | Description |
|------|--------|-------------|
| Create Tauri v2 project | Done | `npm create tauri-app` with React+TS template |
| Install frontend deps | Done | Tailwind CSS, Zustand, Framer Motion, Lucide |
| Configure Rust deps | Done | cpal, enigo, reqwest, arboard, hound |
| Directory structure | Done | Rust modules, React components, docs skeleton |
| Core state types | Done | Config, Error, AppState, recording/settings/history stores |
| Provider interfaces | Done | SttProvider, LlmProvider traits defined |
| Capsule UI shell | Done | Basic floating component with state display |
| Settings UI shell | Done | Settings panel with STT/LLM config forms |
| History UI shell | Done | History list component |
| **Phase Review** | Pending | Compile check, architecture review |

### S1: Audio Capture + Hotkey (Planned)

| Task | Status | Description |
|------|--------|-------------|
| Audio capture (cpal) | Pending | Record from default mic, 16kHz mono WAV |
| Audio streaming | Pending | Real-time audio buffer accumulation |
| VAD implementation | Pending | Energy-based voice activity detection |
| Preprocessing | Pending | Noise reduction, gain normalization |
| Global hotkey (macOS) | Pending | Fn key detection via CGEvent |
| Global hotkey (Windows) | Pending | Right Alt via SetWindowsHookEx |
| Global hotkey (Linux) | Pending | Right Alt via X11 |
| **Tests** | Pending | Audio capture, VAD, hotkey registration |
| **Phase Review** | Pending | |

### S2: STT Integration (Planned)

| Task | Status | Description |
|------|--------|-------------|
| Groq API integration | Pending | POST audio to Whisper API |
| OpenAI Whisper API | Pending | Standard whisper-1 endpoint |
| Deepgram API | Pending | Deepgram streaming API |
| Custom endpoint | Pending | Generic OpenAI-compatible audio endpoint |
| Provider fallback logic | Pending | Auto-switch on failure |
| Front-end transcript display | Pending | Real-time transcript in capsule |
| **Tests** | Pending | Mock API responses, error handling |
| **Phase Review** | Pending | |

### S3: Keyboard Output (Planned)

| Task | Status | Description |
|------|--------|-------------|
| enigo keyboard sim (macOS) | Pending | Unicode text input via CGEvent |
| enigo keyboard sim (Windows) | Pending | SendInput with UTF-16 |
| enigo keyboard sim (Linux) | Pending | X11 key events |
| Clipboard paste fallback | Pending | Cmd+V / Ctrl+V with clipboard restore |
| IME detection | Pending | Detect active IME per platform |
| IME bypass strategy | Pending | Force clipboard paste when IME active |
| **Tests** | Pending | Output accuracy, clipboard preservation |
| **Phase Review** | Pending | |

### S4: Capsule UI (Planned)

| Task | Status | Description |
|------|--------|-------------|
| Floating window management | Pending | Always-on-top, no decorations position follow |
| Recording indicator | Pending | Animated waveform + status badges |
| Real-time transcript | Pending | Streaming text display during recording |
| Error states | Pending | Permission denied, API error, timeout |
| System tray integration | Pending | Menu bar icon with quick actions |
| Dark/light theme | Pending | System theme detection + manual toggle |
| **Tests** | Pending | Component rendering, state transitions |
| **Phase Review** | Pending | |

### S5: LLM Polish (Planned)

| Task | Status | Description |
|------|--------|-------------|
| OpenAI-compatible API | Pending | Chat completions with system prompt |
| Cleanup mode | Pending | Remove fillers, fix punctuation |
| Translation mode | Pending | Detect source → translate to target |
| Format mode | Pending | Lists, email structure |
| Custom prompt support | Pending | User-defined polish instructions |
| Streaming output | Pending | Stream polish results to capsule |
| **Tests** | Pending | Prompt quality, translation accuracy |
| **Phase Review** | Pending | |

### S6: Settings + History (Planned)

| Task | Status | Description |
|------|--------|-------------|
| Settings persistence | Pending | tauri-plugin-store, JSON config file |
| API key storage | Pending | Platform keychain via tauri-plugin-keyring |
| Hotkey customization | Pending | Visual key picker component |
| History SQLite store | Pending | Full-text search, pagination |
| History UI | Pending | Search, filter, delete, copy |
| **Tests** | Pending | Config persistence, keychain, SQLite |
| **Phase Review** | Pending | |

### S7: Local Whisper + Release (Planned)

| Task | Status | Description |
|------|--------|-------------|
| whisper.cpp integration | Pending | Subprocess management, model download |
| Model manager UI | Pending | Download progress, model switching |
| Offline mode | Pending | Fully offline with local models |
| macOS code signing | Pending | Developer certificate, notarization |
| Windows installer | Pending | MSI/NSIS installer |
| Linux AppImage | Pending | AppImage packaging |
| Auto-update | Pending | GitHub Releases integration |
| DMG/Homebrew | Pending | macOS distribution |
| **Tests** | Pending | E2E tests, install verification |
| **Phase Review** | Pending | |
