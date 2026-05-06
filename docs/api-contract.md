# API Contract: Rust Backend ↔ React Frontend

Communication via Tauri v2 IPC (invoke/events).

## Tauri Commands (Rust → exposed to frontend)

### Recording

```typescript
// Start recording from default microphone
invoke("start_recording"): Promise<void>

// Stop recording, transcribe, polish, output
invoke("stop_recording"): Promise<string> // returns final text
```

### Settings

```typescript
// Get current settings
invoke("get_settings"): Promise<AppConfig>

// Save settings (also persists to disk)
invoke("save_settings", { config: AppConfig }): Promise<void>
```

### History

```typescript
interface HistoryItem {
  id: string;
  raw_text: string;
  final_text: string;
  stt_provider: string;
  llm_provider: string;
  app_name: string;
  duration_ms: number;
  mode: "cleanup" | "translate" | "format";
  created_at: string; // ISO 8601
}

invoke("get_history"): Promise<HistoryItem[]>
invoke("add_history_item", { item: HistoryItem }): Promise<void>
invoke("remove_history_item", { id: string }): Promise<void>
```

### Clipboard

```typescript
invoke("copy_to_clipboard", { text: string }): Promise<void>
invoke("paste_text", { text: string }): Promise<void>
```

### Audio Devices (planned)

```typescript
interface AudioDevice {
  name: string;
  is_default: boolean;
}

invoke("list_audio_devices"): Promise<AudioDevice[]>
invoke("set_audio_device", { device_name: string }): Promise<void>
```

### Models (planned)

```typescript
invoke("download_whisper_model", { size: "small" | "medium" | "large" }): Promise<void>
invoke("get_whisper_model_status"): Promise<{ size: string; downloaded: boolean }>
```

## Tauri Events (Rust → Frontend)

### Recording State

```typescript
enum RecordingState {
  Idle = "recording:idle",
  Recording = "recording:recording",
  Transcribing = "recording:transcribing",
  Polishing = "recording:polishing",
  Done = "recording:done",
  Error = "recording:error",
}

// Listen for state changes
listen("recording:state", (event: Event<RecordingState>) => {...})
```

### Transcript Progress

```typescript
listen("recording:transcript", (event: Event<{ text: string }>) => {...})
listen("recording:polished", (event: Event<{ text: string }>) => {...})
```

### Error Events

```typescript
listen("recording:error", (event: Event<{ code: string; message: string }>) => {...})
```

## Config Types (shared between frontend and backend)

```typescript
interface AppConfig {
  stt: SttConfig;
  llm: LlmConfig;
  hotkey: HotkeyConfig;
  translate: TranslateConfig;
  theme: string;
}

interface SttConfig {
  provider: "groq" | "openai" | "deepgram" | "local" | "custom";
  base_url: string;        // API endpoint
  api_key: string;         // Stored in keychain
  model: string;           // e.g. "whisper-large-v3-turbo"
  language: string;        // Language hint (zh, en, ja, etc.)
}

interface LlmConfig {
  provider: "openai" | "deepseek" | "anthropic" | "gemini" | "groq" | "ollama" | "custom";
  base_url: string;
  api_key: string;
  model: string;
  temperature: number;     // 0.0 - 2.0
  max_tokens: number;
  custom_prompt: string | null;
}

interface HotkeyConfig {
  macos: string;           // e.g. "Fn"
  other: string;           // e.g. "RightAlt"
}

interface TranslateConfig {
  enabled: boolean;
  source_lang: string;     // "auto" or language code
  target_lang: string;     // e.g. "en", "zh"
}
```

## Error Codes

| Code | Description | User Message |
|------|-------------|-------------|
| `audio:device_not_found` | No microphone detected | "No microphone found. Please connect a microphone." |
| `audio:permission_denied` | Mic access denied | "Microphone access denied. Check system permissions." |
| `stt:api_error` | STT API returned error | "Speech recognition failed. Check your API key." |
| `stt:timeout` | STT request timed out | "Speech recognition timed out. Try again." |
| `stt:model_not_found` | Local model missing | "Whisper model not downloaded. Go to Settings." |
| `llm:api_error` | LLM API returned error | "Text polishing failed. Check your API key." |
| `llm:quota_exceeded` | API quota exceeded | "API quota exceeded. Try another provider." |
| `output:permission_denied` | Accessibility not granted | "Grant Accessibility permission in System Settings." |
| `hotkey:conflict` | Hotkey already in use | "Hotkey conflict. Choose another key." |
| `unknown` | Unknown error | "Something went wrong. Please try again." |
