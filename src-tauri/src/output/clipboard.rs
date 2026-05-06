use arboard::Clipboard;
use crate::error::{Result, VoxTypeError};
use std::time::Duration;
use tokio::time::sleep;
use tracing;

/// Set clipboard text + simulate Cmd+V / Ctrl+V + restore original.
/// This is the primary text output method — it bypasses IME entirely.
pub async fn paste_with_clipboard(text: &str) -> Result<()> {
    if text.is_empty() {
        return Ok(());
    }

    let mut clipboard = Clipboard::new().map_err(|e| {
        VoxTypeError::Output(format!("Clipboard access denied: {}. Grant accessibility permission.", e))
    })?;

    // Save original clipboard content
    let original = clipboard.get_text().unwrap_or_default();
    tracing::debug!("Clipboard original saved: {} chars", original.len());

    // Set our text
    clipboard.set_text(text).map_err(|e| {
        VoxTypeError::Output(format!("Failed to set clipboard: {}", e))
    })?;
    tracing::debug!("Clipboard set with new text: {} chars", text.len());

    // Small delay for clipboard to propagate
    sleep(Duration::from_millis(30)).await;

    // Simulate paste keystroke
    super::keyboard::simulate_paste_keystroke()?;

    // Wait for paste to complete before restoring clipboard
    // The paste operation is async from the target app's perspective
    sleep(Duration::from_millis(150)).await;

    // Restore original clipboard
    match clipboard.set_text(&original) {
        Ok(()) => tracing::debug!("Clipboard restored: {} chars", original.len()),
        Err(e) => {
            // Non-fatal: clipboard restore failed but text was already pasted
            tracing::warn!("Failed to restore clipboard: {}", e);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_paste_empty_text() {
        // Empty text should be no-op
        let result = paste_with_clipboard("").await;
        assert!(result.is_ok());
    }
}
