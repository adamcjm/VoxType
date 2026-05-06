/// Global hotkey module.
///
/// Platform-specific hotkey targets:
///   macOS:   Fn key (kVK_Function, keycode 63)
///   Windows: Right Alt (VK_RMENU)  
///   Linux:   Right Alt
///
/// Implementation notes:
///   Global hotkeys require platform-specific APIs and accessibility permissions.
///   For the current MVP, hotkey handling is done via Tauri window keyboard events
///   in the capsule window (always-on-top, captures key events).
///
///   The capsule window runs as an always-on-top transparent window that listens
///   for the target key, which provides a simpler cross-platform path.
///
///   Future: implement OS-level global hotkey using:
///     - macOS: CGEventTap or Carbon Event HotKey
///     - Windows: RegisterHotKey / SetWindowsHookEx
///     - Linux: X11 XGrabKey

/// Get the human-readable description of the hotkey for the current platform
pub fn hotkey_description() -> &'static str {
    #[cfg(target_os = "macos")]
    {
        "Fn (globe key)"
    }

    #[cfg(not(target_os = "macos"))]
    {
        "Right Alt"
    }
}

/// Get the keycode string for the current platform's hotkey
pub fn hotkey_keycode() -> &'static str {
    #[cfg(target_os = "macos")]
    {
        "Fn"
    }

    #[cfg(not(target_os = "macos"))]
    {
        "AltRight"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hotkey_description_not_empty() {
        let desc = hotkey_description();
        assert!(!desc.is_empty());
    }

    #[test]
    fn test_hotkey_keycode_not_empty() {
        let code = hotkey_keycode();
        assert!(!code.is_empty());
    }
}
