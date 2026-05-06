# UI/UX Design Specification

## Design Principles

1. **Invisible until needed** - App stays in system tray, appears only when recording or configuring
2. **One glance** - Status must be understood in < 0.5 seconds
3. **Zero friction** - No clicks needed to start dictating (just hold hotkey)
4. **Works everywhere** - Capsule floats on top, adapts to any app's design

## Window Architecture

### 1. Capsule Window

```
┌─────────────────────────────────────────────────────────────┐
│  ● Recording...                          0:12              │
│                                                             │
│  This is what I'm saying right now it appears here in       │
│  real time as the STT processes my voice...                 │
└─────────────────────────────────────────────────────────────┘
```

- **Type**: `decorations: false`, `alwaysOnTop: true`, `transparent: true`
- **Position**: Top center of screen, follows cursor to relevant monitor
- **Size**: `width: 360px`, `height: auto` (80-200px based on content)
- **Show**: On recording start or on error
- **Hide**: After 2s on completion, or on click-away

### 2. Settings Window

```
┌───────────────────────────────────────────┐
│  ≡ Settings                          ✕    │
│───────────────────────────────────────────│
│                                           │
│  🎤 Speech Recognition                    │
│  ┌─────────────────────────────────────┐  │
│  │ Provider  [Groq        ▼]           │  │
│  │ API Key   [•••••••••• ]            │  │
│  │ Model     [whisper-v3      ]        │  │
│  │ Language  [Chinese ▼]              │  │
│  └─────────────────────────────────────┘  │
│                                           │
│  🤖 AI Polish                             │
│  ┌─────────────────────────────────────┐  │
│  │ Provider  [DeepSeek    ▼]           │  │
│  │ API Key   [•••••••••• ]            │  │
│  │ Model     [deepseek-chat   ]        │  │
│  │ Temp      0.3          Max 4096     │  │
│  └─────────────────────────────────────┘  │
│                                           │
│  🌐 Translation                           │
│  Source [Auto ▼]  Target [English ▼]      │
│                                           │
│  ⌨️ Hotkey                                │
│  [Click to record new key]                │
│                                           │
│  ───────────────────────────────           │
│  [Reset] [Cancel]           [Save]         │
└───────────────────────────────────────────┘
```

- **Type**: Standard window, modal (blocks app interaction)
- **Size**: `640 x 520px`
- **Show**: System tray → Settings, or first launch

### 3. System Tray Menu

```
┌────────────────────────┐
│  VoxType               │ (disabled)
│ ──────────────────────  │
│  ▶ Start Dictation      │
│  ⚙ Settings...          │
│  📋 Copy Last           │
│ ──────────────────────  │
│  ⬜ About VoxType        │
│  ✕ Quit                 │
└────────────────────────┘
```

## Interaction States

### Recording Flow

```
  idle ──[hotkey press]──▶ recording ──[hotkey press]──▶ transcribing
    ▲                                                     │
    │                                                     ▼
    └───────────[complete/error]────────── polishing
                                                          │
                                                          ▼
                                                        done ──[2s timeout]──▶ idle
```

### State Visuals

| State | Badge Color | Icon | Capsule Animation |
|-------|-------------|------|------------------|
| `idle` | - | - | Hidden |
| `recording` | Red | 🔴 | Pulse glow + waveform |
| `transcribing` | Amber | ⏳ | Spinning indicator |
| `polishing` | Amber | ✨ | Shimmer text |
| `done` | Green | ✅ | Checkmark → fade out |
| `error` | Red | ⚠️ | Error message shown |

## Theme

### Color Tokens

| Token | Light | Dark |
|-------|-------|------|
| `bg-primary` | `#FFFFFF` | `#171717` |
| `bg-secondary` | `#F5F5F5` | `#262626` |
| `text-primary` | `#171717` | `#FAFAFA` |
| `text-secondary` | `#525252` | `#A3A3A3` |
| `border` | `#E5E5E5` | `#404040` |
| `brand` | `#6366F1` | `#818CF8` |
| `error` | `#EF4444` | `#F87171` |
| `success` | `#22C55E` | `#4ADE80` |
| `warning` | `#F59E0B` | `#FBBF24` |

### Typography

- **System font stack**: `Inter, SF Pro Display, system-ui`
- **Mono**: `JetBrains Mono, SF Mono`
- **Sizes**: `12px` (label), `13px` (body), `14px` (emphasis), `16px` (heading)

## Accessibility

- All interactive elements keyboard navigable
- Color alone does not convey state (always paired with icon/text)
- WCAG AA contrast ratios
- Screen reader labels on all interactive elements
