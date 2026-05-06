use async_trait::async_trait;
use crate::config::SttConfig;
use crate::error::Result;
use crate::stt::{AudioFormat, SttProvider, common};
use reqwest::Client;

pub struct GroqProvider {
    config: SttConfig,
    client: Client,
}

impl GroqProvider {
    pub fn new(config: SttConfig) -> Self {
        Self {
            config,
            client: Client::new(),
        }
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

    async fn transcribe(&self, audio_data: &[u8], format: AudioFormat, language: Option<&str>) -> Result<String> {
        if !self.is_available() {
            return Err(crate::error::VoxTypeError::Stt(
                "Groq API key not configured".to_string(),
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
