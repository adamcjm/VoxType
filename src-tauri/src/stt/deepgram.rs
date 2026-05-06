use async_trait::async_trait;
use crate::config::SttConfig;
use crate::error::Result;
use crate::stt::{AudioFormat, SttProvider};

pub struct DeepgramProvider {
    config: SttConfig,
}

impl DeepgramProvider {
    pub fn new(config: SttConfig) -> Self {
        Self { config }
    }
}

#[async_trait]
impl SttProvider for DeepgramProvider {
    fn name(&self) -> &str { "Deepgram" }

    fn is_available(&self) -> bool {
        !self.config.api_key.is_empty()
    }

    fn supported_languages(&self) -> Vec<&str> {
        vec!["zh", "en", "ja", "ko", "de", "fr", "es", "pt"]
    }

    async fn transcribe(&self, _audio_data: &[u8], _format: AudioFormat, _language: Option<&str>) -> Result<String> {
        // TODO: Implement Deepgram API call
        Err(crate::error::VoxTypeError::Stt("Not implemented".to_string()))
    }
}
