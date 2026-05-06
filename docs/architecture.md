# VoxType Architecture

## 1. System Overview

```
┌──────────────────────────────────────────────────────────────────┐
│                         VoxType Desktop App                       │
│                                                                   │
│  ┌──────────────────────────┐   ┌───────────────────────────────┐│
│  │    React/TS Frontend     │   │        Rust Backend            ││
│  │                          │   │                               ││
│  │  Floating Capsule        │◄──┤  Hotkey Manager               ││
│  │  Settings Panel          │   │  Audio Pipeline                ││
│  │  History Panel           │   │  STT Providers                 ││
│  │  System Tray Menu        │   │  LLM Providers                 ││
│  │                          │   │  Output Manager                ││
│  │                          │   │  Config / History Store        ││
│  └──────────────────────────┘   └───────────────────────────────┘│
│                                                                   │
│  Window 1: "capsule" - Always-on-top, no decorations, transparent │
│  Window 2: "main"    - Settings & history, standard window        │
└──────────────────────────────────────────────────────────────────┘
```

## 2. Data Flow Pipeline

```
User presses hotkey (Fn / Right Alt)
  │
  ▼
┌──────────────┐
│ Audio Capture │  cpal microphone stream → WAV buffer
└──────┬───────┘
       │
  ┌────▼───────┐
  │    VAD      │  Voice Activity Detection → skip silence
  └────┬───────┘
       │
  ┌────▼───────┐
  │ Preprocess  │  Noise reduction, gain normalization
  └────┬───────┘
       │
  ┌────▼───────┐
  │ STT Provider│  Local Whisper or Cloud API (Groq/OpenAI/Deepgram)
  └────┬───────┘
       │ raw text
  ┌────▼───────┐
  │ LLM Provider│  Polish: remove fillers, fix punctuation, translate
  └────┬───────┘
       │ polished text
  ┌────▼───────┐
  │   Output    │  Clipboard paste (CMD+V) with clipboard restore
  └────┬───────┘
       │
  ┌────▼───────┐
  │   History   │  Save to SQLite
  └────────────┘
```

## 3. Module Architecture

### 3.1 Rust Backend (`src-tauri/src/`)

| Module | Path | Responsibility |
|--------|------|---------------|
| `audio` | `audio/` | Microphone capture, VAD, preprocessing |
| `stt` | `stt/` | STT provider abstraction and implementations |
| `llm` | `llm/` | LLM provider abstraction and prompt management |
| `output` | `output/` | Keyboard simulation, clipboard management, IME bypass |
| `pipeline` | `pipeline/` | End-to-end processing orchestration |
| `hotkey` | `hotkey/` | Global hotkey registration per platform |
| `config` | `config/` | Configuration types and defaults |
| `history` | `history/` | SQLite-based transcription history |
| `commands` | `commands/` | Tauri command handlers (Rust ↔ JS bridge) |

### 3.2 React Frontend (`src/`)

| Module | Path | Responsibility |
|--------|------|---------------|
| `capsule` | `components/capsule/` | Floating recording indicator with transcript preview |
| `settings` | `components/settings/` | Full settings panel with STT/LLM/hotkey config |
| `history` | `components/history/` | Past transcriptions list with search |
| `common` | `components/common/` | Shared UI components (Toast, Dialog) |
| `stores` | `stores/` | Zustand state stores (recording, settings, history) |
| `hooks` | `hooks/` | React hooks for Tauri IPC |

## 4. Provider Abstraction

### 4.1 STT Provider Interface

```rust
#[async_trait]
pub trait SttProvider: Send + Sync {
    fn name(&self) -> &str;
    fn is_available(&self) -> bool;
    fn supported_languages(&self) -> Vec<&str>;
    async fn transcribe(
        &self,
        audio_data: &[u8],
        format: AudioFormat,
        language: Option<&str>,
    ) -> Result<String>;
}
```

**Implementations:**
- `GroqProvider` - Groq Whisper API (free tier available)
- `OpenAiWhisperProvider` - OpenAI Whisper API + custom endpoints
- `DeepgramProvider` - Deepgram API
- `LocalWhisperProvider` - Local whisper.cpp via subprocess

### 4.2 LLM Provider Interface

```rust
#[async_trait]
pub trait LlmProvider: Send + Sync {
    fn name(&self) -> &str;
    fn is_available(&self) -> bool;
    async fn polish(&self, text: &str, mode: PolishMode) -> Result<String>;
}
```

All providers use OpenAI-compatible chat completions API. The `polish` method sends a system prompt for text cleanup and returns the polished result.

## 5. Platform-Specific Design

| Feature | macOS | Windows | Linux |
|---------|-------|---------|-------|
| Default Hotkey | Fn | Right Alt | Right Alt |
| Keyboard Output | CGEventPost | SendInput | X11/xdotool |
| Clipboard | arboard | arboard | arboard |
| Audio | cpal (CoreAudio) | cpal (WASAPI) | cpal (PulseAudio/PipeWire) |
| Keychain Storage | Security framework | Credential Manager | Secret Service |
| Config Path | `~/.config/voxtype/` | `%APPDATA%/VoxType/` | `~/.config/voxtype/` |

## 6. Security Design

- API keys stored in platform keychain (not plain text)
- Local STT runs entirely offline on device
- All cloud requests go directly from user machine to provider APIs
- No VoxType servers involved in BYOK mode
- Clipboard is saved and restored during paste operations
- Application permissions clearly scoped (microphone, accessibility, keyboard)
