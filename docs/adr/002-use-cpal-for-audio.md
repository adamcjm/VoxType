# ADR-002: Using cpal for Audio Capture

## Status

Accepted

## Context

VoxType needs to capture audio from the system's default microphone across macOS, Windows, and Linux. Requirements:
1. Capture mono 16-bit PCM audio at 16kHz
2. Support device enumeration for multi-mic setups
3. Cross-platform (CoreAudio/WASAPI/ALSA/PulseAudio/PipeWire)
4. Low latency for real-time VAD

## Options Considered

### cpal (Cross-Platform Audio Library)

- Pure Rust, maintained by the RustAudio community
- Backends: CoreAudio (macOS), WASAPI (Windows), ALSA/PulseAudio/JACK (Linux)
- Active development, good documentation
- Used by Spotify, Discord audio libraries in Rust ecosystem

### Alternatives

| Library | Language | Cross-platform | Note |
|---------|----------|--------------|------|
| cpal | Rust | Yes | Chosen |
| portaudio-rs | Rust (bindings) | Yes | Requires portaudio C lib, extra dependency |
| rodio | Rust | Yes | Playback-focused, limited capture |
| cpal + symphonia | Rust | Yes | Overkill (we only need PCM capture) |

## Decision

**Chosen: cpal v0.15**

cpal is the de-facto standard for audio capture in Rust. It provides native backends without external C dependencies and supports all required platforms.

## Implementation

```rust
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

// Get default input device
let host = cpal::default_host();
let device = host.default_input_device()
    .expect("No input device found");

// Configure: 16kHz, mono, f32 samples
let config = cpal::StreamConfig {
    channels: 1,
    sample_rate: cpal::SampleRate(16000),
    buffer_size: cpal::BufferSize::Default,
};

// Build stream with audio callback
let stream = device.build_input_stream(
    &config.into(),
    move |data: &[f32], _: &cpal::InputCallbackInfo| {
        // Buffer samples for STT
    },
    |err| tracing::error!("Audio error: {}", err),
    None,
)?;
```

## Consequences

- Audio is captured as `f32` PCM; needs conversion to `i16` WAV for STT APIs
- Buffer size affects latency; 256-1024 samples recommended
- Permission handling differs per platform
