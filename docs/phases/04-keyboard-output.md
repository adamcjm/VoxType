# Phase S3: Keyboard Output

## Date
2026-05-06

## Goal
Implement cross-platform keyboard simulation and text output with IME bypass.

## Deliverables

### 1. Output Manager (`output/mod.rs`)
- [x] `write_text()` — unified entry point for text output
- [x] Primary: clipboard paste (Cmd+V/Ctrl+V)
- [x] Fallback: keyboard simulation via enigo

### 2. Keyboard Simulation (`output/keyboard.rs`)
- [x] `type_text()` — character-by-character input via enigo
- [x] `simulate_paste_keystroke()` — Cmd+V (macOS) / Ctrl+V (Win/Linux)
- [x] Platform-aware modifier keys (Meta vs Control)
- [x] Shift character handling for special characters
- [x] Unicode support for CJK and emoji

### 3. Clipboard Manager (`output/clipboard.rs`)
- [x] `paste_with_clipboard()` — set clipboard → simulate paste → restore
- [x] Original clipboard content save/restore
- [x] Timing delays: 30ms set, 150ms wait before restore
- [x] Bypasses IME entirely (clipboard paste doesn't trigger composition)

### 4. IME Detection (`output/ime.rs`)
- [x] `is_ime_active()` — platform-specific heuristics
- [x] macOS: LANG env var (zh_/ja_/ko_)
- [x] Linux: GTK_IM_MODULE/QT_IM_MODULE/XMODIFIERS env vars
- [x] Windows: placeholder for future implementation

### 5. Pipeline Integration
- [x] Pipeline now uses `output::write_text()` instead of direct clipboard call
- [x] Clipboard paste as primary output method (works with all IMEs)

### 6. Test Coverage
- [x] `test_is_shift_char` — shift character detection
- [x] `test_paste_empty_text` — empty text no-op
- [x] `test_ime_detection_does_not_panic` — IME detection safety
- [x] **Total: 20 tests, 20 passed, 0 failed** (+3 from S2)

## Architecture

```
write_text(text)
  │
  ├─ paste_with_clipboard(text)   ← PRIMARY (bypasses IME)
  │   ├─ Save original clipboard
  │   ├─ Set new text to clipboard
  │   ├─ simulate_paste_keystroke()  → Cmd+V / Ctrl+V via enigo
  │   └─ Restore original clipboard (after 150ms delay)
  │
  └─ type_text(text)              ← FALLBACK (may trigger IME)
      └─ Character loop → enigo.key(Key::Unicode(ch), Click)
```

## Platform Details

| Platform | Paste Modifier | Keyboard Backend |
|----------|---------------|-----------------|
| macOS | Cmd (Meta) | CGEventPost via enigo |
| Windows | Ctrl | SendInput via enigo |
| Linux | Ctrl | X11/xdotool via enigo |

## Build Results

| Artifact | Status |
|----------|--------|
| Rust compile | 0 errors, 0 warnings |
| Tests | 20/20 passed |
