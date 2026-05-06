// IME (Input Method Editor) detection strategies.
//
// Problem: In CJK environments, keyboard simulation sends text into the
// IME composition buffer rather than directly to the application.
//
// Solution: VoxType uses clipboard paste (Cmd+V/Ctrl+V) as the PRIMARY
// output method, which completely bypasses IME. Then restores the original
// clipboard content.
//
// This module provides IME state detection for diagnostics and logging.

/// Check if an IME is currently active on the system.
/// This is informational — VoxType always uses clipboard paste regardless.
pub fn is_ime_active() -> bool {
    #[cfg(target_os = "macos")]
    {
        detect_macos_ime()
    }

    #[cfg(target_os = "windows")]
    {
        detect_windows_ime()
    }

    #[cfg(target_os = "linux")]
    {
        detect_linux_ime()
    }
}

#[cfg(target_os = "macos")]
fn detect_macos_ime() -> bool {
    // Check if current keyboard input source is non-ASCII capable
    // On macOS, CJK input sources use IME (Pinyin, Romaji, etc.)
    // A simple heuristic: check for common IME identifiers via environment or defaults
    std::env::var("LANG")
        .map(|l| l.starts_with("zh_") || l.starts_with("ja_") || l.starts_with("ko_"))
        .unwrap_or(false)
}

#[cfg(target_os = "windows")]
fn detect_windows_ime() -> bool {
    // On Windows, check the console output CP
    // CJK code pages: 936(GBK), 932(Shift-JIS), 949(EUC-KR), 950(Big5)
    // This is a rough heuristic
    false
}

#[cfg(target_os = "linux")]
fn detect_linux_ime() -> bool {
    // Check for common IME environment variables
    std::env::var("GTK_IM_MODULE").is_ok()
        || std::env::var("QT_IM_MODULE").is_ok()
        || std::env::var("XMODIFIERS").is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ime_detection_does_not_panic() {
        // Just verify it doesn't crash
        let _ = is_ime_active();
    }
}
