use std::sync::Mutex;

use crate::audio::AudioCapture;
use crate::config::AppConfig;
use crate::history::HistoryStore;

pub struct AppState {
    pub config: Mutex<AppConfig>,
    pub audio_capture: Mutex<AudioCapture>,
    pub history: HistoryStore,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            config: Mutex::new(AppConfig::load().unwrap_or_default()),
            audio_capture: Mutex::new(
                AudioCapture::new().expect("Failed to initialize audio capture"),
            ),
            history: HistoryStore::new().expect("Failed to initialize history store"),
        }
    }
}
