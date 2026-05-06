pub mod keyboard;
pub mod clipboard;
pub mod ime;

use crate::error::Result;
use tracing;

/// Write text to the current cursor position.
///
/// Strategy:
/// 1. Clipboard paste (Cmd+V/Ctrl+V) — bypasses IME, works everywhere
/// 2. Keyboard simulation — direct key events, may trigger IME
pub async fn write_text(text: &str) -> Result<()> {
    if text.is_empty() {
        return Ok(());
    }

    // Always use clipboard paste as it bypasses IME issues entirely
    match clipboard::paste_with_clipboard(text).await {
        Ok(()) => {
            tracing::info!("Text output via clipboard paste: {} chars", text.len());
            Ok(())
        }
        Err(clip_err) => {
            tracing::warn!("Clipboard paste failed: {}. Trying keyboard sim.", clip_err);
            // Fallback: direct keyboard simulation
            keyboard::type_text(text)
        }
    }
}
