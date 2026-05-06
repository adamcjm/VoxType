use async_trait::async_trait;
use crate::config::SttConfig;
use crate::error::Result;
use crate::stt::{AudioFormat, SttProvider};

pub struct LocalWhisperProvider {
    #[allow(dead_code)]
    config: SttConfig,
}

impl LocalWhisperProvider {
    pub fn new(config: SttConfig) -> Self {
        Self { config }
    }
}

#[async_trait]
impl SttProvider for LocalWhisperProvider {
    fn name(&self) -> &str { "Local Whisper" }

    fn is_available(&self) -> bool {
        true // Always "available" - check model file existence at transcribe time
    }

    fn supported_languages(&self) -> Vec<&str> {
        vec!["zh", "en", "ja", "ko", "de", "fr", "es", "pt", "ru", "ar", "hi"]
    }

    async fn transcribe(&self, _audio_data: &[u8], _format: AudioFormat, _language: Option<&str>) -> Result<String> {
        // TODO: Implement local whisper.cpp subprocess call
        Err(crate::error::VoxTypeError::Stt("Local Whisper not yet implemented".to_string()))
    }
}
