# Floating Capsule Design

## Purpose

The capsule is the primary user interaction surface during dictation. It provides:
1. Visual confirmation that recording is active
2. Real-time transcript preview
3. Status indication (recording/transcribing/polishing/done/error)
4. Non-intrusive presence via floating overlay

## Dimensions

```
┌──────────────────────────────────────┐
│  ● ● Recording...         0:12       │  ← Status bar (40px)
│                                      │
│  This is the real-time transcript    │  ← Content area (variable)
│  that appears as you speak...        │     Max ~160px scrollable
│                                      │
└──────────────────────────────────────┘
```

- **Min width**: 320px
- **Max width**: 480px
- **Min height**: 60px (status only)
- **Max height**: 200px (with transcript)
- **Border radius**: 20px
- **Padding**: 16px horizontal, 12px vertical

## Positioning

- **Default**: Top center of active screen (`top: 16px, left: 50%, transform: translateX(-50%)`)
- **Smart behavior**: Avoids menu bar (macOS) / taskbar area
- **Floating**: Renders above all other windows (`z-index: 9999`)
- **Repositioning**: When user moves to another monitor, capsule follows

## Visual States

### Idle
Hidden. No rendering.

### Recording
```
┌────────────────────────────────────────────┐
│  🔴 Recording         ▲ ●  ●  ●  ●  ●     │  ← Waveform bars
│                   0:15                      │
├────────────────────────────────────────────┤
│  The quick brown fox jumps over the lazy   │  ← Real-time STT text
│  dog and then continues to run through     │     fading in character by char
│  the forest without stopping...            │
└────────────────────────────────────────────┘
```

Features:
- Red pulsing dot with `animate-pulse-glow`
- Animated waveform bars using `animate-wave` with staggered delays
- Real-time transcript streaming (using Tauri events)
- Elapsed timer counting up

### Transcribing
```
┌────────────────────────────────────────────┐
│  ⏳ Transcribing...              │
│────────────────────────────────────────────│
│  (spinner animation)           │
└────────────────────────────────────────────┘
```

### Polishing
```
┌────────────────────────────────────────────┐
│  ✨ Polishing text...         │
│────────────────────────────────────────────│
│  The quick brown fox...                   │
│  ↓ (sparkle animation)                    │
│  The quick brown fox jumps over the lazy   │
│  dog.                                     │
└────────────────────────────────────────────┘
```

### Done (auto-dismiss in 2s)
```
┌────────────────────────────────────────────┐
│  ✅ Done                     │
└────────────────────────────────────────────┘
```

### Error
```
┌────────────────────────────────────────────┐
│  ⚠️ Microphone access denied  ✕           │
│────────────────────────────────────────────│
│  Please enable microphone access in        │
│  System Settings → Privacy.                │
│                                            │
│                    [Open Settings]         │
└────────────────────────────────────────────┘
```

## Animations

| Animation | Property | Duration | Easing |
|-----------|----------|---------|--------|
| Capsule appear | `slide-up` | 300ms | `ease-out` |
| Capsule dismiss | `fade-out` | 200ms | `ease-in` |
| Waveform bar | `scaleY` | 1200ms | `ease-in-out` (infinite) |
| Dot pulse | `opacity + scale` | 2000ms | `ease-in-out` (infinite) |
| Text appear | `fade-in` | 200ms | `ease-out` |

## Backdrop

- `backdrop-filter: blur(20px)`
- Light theme: `bg-white/80`
- Dark theme: `bg-neutral-900/80`
- Border: `1px solid neutral-200/50` (light) / `neutral-700/50` (dark)
- Recording border accent: `border-brand-400/50`

## Implementation Notes

The capsule window uses Tauri's window API:
```rust
// In tauri.conf.json:
{
  "label": "capsule",
  "decorations": false,
  "alwaysOnTop": true,
  "skipTaskbar": true,
  "transparent": true,
  "shadow": false,
  "visible": false
}
```

The frontend controls visibility via:
```typescript
import { getCurrentWindow } from "@tauri-apps/api/window";

const capsule = getCurrentWindow();

// Show/hide based on recording state
await capsule.show();
await capsule.hide();

// Reposition on screen change
await capsule.center();
```
