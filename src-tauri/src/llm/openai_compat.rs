use async_trait::async_trait;
use crate::config::LlmConfig;
use crate::error::{Result, VoxTypeError};
use crate::llm::{LlmProvider, PolishMode};
use crate::llm::prompts;
use reqwest::Client;
use serde_json::json;

pub struct OpenAiCompatProvider {
    config: LlmConfig,
    client: Client,
}

impl OpenAiCompatProvider {
    pub fn new(config: LlmConfig) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }

    fn get_system_prompt(&self, mode: &PolishMode) -> String {
        match &self.config.custom_prompt {
            Some(custom) if !custom.is_empty() => custom.clone(),
            _ => match mode {
                PolishMode::Cleanup => prompts::CLEANUP_PROMPT.to_string(),
                PolishMode::Translate { source_lang, target_lang } => {
                    prompts::get_translate_prompt(source_lang, target_lang)
                }
                PolishMode::Format => prompts::FORMAT_PROMPT.to_string(),
            },
        }
    }
}

#[async_trait]
impl LlmProvider for OpenAiCompatProvider {
    fn name(&self) -> &str { "OpenAI Compatible" }

    fn is_available(&self) -> bool {
        !self.config.api_key.is_empty()
    }

    async fn polish(&self, text: &str, mode: PolishMode) -> Result<String> {
        if !self.is_available() {
            return Err(VoxTypeError::Llm("API key not configured".to_string()));
        }

        let system_prompt = self.get_system_prompt(&mode);
        let url = format!("{}/chat/completions", self.config.base_url.trim_end_matches('/'));

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .json(&json!({
                "model": self.config.model,
                "temperature": self.config.temperature,
                "max_tokens": self.config.max_tokens,
                "messages": [
                    {"role": "system", "content": system_prompt},
                    {"role": "user", "content": text}
                ]
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(VoxTypeError::Llm(
                format!("LLM API error ({}): {}", status, body)
            ));
        }

        let json_resp: serde_json::Value = response.json().await?;
        let content = json_resp["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();

        Ok(content.trim().to_string())
    }
}
