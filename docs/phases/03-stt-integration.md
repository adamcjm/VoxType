# Phase S2: STT Integration

## Date
2026-05-06

## Goal
Implement real HTTP API calls for all Speech-to-Text providers.

## Deliverables

### 1. Shared HTTP Helper (`stt/common.rs`)
- [x] `transcribe_openai_compat()` - Generic OpenAI-compatible Whisper API call
- [x] Multipart/form-data POST to `/audio/transcriptions`
- [x] Proper error handling: HTTP errors, JSON parse errors, missing fields
- [x] Language parameter support
- [x] Configurable model name

### 2. Groq Provider (`stt/groq.rs`)
- [x] Uses OpenAI-compatible format via shared helper
- [x] Default base URL: `https://api.groq.com/openai/v1`
- [x] API key validation before request

### 3. OpenAI Whisper Provider (`stt/openai_whisper.rs`)
- [x] Uses OpenAI-compatible format via shared helper
- [x] Also serves as Custom endpoint provider (different base_url)
- [x] `new_custom()` constructor for Custom provider type

### 4. Deepgram Provider (`stt/deepgram.rs`)
- [x] Native Deepgram API format (POST audio body, different auth)
- [x] Dynamic URL building with query params (model, language, smart_format)
- [x] Parses nested JSON response: `results.channels[0].alternatives[0].transcript`
- [x] Token-based auth header (`Token {api_key}`)

### 5. Provider Trait Updates
- [x] `AudioFormat` now has `mime_type()` and `extension()` methods
- [x] All providers share `reqwest::Client` for connection reuse

### 6. Test Coverage
- [x] Mock HTTP server tests via `wiremock`
- [x] `test_transcribe_openai_compat_success` - happy path
- [x] `test_transcribe_openai_compat_auth_error` - 401 error handling
- [x] `test_transcribe_openai_compat_with_language` - language parameter
- [x] **Total: 17 tests, 17 passed, 0 failed** (+3 from S1)

## API Formats

### OpenAI-compatible (Groq, OpenAI, Custom)
```
POST {base_url}/audio/transcriptions
Authorization: Bearer {api_key}
Content-Type: multipart/form-data

file: <wav_audio>
model: "{model}"
response_format: "json"
language: "{lang}" (optional)

Response: {"text": "transcription"}
```

### Deepgram
```
POST {base_url}/listen?model=nova-2&language=zh&smart_format=true
Authorization: Token {api_key}
Content-Type: audio/wav

<raw_audio_bytes>

Response: {"results": {"channels": [{"alternatives": [{"transcript": "text"}]}]}}
```

## Build Results

| Artifact | Status |
|----------|--------|
| Rust compile | 0 errors, 0 warnings |
| Tests | 17/17 passed |
