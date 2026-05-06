use crate::error::VoxTypeError;
use enigo::{
    Enigo, Keyboard, Settings,
    Key, Direction,
};
use tracing;

/// Type text via keyboard simulation.
/// Uses enigo to simulate key presses character by character.
///
/// Note: This can trigger IME composition on CJK systems.
/// Prefer clipboard paste via `output::write_text()`.
pub fn type_text(text: &str) -> Result<(), VoxTypeError> {
    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| VoxTypeError::Output(format!("Failed to create Enigo: {}", e)))?;

    for ch in text.chars() {
        // Check if character is ASCII (no IME needed)
        if ch.is_ascii() {
            if ch.is_ascii_uppercase() || is_shift_char(ch) {
                match enigo.key(Key::Unicode(ch), Direction::Click) {
                    Ok(_) => {}
                    Err(e) => {
                        tracing::error!("Failed to type char '{}': {}", ch, e);
                        return Err(VoxTypeError::Output(format!(
                            "Failed to type character '{}': {}",
                            ch, e
                        )));
                    }
                }
            } else {
                // For ASCII lowercase and simple chars, use Unicode click
                match enigo.key(Key::Unicode(ch), Direction::Click) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(VoxTypeError::Output(format!(
                            "Failed to type character '{}': {}",
                            ch, e
                        )));
                    }
                }
            }
        } else {
            // Non-ASCII (CJK, emoji, etc.) — use Unicode
            match enigo.key(Key::Unicode(ch), Direction::Click) {
                Ok(_) => {}
                Err(e) => {
                    return Err(VoxTypeError::Output(format!(
                        "Failed to type unicode character '{}': {}",
                        ch, e
                    )));
                }
            }
        }
    }

    Ok(())
}

/// Simulate paste keystroke (Cmd+V on macOS, Ctrl+V on others)
pub fn simulate_paste_keystroke() -> Result<(), VoxTypeError> {
    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| VoxTypeError::Output(format!("Failed to create Enigo: {}", e)))?;

    let modifier = platform_paste_modifier();

    // Press modifier + V, then release
    enigo.key(modifier, Direction::Press)
        .map_err(|e| VoxTypeError::Output(format!("Failed to press paste modifier: {}", e)))?;

    enigo.key(Key::Unicode('v'), Direction::Click)
        .map_err(|e| VoxTypeError::Output(format!("Failed to press V: {}", e)))?;

    enigo.key(modifier, Direction::Release)
        .map_err(|e| VoxTypeError::Output(format!("Failed to release paste modifier: {}", e)))?;

    Ok(())
}

/// Get paste modifier for current platform
fn platform_paste_modifier() -> Key {
    #[cfg(target_os = "macos")]
    {
        Key::Meta // Cmd on macOS
    }

    #[cfg(not(target_os = "macos"))]
    {
        Key::Control
    }
}

/// Characters that typically require shift
fn is_shift_char(ch: char) -> bool {
    matches!(
        ch,
        '!' | '@' | '#' | '$' | '%' | '^' | '&' | '*' | '(' | ')' |
        '_' | '+' | '{' | '}' | '|' | ':' | '"' | '<' | '>' | '?' |
        '~' | 'A'..='Z'
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_shift_char() {
        assert!(is_shift_char('!'));
        assert!(is_shift_char('A'));
        assert!(is_shift_char('?'));
        assert!(!is_shift_char('a'));
        assert!(!is_shift_char('1'));
        assert!(!is_shift_char(' '));
    }
}
