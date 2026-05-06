# Phase S7: Local Whisper + Release

## Date
2026-05-06

## Goal
Implement offline speech recognition via whisper.cpp, model download management, and release packaging configuration.

## Deliverables

### 1. Local Whisper Provider (`stt/local.rs`)
- [x] whisper.cpp subprocess invocation
- [x] Auto-detect whisper binary (bundled or $PATH)
- [x] Auto-detect downloaded model files
- [x] Write audio to temp file, run whisper, parse output
- [x] `spawn_blocking` for CPU-intensive transcription
- [x] Temp file cleanup after transcription
- [x] Supports all languages via `-l` flag

### 2. Model Manager (`model_manager/mod.rs`)
- [x] 5 model sizes: tiny (78MB), base (148MB), small (466MB), medium (1.5GB), large (3.1GB)
- [x] Download from HuggingFace (ggerganov/whisper.cpp) with progress logging
- [x] Stream-based download with progress tracking
- [x] Atomic file rename after download completes
- [x] Delete existing model files
- [x] Query model existence and file size

### 3. Model Commands (`commands/model.rs`)
- [x] `list_models` — enumerate with download status
- [x] `download_model` — async download with progress
- [x] `delete_model` — remove downloaded model
- [x] `model_exists` — check if specific model is present

### 4. Bundle Configuration (`tauri.conf.json`)
- [x] macOS: minimumSystemVersion 13.0, standard entitlements
- [x] Updater: GitHub Releases endpoint
- [x] Icon set: PNG, ICNS, ICO
- [x] Category: Utilities
- [x] Short/long description
- [x] Copyright: MIT
- [x] `createUpdaterArtifacts: true`

### 5. Platform Storage
Settings, history, and models all use a unified directory:

| Platform | Path |
|----------|------|
| macOS | `~/.VoxType/` |
| Windows | `~/.VoxType/` |
| Linux | `~/.VoxType/` |

## Build Results

| Artifact | Status |
|----------|--------|
| Rust compile | 0 errors, 0 warnings |
| Tests | 28/28 passed |
| Frontend | Build OK |

## Release Checklist

### Pre-release
- [ ] Test on physical microphone (macOS)
- [ ] Test on Windows VM
- [ ] Test on Linux (Ubuntu)
- [ ] Verify hotkey works on each platform
- [ ] Test with real STT API keys (Groq/Deepgram)
- [ ] Test with real LLM API keys (DeepSeek/OpenAI)
- [ ] Run `pnpm tauri build` for each platform
- [ ] Sign macOS build with Developer ID
- [ ] Notarize macOS build
- [ ] Create GitHub Release with platform artifacts
- [ ] Verify auto-updater endpoint

### Future
- [ ] OS-level global hotkey (CGEventTap/RegisterHotKey/XGrabKey)
- [ ] Bundled whisper.cpp binary in app resources
- [ ] Homebrew cask formula
- [ ] WinGet / Chocolatey packages
- [ ] Linux Flatpak/Snap packages
- [ ] Mobile companion app (iOS/Android)
