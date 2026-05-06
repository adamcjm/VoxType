# Phase S6: Settings + History

## Date
2026-05-06

## Goal
Implement persistent configuration storage and SQLite-based transcription history.

## Deliverables

### 1. Settings Persistence (`config/mod.rs`)
- [x] `AppConfig::load()` — load from JSON file on disk
- [x] `AppConfig::save()` — save to JSON file (pretty-printed)
- [x] Platform-specific config paths:
  - macOS: `~/Library/Application Support/com.voxtype.app/config.json`
  - Windows: `%APPDATA%/VoxType/config.json`
  - Linux: `$XDG_CONFIG_HOME/voxtype/config.json`
- [x] Auto-creates config directory on first save

### 2. History Storage (`history/mod.rs`)
- [x] SQLite via `rusqlite` (bundled, no system dependency)
- [x] Schema: id, raw_text, final_text, stt_provider, llm_provider, app_name, duration_ms, mode, created_at
- [x] Indexes on created_at DESC and mode
- [x] `add()` — insert new entry
- [x] `get_all()` — list with optional limit and full-text search (LIKE)
- [x] `remove()` — delete by ID
- [x] Platform-specific DB paths:
  - macOS: `~/Library/Application Support/com.voxtype.app/history.db`
  - Windows: `%APPDATA%/VoxType/history.db`
  - Linux: `$XDG_DATA_HOME/voxtype/history.db`

### 3. Pipeline Result (`pipeline/`)
- [x] `PipelineResult` struct: raw_text, final_text, stt_provider, llm_provider, mode
- [x] All pipeline output metadata captured for history

### 4. Recording Command Integration
- [x] `stop_recording` automatically saves history after pipeline completes
- [x] Uses `uuid::Uuid::v4()` for unique IDs
- [x] Uses `chrono::Utc::now().to_rfc3339()` for timestamps
- [x] Non-fatal history save failure (logged as warning)

### 5. Command Handlers
- [x] `save_settings` now persists to disk before updating memory
- [x] `get_settings` loads from disk on app start
- [x] `get_history` supports optional search and limit params
- [x] `remove_history_item` connected to real SQLite

### 6. Test Coverage
- [x] `test_add_and_get_history` — insert and retrieve
- [x] `test_search_history` — full-text LIKE search
- [x] `test_remove_history` — delete by ID
- [x] **Total: 28 tests, 28 passed, 0 failed** (+3 from S5)

## Build Results

| Artifact | Status |
|----------|--------|
| Rust compile | 0 errors, 0 warnings |
| Frontend | Build OK |
| Tests | 28/28 passed |
