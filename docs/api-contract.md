# API Contract: Rust Backend ↔ React Frontend

## Tauri Commands

### Recording

```typescript
invoke("start_recording"): Promise<void>
invoke("stop_recording"): Promise<string>  // returns final text
invoke("toggle_recording"): Promise<string> // returns "recording" or final text
invoke("list_audio_devices"): Promise<AudioDeviceInfo[]>
```

### Settings

```typescript
invoke("get_settings"): Promise<AppConfig>
invoke("save_settings", { config: AppConfig }): Promise<void>
invoke("needs_onboarding"): Promise<boolean>
```

### History

```typescript
interface HistoryItem {
  id: string; raw_text: string; final_text: string;
  stt_provider: string; llm_provider: string; app_name: string;
  duration_ms: number; mode: string; created_at: string;
}

invoke("get_history", { search?: string, limit?: number }): Promise<HistoryItem[]>
invoke("add_history_item", { item: HistoryItem }): Promise<void>
invoke("remove_history_item", { id: string }): Promise<void>
```

### Model Fetching (NEW)

```typescript
interface ModelInfo {
  id: string;
  display_name: string;
}

// Fetch available models from STT provider
invoke("fetch_stt_models", {
  provider: string,    // "groq" | "openai" | "deepgram" | "local" | "custom"
  baseUrl: string,     // API endpoint
  apiKey: string       // Provider API key
}): Promise<ModelInfo[]>

// Fetch available models from LLM provider
invoke("fetch_llm_models", {
  baseUrl: string,
  apiKey: string
}): Promise<ModelInfo[]>
```

### Model Manager (Whisper)

```typescript
invoke("list_models"): Promise<ModelStatus[]>
invoke("download_model", { sizeStr: string }): Promise<string>
invoke("delete_model", { sizeStr: string }): Promise<void>
invoke("model_exists", { sizeStr: string }): Promise<boolean>
```

## Tauri Events (Rust → Frontend)

```typescript
// Fn key toggle (macOS CGEventTap)
listen("hotkey:toggle", () => { /* toggle recording */ })

// Recording state changes (future)
listen("recording:state", (event: Event<State>) => {})
listen("recording:transcript", (event: Event<{ text: string }>) => {})
```

## Config Types

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
  base_url: string;
  api_key: string;
  model: string;
  language: string;
}

interface LlmConfig {
  provider: "openai" | "deepseek" | "anthropic" | "gemini" | "groq" | "ollama" | "custom";
  base_url: string;
  api_key: string;
  model: string;
  temperature: number;
  max_tokens: number;
  custom_prompt: string | null;
}

interface HotkeyConfig {
  macos: string;
  other: string;
}

interface TranslateConfig {
  enabled: boolean;
  source_lang: string;
  target_lang: string;
}
```

## Error Codes

| Code | User Message |
|------|-------------|
| `audio:device_not_found` | "No microphone found" |
| `audio:permission_denied` | "Microphone access denied" |
| `stt:api_error` | "Speech recognition failed. Check API key" |
| `stt:model_not_found` | "Whisper model not downloaded" |
| `llm:api_error` | "Text polishing failed. Check API key" |
| `output:permission_denied` | "Grant Accessibility permission" |
| `hotkey:conflict` | "Hotkey already in use" |
