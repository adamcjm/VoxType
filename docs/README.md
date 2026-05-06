# VoxType Documentation

VoxType is an open-source AI voice input tool for desktop — a Typeless alternative.  
Speak naturally, get polished text at your cursor in any app.

## Documents

| Document | Description |
|----------|-------------|
| [architecture.md](./architecture.md) | Overall system architecture and design decisions |
| [development-plan.md](./development-plan.md) | Phased development roadmap with milestones |
| [progress.md](./progress.md) | Real-time development progress tracking |
| [api-contract.md](./api-contract.md) | Rust backend ↔ React frontend API contract |
| [stt-provider-guide.md](./stt-provider-guide.md) | STT provider integration guide |
| [llm-provider-guide.md](./llm-provider-guide.md) | LLM provider integration guide |
| [release-checklist.md](./release-checklist.md) | Pre-release verification checklist |
| [contributing.md](./contributing.md) | Contribution guidelines |
| [changelog.md](./changelog.md) | Release changelog |

## Design

| Document | Description |
|----------|-------------|
| [design/ui-design.md](./design/ui-design.md) | Overall UI/UX design specification |
| [design/capsule-design.md](./design/capsule-design.md) | Floating capsule widget design |
| [design/color-tokens.md](./design/color-tokens.md) | Design tokens and color system |

## Architecture Decision Records

| # | Title |
|---|-------|
| [ADR-001](./adr/001-choose-tauri-rust.md) | Choosing Tauri + Rust for the desktop framework |
| [ADR-002](./adr/002-use-cpal-for-audio.md) | Using cpal for audio capture |
| [ADR-003](./adr/003-stt-provider-abstraction.md) | STT provider abstraction design |
| [ADR-004](./adr/004-ime-bypass-strategy.md) | IME input method bypass strategy |
| [ADR-005](./adr/005-clipboard-fallback.md) | Clipboard paste as fallback output method |

## Phase Reports

| Phase | Document |
|-------|----------|
| S1: Scaffold | [phases/01-scaffold.md](./phases/01-scaffold.md) |
| S2: Audio + Hotkey | [phases/02-audio-hotkey.md](./phases/02-audio-hotkey.md) |
| S3: STT Integration | [phases/03-stt-integration.md](./phases/03-stt-integration.md) |
| S4: Keyboard Output | [phases/04-keyboard-output.md](./phases/04-keyboard-output.md) |
| S5: Capsule UI | [phases/05-capsule-ui.md](./phases/05-capsule-ui.md) |
| S6: LLM Polish | [phases/06-llm-polish.md](./phases/06-llm-polish.md) |
| S7: Settings + History | [phases/07-settings-history.md](./phases/07-settings-history.md) |
| S8: Local Whisper + Release | [phases/08-local-whisper-packaging.md](./phases/08-local-whisper-packaging.md) |

## Tech Stack

- **Desktop Framework**: Tauri v2
- **Backend Language**: Rust (edition 2021)
- **Frontend**: React 19 + TypeScript 5.8 + Vite 7
- **Styling**: Tailwind CSS 4
- **State Management**: Zustand 5
- **Animations**: Framer Motion 12

## Quick Links

- [Installation](#) (Coming soon)
- [Contributing](./contributing.md)
- [License](../LICENSE)
