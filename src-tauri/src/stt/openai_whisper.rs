use async_trait::async_trait;
use crate::config::SttConfig;
use crate::error::Result;
use crate::stt::{AudioFormat, SttProvider, common};
use reqwest::Client;

pub struct OpenAiWhisperProvider {
    config: SttConfig,
    client: Client,
}

impl OpenAiWhisperProvider {
    pub fn new(config: SttConfig) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }

    /// Create a provider with a custom base URL (for custom endpoints)
    pub fn new_custom(config: SttConfig) -> Self {
        Self::new(config)
    }
}

#[async_trait]
impl SttProvider for OpenAiWhisperProvider {
    fn name(&self) -> &str {
        if matches!(self.config.provider, crate::config::SttProvider::Custom) {
            "Custom"
        } else {
            "OpenAI Whisper"
        }
    }

    fn is_available(&self) -> bool {
        !self.config.api_key.is_empty()
    }

    fn supported_languages(&self) -> Vec<&str> {
        vec!["zh", "en", "ja", "ko", "de", "fr", "es", "pt", "ru", "ar", "hi"]
    }

    async fn transcribe(&self, audio_data: &[u8], format: AudioFormat, language: Option<&str>) -> Result<String> {
        if !self.is_available() {
            return Err(crate::error::VoxTypeError::Stt(
                "API key not configured".to_string(),
            ));
        }

        common::transcribe_openai_compat(
            &self.client,
            &self.config.base_url,
            &self.config.api_key,
            &self.config.model,
            audio_data,
            format.mime_type(),
            format.extension(),
            language,
        )
        .await
    }
}
