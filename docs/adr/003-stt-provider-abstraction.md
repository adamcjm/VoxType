# ADR-003: STT Provider Abstraction

## Status

Accepted

## Context

VoxType needs to support multiple speech-to-text backends:
1. Cloud APIs: Groq, OpenAI Whisper, Deepgram
2. Local: whisper.cpp via subprocess
3. Custom: Any OpenAI-compatible audio endpoint

Users should be able to configure providers without code changes.

## Decision

**Use the Strategy pattern with an `SttProvider` trait and `async-trait` for async support.**

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

All providers implement this trait. The `SttManager` selects the active provider based on user config:

```rust
let provider: Box<dyn SttProvider> = match config.provider {
    SttProvider::Groq => Box::new(GroqProvider::new(config)),
    SttProvider::OpenAi => Box::new(OpenAiWhisperProvider::new(config)),
    SttProvider::Deepgram => Box::new(DeepgramProvider::new(config)),
    SttProvider::Local => Box::new(LocalWhisperProvider::new(config)),
    SttProvider::Custom => Box::new(OpenAiWhisperProvider::new_custom(config)),
};
```

## Cloud API Interface

All cloud providers use HTTP POST with multipart/form-data:

```
POST {base_url}/audio/transcriptions
Authorization: Bearer {api_key}
Content-Type: multipart/form-data

file: <wav_audio>
model: "whisper-large-v3-turbo"
language: "zh" (optional)
response_format: "json"
```

The `OpenAiWhisperProvider` handles both OpenAI's endpoint and custom endpoints by accepting a configurable `base_url`.

## Local Whisper

Local whisper runs `whisper.cpp` as a subprocess:
```
whisper -m models/ggml-small.bin -l zh -f input.wav -otxt output.txt
```

## Fallback Strategy

If the primary STT provider fails:
1. Try the same provider again (network retry, max 2 attempts)
2. Fall back to local Whisper if available
3. Show user error if none available

## Consequences

- All providers accept `Vec<u8>` WAV audio for simplicity
- Adding a new provider requires implementing the `SttProvider` trait
- `async-trait` adds a minor allocation overhead (acceptable for STT calls)
