# Phase S1: Audio Capture + Hotkey

## Date
2026-05-06

## Goal
Implement real microphone audio capture with VAD, preprocessing, and hotkey support infrastructure.

## Deliverables

### 1. Audio Capture (cpal)
- [x] `AudioCapture` struct with dedicated audio thread
- [x] cpal-based microphone capture at 16kHz mono PCM
- [x] Start/stop recording with channel-based command system
- [x] Thread-safe design: cpal Stream (not Send on macOS) lives on dedicated thread
- [x] WAV encoding from f32 samples to i16 PCM
- [x] Duration calculation from sample count

### 2. Voice Activity Detection (VAD)
- [x] Energy-based (RMS) speech detection
- [x] Configurable speech threshold (default 0.02)
- [x] Frame-based analysis with segment detection
- [x] Min speech/silence duration filtering

### 3. Audio Preprocessing
- [x] Noise gate: zero out samples below amplitude threshold
- [x] Gain normalization: amplify quiet audio to target peak
- [x] Pipeline: noise gate → gain normalization

### 4. Device Enumeration
- [x] `list_input_devices()` - list all available microphones
- [x] `AudioDeviceInfo` struct with name, default flag, channel count
- [x] Exposed as Tauri command: `list_audio_devices`

### 5. Hotkey Infrastructure
- [x] Platform-specific hotkey descriptions (macOS: Fn, Win/Linux: Right Alt)
- [x] `hotkey_description()` and `hotkey_keycode()` utilities
- [x] Implementation strategy documented for future OS-level hotkey

### 6. Pipeline Integration
- [x] `run_pipeline()` free function: STT → LLM → Output
- [x] No MutexGuard held across await points (Send-safe futures)
- [x] VAD gate before STT (avoid wasting API calls on silence)
- [x] Audio preprocessing before STT (improve accuracy)

### 7. State Management
- [x] `AppState` simplified: config + audio_capture only
- [x] Removed pipeline from state (replaced by free function)
- [x] All Tauri commands use `state.inner()` pattern
- [x] All async futures are Send-safe

### 8. Test Coverage
- [x] `audio/capture.rs`: 3 tests (WAV encoding, clipping, device detection)
- [x] `audio/vad.rs`: 5 tests (silence, speech, empty, segments, RMS)
- [x] `audio/preprocess.rs`: 4 tests (noise gate, gain, empty, no-change)
- [x] `hotkey/mod.rs`: 2 tests (description, keycode)
- [x] **Total: 14 tests, 14 passed, 0 failed**

## Architecture Decisions

### Audio Thread Isolation
**Problem**: cpal's `Stream` is not `Send` on macOS (CoreAudio backend uses `PhantomData<*mut ()>`).
**Solution**: Dedicated audio thread that owns the Stream, communicating via `mpsc` channels and `Arc<Mutex<Vec<f32>>>` for audio data.

### Pipeline as Free Function
**Problem**: Holding `MutexGuard` across `.await` makes futures non-Send in Tauri commands.
**Solution**: `run_pipeline()` is a free function that takes references; no state mutation needed.

### Hotkey Strategy
**Problem**: Truly global hotkeys require platform-specific APIs (CGEventTap, SetWindowsHookEx, XGrabKey) and accessibility permissions.
**Solution**: For MVP, hotkey handling will use Tauri window keyboard events in the capsule window. OS-level global hotkey is deferred to a future phase.

## Build Results

| Artifact | Status |
|----------|--------|
| Rust compile | 0 errors, 0 warnings |
| Frontend build | CSS 18KB + JS 203KB |
| Tests | 14/14 passed |
