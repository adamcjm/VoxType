# ADR-001: Choose Tauri + Rust over Electron and Wails

## Status

Accepted

## Context

We need a cross-platform desktop framework for VoxType (macOS/Windows/Linux). The core requirements are:
1. Low-level system access (audio capture, global hotkeys, keyboard simulation)
2. Convenient UI development (settings panel, capsule overlay, history browser)
3. Cross-platform deployment with reasonable binary size
4. Strong ecosystem for STT and LLM integrations

## Options Considered

### Option A: Tauri v2 + Rust + React

- **Pros**:
  - Rust provides direct access to OS APIs (CoreAudio, WASAPI, X11)
  - Rich ecosystem: cpal (audio), enigo (keyboard), reqwest (HTTP), arboard (clipboard)
  - Small binary size (~5-15MB), low memory footprint
  - Tauri's IPC automatically bridges Rust ↔ JS
  - Strong community with growing adoption
  - Excellent reference implementation: tover0314-w/opentypeless (⭐175, 83 commits)
  - TypeScript frontend with modern tooling (Vite, React 19, Tailwind)

- **Cons**:
  - Rust learning curve if contributors are new to it
  - Longer compile times than Go or JS
  - Some platform-specific hacks needed for audio/keyboard

### Option B: Electron + Node.js

- **Pros**:
  - Largest ecosystem, most tutorials
  - Quick to prototype
  - Many audio/keyboard npm packages

- **Cons**:
  - Large binary size (~120MB+)
  - High memory usage (~200MB+)
  - No native OS API access without native addons
  - Audio capture quality limited in JS
  - Generally considered poor UX for background utility apps
  - Poor performance for always-on apps

### Option C: Wails + Go

- **Pros**:
  - Go's simple concurrency model (goroutines)
  - Fast compile times
  - Good standard library

- **Cons**:
  - Weaker ecosystem for audio/keyboard than Rust
  - `gohook` (hotkey library) poorly maintained
  - Smaller community, fewer references
  - No existing OpenTypeless implementation to reference

## Decision

**Chosen: Tauri v2 + Rust + React**

Rust with Tauri v2 provides the best balance of system-level capability, cross-platform support, binary size, and ecosystem maturity.

Key deciding factors:
1. `tover0314-w/opentypeless` provides a proven reference implementation
2. `cpal`, `enigo`, `reqwest`, `arboard` cover all system integration needs
3. React + Tailwind CSS provides a modern, maintainable frontend
4. Tauri v2's IPC model is clean and well-documented

## Consequences

### Positive
- Clean separation between UI (React) and system logic (Rust)
- Small binary size for distribution
- Strong type safety on both sides (TypeScript + Rust)
- Can leverage existing OpenTypeless codebase as reference

### Negative
- Contributors need Rust knowledge for backend changes
- C++ interop for whisper.cpp adds complexity
- Need platform-specific hotkey handling (Fn key on macOS is non-trivial)

### Mitigations
- Keep Rust API surface minimal and well-documented
- Use `async-trait` and clear interfaces to reduce complexity
- Provide development scripts and comprehensive onboarding docs
