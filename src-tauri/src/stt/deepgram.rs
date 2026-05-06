use async_trait::async_trait;
use crate::config::SttConfig;
use crate::error::{Result, VoxTypeError};
use crate::stt::{AudioFormat, SttProvider};
use reqwest::Client;
use tracing;

pub struct DeepgramProvider {
    config: SttConfig,
    client: Client,
}

impl DeepgramProvider {
    pub fn new(config: SttConfig) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }

    fn build_url(&self, language: Option<&str>) -> String {
        let mut url = format!("{}/listen", self.config.base_url.trim_end_matches('/'));
        let mut params = vec![];

        if !self.config.model.is_empty() {
            params.push(format!("model={}", self.config.model));
        }
        if let Some(lang) = language {
            params.push(format!("language={}", lang));
        }
        params.push("smart_format=true".to_string());

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }
        url
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

    async fn transcribe(&self, audio_data: &[u8], format: AudioFormat, language: Option<&str>) -> Result<String> {
        if !self.is_available() {
            return Err(VoxTypeError::Stt(
                "Deepgram API key not configured".to_string(),
            ));
        }

        let url = self.build_url(language);
        tracing::info!("Deepgram: POST {}", url);

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Token {}", self.config.api_key))
            .header("Content-Type", format.mime_type())
            .body(audio_data.to_vec())
            .send()
            .await
            .map_err(|e| VoxTypeError::Network(e))?;

        let status = response.status();
        let body = response.text().await.unwrap_or_default();

        if !status.is_success() {
            tracing::error!("Deepgram API error ({}): {}", status.as_u16(), body);
            return Err(VoxTypeError::Stt(format!(
                "Deepgram API error ({}): {}",
                status.as_u16(),
                &body[..body.len().min(200)]
            )));
        }

        let json: serde_json::Value = serde_json::from_str(&body)
            .map_err(|e| VoxTypeError::Stt(format!("Failed to parse Deepgram response: {}", e)))?;

        json["results"]["channels"][0]["alternatives"][0]["transcript"]
            .as_str()
            .map(|s| s.trim().to_string())
            .ok_or_else(|| {
                VoxTypeError::Stt(format!(
                    "Unexpected Deepgram response: {}",
                    &body[..body.len().min(200)]
                ))
            })
    }
}
