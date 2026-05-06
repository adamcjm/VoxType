# Phase S4: Capsule UI

## Date
2026-05-06

## Goal
Implement the floating capsule overlay — the primary user interaction surface during dictation.

## Deliverables

### 1. Capsule Component (`Capsule.tsx`)
- [x] Full state-driven animation orchestration (Framer Motion)
- [x] Tauri window auto show/hide (getCurrentWindow API)
- [x] Auto-dismiss after 2s on completion
- [x] Click-to-dismiss on done/error
- [x] `data-tauri-drag-region` for native drag support
- [x] Glassmorphism: backdrop-blur-2xl, translucent background, shadow
- [x] State-based border accent colors
- [x] Duration counter with monospace tabular-nums

### 2. Waveform Component (`Waveform.tsx`)
- [x] Animated audio bars during recording
- [x] Configurable bar count (default 5)
- [x] Staggered height/opacity animation with varied delays
- [x] Framer Motion transitions

### 3. StatusBadge Component (`StatusBadge.tsx`)
- [x] Color-coded dot per state (red=recording, amber=processing, green=done, etc.)
- [x] Pulse-glow animation during active recording
- [x] Shadow glow effect via CSS shadow-color

### 4. State Visualization
- [x] Recording: red badge + waveform + timer
- [x] Transcribing: amber badge + spinner
- [x] Polishing: brand badge + 3-dot shimmer
- [x] Done: green badge + "Text pasted" message → auto-dismiss
- [x] Error: red badge + error card with red background

### 5. Tauri Integration
- [x] Window show/hide via `getCurrentWindow()`
- [x] Always-on-top for capsule window
- [x] `useRecording` hook with Tauri `invoke` + `listen`
- [x] Event listeners for real-time transcript streaming (future)

### 6. App Entry Restructure
- [x] `App.tsx` simplified: Capsule (always mounted) + conditional Settings/History
- [x] Capsule self-manages visibility via recording state

## Build Results

| Artifact | Status |
|----------|--------|
| Frontend (tsc + vite) | CSS 25KB + JS 347KB, build OK |
| Rust compile | 0 errors, 0 warnings |
| Rust tests | 20/20 passed |
