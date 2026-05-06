use std::sync::Mutex;

use crate::audio::AudioCapture;
use crate::config::AppConfig;

pub struct AppState {
    pub config: Mutex<AppConfig>,
    pub audio_capture: Mutex<AudioCapture>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            config: Mutex::new(AppConfig::default()),
            audio_capture: Mutex::new(
                AudioCapture::new().expect("Failed to initialize audio capture"),
            ),
        }
    }
}
