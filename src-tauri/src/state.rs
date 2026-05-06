use std::sync::Mutex;

use crate::config::AppConfig;
use crate::pipeline::Pipeline;

pub struct AppState {
    pub config: Mutex<AppConfig>,
    pub pipeline: Mutex<Pipeline>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            config: Mutex::new(AppConfig::default()),
            pipeline: Mutex::new(Pipeline::new()),
        }
    }
}
