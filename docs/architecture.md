# VoxType Architecture

## System Overview

```
┌──────────────────────────────────────────────────────────────────┐
│                         VoxType Desktop App                       │
│                                                                   │
│  ┌──────────────────────────┐   ┌───────────────────────────────┐│
│  │    React/TS Frontend     │   │        Rust Backend            ││
│  │                          │   │                               ││
│  │  Capsule Window          │   │  Hotkey Manager               ││
│  │   → Recording indicator  │◄──┤   → CGEventTap (macOS Fn)     ││
│  │   → Waveform + timer     │   │  Audio Pipeline               ││
│  │   → Transcript preview   │   │   → cpal capture + VAD        ││
│  │                          │   │  STT Providers                ││
│  │  Main Window             │   │   → Groq/Deepgram/OpenAI/     ││
│  │   → Settings (sidebar)   │   │     Local/Custom              ││
│  │   → History list         │   │  LLM Providers                ││
│  │   → i18n (zh-CN/en-US)   │   │   → OpenAI-compat API         ││
│  │                          │   │  Output Manager               ││
│  │  System Tray             │   │   → Clipboard paste + restore  ││
│  │                          │   │  Config Store (~/.VoxType/)   ││
│  │                          │   │  History Store (SQLite)       ││
│  │                          │   │  Model Manager (Whisper DL)   ││
│  └──────────────────────────┘   └───────────────────────────────┘│
└──────────────────────────────────────────────────────────────────┘
```

## Window Architecture

Two separate webview windows with independent entry points:

| Window | Label | Entry | Content | Properties |
|--------|-------|-------|---------|------------|
| Capsule | `capsule` | `capsule.html` → `capsule.tsx` | Recording pill overlay | decorations:false, alwaysOnTop, transparent |
| Main | `main` | `index.html` → `App.tsx` | Settings + History + idle launcher | Standard window, hidden by default |

## Data Flow Pipeline

```
Fn key (keycode 63)
  │
  ├→ CGEventTap intercepts → consume event (block emoji) → emit "hotkey:toggle"
  │
  ▼
App.tsx receives event → toggle_recording command
  │
  ├─ if idle: AudioCapture.start() → microphone stream → buffer
  │   └─ Capsule shows: red dot + waveform + timer
  │
  └─ if recording: AudioCapture.stop() → get samples
       │
       ▼
     VAD check (has_speech?) ─ No → "No speech detected"
       │ Yes
       ▼
     Preprocess (noise gate + gain normalize)
       │
       ▼
     Encode to WAV (16kHz, mono, i16)
       │
       ▼
     STT Provider transcribe ─ Error → show error in Capsule
       │ OK
       ▼
     Raw transcript text
       │
       ▼
     LLM Provider polish (if API key configured)
       │
       ▼
     Final polished text
       │
       ▼
     Clipboard paste (set → Cmd+V → restore original)
       │
       ▼
     Save to History (SQLite)
       │
       ▼
     Capsule shows: ✓ "Pasted" → auto-dismiss 2s
```

## Provider Architecture

### STT Providers

All implement `SttProvider` trait:

```rust
#[async_trait]
pub trait SttProvider: Send + Sync {
    fn name(&self) -> &str;
    fn is_available(&self) -> bool;
    fn supported_languages(&self) -> Vec<&str>;
    async fn transcribe(&self, audio: &[u8], format: AudioFormat, lang: Option<&str>) -> Result<String>;
}
```

| Implementation | API Format | Endpoint |
|---------------|-----------|----------|
| GroqProvider | OpenAI-compat multipart/form-data | `/audio/transcriptions` |
| OpenAiWhisperProvider | OpenAI-compat multipart/form-data | `/audio/transcriptions` |
| DeepgramProvider | Raw audio body | `/listen?model=...` |
| LocalWhisperProvider | whisper.cpp subprocess | local binary |
| Custom (via OpenAiWhisperProvider) | OpenAI-compat | user-defined |

### LLM Providers

All use OpenAI-compatible chat completions via `OpenAiCompatProvider`:

```
POST {base_url}/chat/completions
Authorization: Bearer {api_key}
{ "model": "...", "messages": [system_prompt, user_text] }
```

**Prompt modes**: Cleanup, Translate, Format — with custom user prompt override.

**Retry logic**: Exponential backoff (2 retries), no retry on 401/403/429.

## Data Storage

```
~/.VoxType/
├── config.json     # AppConfig (STT/LLM provider, model, API key, hotkey)
├── history.db      # SQLite (id, raw_text, final_text, provider, mode, created_at)
└── models/          # Whisper models (ggml-small.bin, ggml-medium.bin, etc.)
```

**Config loading priority**: `.env` → `config.json` → code defaults

## IME Bypass Strategy

1. Primary: Clipboard paste (set text → Cmd+V → restore original) — bypasses IME entirely
2. Fallback: enigo keyboard simulation (character-by-character Unicode input)

## Performance

| Metric | Value |
|--------|-------|
| Memory (idle) | ~25MB |
| Memory (recording) | ~40MB |
| CPU (recording) | ~2% |
| STT latency (Groq) | ~200ms |
| LLM polish (DeepSeek) | ~500ms |
| End-to-end | ~1-2s |
