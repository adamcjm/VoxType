//! Unified data directory: `~/.VoxType/`
//!
//! Cross-platform consistent — one path on macOS, Windows, and Linux.
//!
//! Directory layout:
//! ```text
//! ~/.VoxType/
//! ├── config.json     # User settings (STT/LLM providers, hotkeys, theme)
//! ├── history.db      # SQLite transcription history
//! └── models/          # Downloaded Whisper models
//!     └── ggml-small.bin
//! ```

use std::path::PathBuf;

/// Get the VoxType data directory: `~/.VoxType/`
///
/// # Panics
/// Only panics if HOME directory cannot be determined.
pub fn data_dir() -> PathBuf {
    let home = if cfg!(target_os = "windows") {
        std::env::var("USERPROFILE").unwrap_or_else(|_| "C:\\".into())
    } else {
        std::env::var("HOME").unwrap_or_else(|_| "/tmp".into())
    };
    PathBuf::from(home).join(".VoxType")
}

/// `~/.VoxType/config.json`
pub fn config_file() -> PathBuf {
    data_dir().join("config.json")
}

/// `~/.VoxType/history.db`
pub fn history_db() -> PathBuf {
    data_dir().join("history.db")
}

/// `~/.VoxType/models/`
pub fn models_dir() -> PathBuf {
    data_dir().join("models")
}

/// Ensure the data directory (and subdirectories) exist
pub fn ensure_dirs() -> Result<(), String> {
    let dir = data_dir();
    std::fs::create_dir_all(&dir)
        .map_err(|e| format!("Failed to create {}: {}", dir.display(), e))?;

    let models = models_dir();
    std::fs::create_dir_all(&models)
        .map_err(|e| format!("Failed to create {}: {}", models.display(), e))?;

    Ok(())
}
