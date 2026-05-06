use arboard::Clipboard;
use crate::error::Result;
use std::time::Duration;
use tokio::time::sleep;

pub async fn paste_with_clipboard(text: &str) -> Result<()> {
    let mut clipboard = Clipboard::new().map_err(|e| {
        crate::error::VoxTypeError::Output(format!("Clipboard access failed: {}", e))
    })?;

    // Save original clipboard
    let original = clipboard.get_text().unwrap_or_default();

    // Set new text
    clipboard.set_text(text).map_err(|e| {
        crate::error::VoxTypeError::Output(format!("Clipboard set failed: {}", e))
    })?;

    // Simulate Cmd+V / Ctrl+V
    // TODO: Use enigo to simulate paste keystroke
    sleep(Duration::from_millis(50)).await;

    // Restore original clipboard
    sleep(Duration::from_millis(100)).await;
    clipboard.set_text(original).map_err(|e| {
        crate::error::VoxTypeError::Output(format!("Clipboard restore failed: {}", e))
    })?;

    Ok(())
}
