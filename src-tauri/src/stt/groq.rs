use async_trait::async_trait;
use crate::config::SttConfig;
use crate::error::Result;
use crate::stt::{AudioFormat, SttProvider};

pub struct GroqProvider {
    config: SttConfig,
}

impl GroqProvider {
    pub fn new(config: SttConfig) -> Self {
        Self { config }
    }
}

#[async_trait]
impl SttProvider for GroqProvider {
    fn name(&self) -> &str { "Groq" }

    fn is_available(&self) -> bool {
        !self.config.api_key.is_empty()
    }

    fn supported_languages(&self) -> Vec<&str> {
        vec!["zh", "en", "ja", "ko", "de", "fr", "es", "pt", "ru", "ar", "hi"]
    }

    async fn transcribe(&self, _audio_data: &[u8], _format: AudioFormat, _language: Option<&str>) -> Result<String> {
        // TODO: Implement Groq Whisper API call
        Err(crate::error::VoxTypeError::Stt("Not implemented".to_string()))
    }
}
