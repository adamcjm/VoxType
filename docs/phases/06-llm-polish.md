# Phase S5: LLM Polish

## Date
2026-05-06

## Goal
Enhance the LLM text polishing pipeline with better prompts, retry logic, token tracking, and frontend integration.

## Deliverables

### 1. Enhanced Prompts (`llm/prompts.rs`)
- [x] CLEANUP_PROMPT: comprehensive filler word removal (EN/CN/JA), homophone fix, punctuation
- [x] CASUAL_PROMPT: preserves informal tone, slang, abbreviations
- [x] EMAIL_PROMPT: professional structure with greeting/body/closing
- [x] CODE_PROMPT: preserves technical terms, identifiers, URLs
- [x] FORMAT_PROMPT: bullet points, numbered lists, email structure
- [x] translate_prompt(): dynamic source/target language prompt
- [x] All prompts output Chinese punctuation (。，！？、) for CJK text
- [x] Filler word lists for English, Chinese, Japanese

### 2. API Retry Logic (`llm/openai_compat.rs`)
- [x] Exponential backoff: 800ms, 1600ms (max 2 retries)
- [x] No retry on auth errors (401/403)
- [x] No retry on rate limits (429)
- [x] Retry on transient errors (500-599)
- [x] User-friendly error messages per status code

### 3. Token Usage Tracking
- [x] Log prompt_tokens, completion_tokens, total_tokens per request
- [x] Integrated into tracing/logging

### 4. Polish Modes (existing, validated)
- [x] Cleanup: remove fillers + fix homophones + add punctuation
- [x] Translate: source → target language
- [x] Format: structured output (bullets, numbers, email)

### 5. Frontend Integration
- [x] Settings panel: load/save config via `invoke("get_settings")/save_settings`
- [x] STT provider config: provider, API key, model
- [x] LLM provider config: provider (OpenAI/DeepSeek/Groq/Gemini/Ollama/Custom), API key, model, temperature
- [x] Custom prompt textarea for override instructions
- [x] Translation settings: source/target language selectors
- [x] Save button with backend persistence

### 6. Test Coverage
- [x] `test_polish_success` — happy path with token tracking
- [x] `test_polish_auth_error` — 401 error handling
- [x] `test_polish_empty_text` — empty input no-op
- [x] `test_no_api_key` — missing API key error
- [x] `test_translate_mode` — translate mode with wiremock
- [x] **Total: 25 tests, 25 passed, 0 failed** (+5 from S4)

## Build Results

| Artifact | Status |
|----------|--------|
| Rust compile | 0 errors, 0 warnings |
| Frontend | Build OK |
| Tests | 25/25 passed |
