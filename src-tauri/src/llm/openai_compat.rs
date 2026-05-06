use async_trait::async_trait;
use crate::config::LlmConfig;
use crate::error::{Result, VoxTypeError};
use crate::llm::{LlmProvider, PolishMode};
use crate::llm::prompts;
use reqwest::Client;
use serde_json::json;
use tokio::time::{sleep, Duration};
use tracing;

const MAX_RETRIES: u32 = 2;
const RETRY_DELAY_MS: u64 = 800;

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

    /// Select the best system prompt based on polish mode
    fn select_prompt(&self, mode: &PolishMode) -> String {
        // User's custom prompt takes absolute priority
        if let Some(ref custom) = self.config.custom_prompt {
            if !custom.is_empty() {
                return custom.clone();
            }
        }

        match mode {
            PolishMode::Cleanup => prompts::CLEANUP_PROMPT.to_string(),
            PolishMode::Translate { source_lang, target_lang } => {
                prompts::translate_prompt(source_lang, target_lang)
            }
            PolishMode::Format => prompts::FORMAT_PROMPT.to_string(),
        }
    }

    /// Send a chat completion request with retry logic
    async fn chat_completion(&self, system_prompt: &str, user_text: &str) -> Result<String> {
        let url = format!(
            "{}/chat/completions",
            self.config.base_url.trim_end_matches('/')
        );

        let body = json!({
            "model": self.config.model,
            "temperature": self.config.temperature,
            "max_tokens": self.config.max_tokens,
            "messages": [
                {"role": "system", "content": system_prompt},
                {"role": "user", "content": user_text}
            ]
        });

        let mut last_error = String::new();
        let mut should_retry = true;

        for attempt in 0..=MAX_RETRIES {
            if attempt > 0 && should_retry {
                let delay = RETRY_DELAY_MS * (1 << (attempt - 1));
                tracing::warn!("LLM: retry {}/{} after {}ms", attempt, MAX_RETRIES, delay);
                sleep(Duration::from_millis(delay)).await;
            }

            if attempt > 0 && !should_retry {
                break;
            }

            match self.try_request(&url, &body).await {
                Ok(text) => return Ok(text),
                Err(e) => {
                    last_error = e.to_string();
                    // Don't retry on auth errors (401/403) or rate limits (429)
                    should_retry = !last_error.contains("401:")
                        && !last_error.contains("403:")
                        && !last_error.contains("429:");
                }
            }
        }

        Err(VoxTypeError::Llm(format!(
            "LLM request failed after {} attempts: {}",
            MAX_RETRIES + 1,
            last_error
        )))
    }

    /// Single HTTP request attempt
    async fn try_request(&self, url: &str, body: &serde_json::Value) -> Result<String> {
        let response = self.client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await
            .map_err(|e| VoxTypeError::Network(e))?;

        let status = response.status();

        if status.is_success() {
            let json_resp: serde_json::Value = response.json().await.map_err(|e| {
                VoxTypeError::Llm(format!("Failed to parse LLM response: {}", e))
            })?;

            // Track token usage
            if let Some(usage) = json_resp.get("usage") {
                tracing::info!(
                    "LLM usage: prompt={}, completion={}, total={}",
                    usage.get("prompt_tokens").and_then(|t| t.as_u64()).unwrap_or(0),
                    usage.get("completion_tokens").and_then(|t| t.as_u64()).unwrap_or(0),
                    usage.get("total_tokens").and_then(|t| t.as_u64()).unwrap_or(0),
                );
            }

            let content = json_resp["choices"][0]["message"]["content"]
                .as_str()
                .unwrap_or("")
                .to_string();

            if content.trim().is_empty() {
                return Err(VoxTypeError::Llm(
                    "LLM returned empty response".to_string()
                ));
            }

            return Ok(content.trim().to_string());
        }

        // Build user-friendly error message (prefixed with status code for retry logic)
        let error_msg = match status.as_u16() {
            401 => "401: Invalid API key. Check your LLM provider settings.".to_string(),
            403 => "403: API access denied. Your account may not have access to this model.".to_string(),
            429 => "429: Rate limit exceeded. Please wait a moment and try again.".to_string(),
            500..=599 => "LLM provider server error. Try again later.".to_string(),
            _ => {
                let body = response.text().await.unwrap_or_default();
                format!("LLM API error ({}): {}", status.as_u16(), &body[..body.len().min(300)])
            }
        };

        Err(VoxTypeError::Llm(error_msg))
    }
}

#[async_trait]
impl LlmProvider for OpenAiCompatProvider {
    fn name(&self) -> &str {
        match self.config.provider {
            crate::config::LlmProvider::OpenAI => "OpenAI",
            crate::config::LlmProvider::DeepSeek => "DeepSeek",
            crate::config::LlmProvider::Anthropic => "Anthropic",
            crate::config::LlmProvider::Gemini => "Gemini",
            crate::config::LlmProvider::Groq => "Groq",
            crate::config::LlmProvider::Ollama => "Ollama",
            crate::config::LlmProvider::Custom => "Custom LLM",
        }
    }

    fn is_available(&self) -> bool {
        !self.config.api_key.is_empty()
    }

    async fn polish(&self, text: &str, mode: PolishMode) -> Result<String> {
        if !self.is_available() {
            return Err(VoxTypeError::Llm(
                "LLM API key not configured. Go to Settings → AI Polish to set up.".to_string(),
            ));
        }

        if text.trim().is_empty() {
            return Ok(String::new());
        }

        tracing::info!(
            "LLM polish: provider={}, model={}, mode={:?}, text_len={}",
            self.name(),
            self.config.model,
            mode,
            text.len(),
        );

        let system_prompt = self.select_prompt(&mode);
        self.chat_completion(&system_prompt, text).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, path};
    use crate::config::LlmProvider as LlmProviderCfg;

    fn test_config(server_url: &str) -> LlmConfig {
        LlmConfig {
            provider: LlmProviderCfg::DeepSeek,
            base_url: server_url.to_string(),
            api_key: "test-key".to_string(),
            model: "test-model".to_string(),
            temperature: 0.3,
            max_tokens: 100,
            custom_prompt: None,
        }
    }

    #[tokio::test]
    async fn test_polish_success() {
        let server = MockServer::start().await;
        let config = test_config(&server.uri());
        let provider = OpenAiCompatProvider::new(config);

        Mock::given(method("POST"))
            .and(path("/chat/completions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::json!({
                    "choices": [{"message": {"content": "Cleaned text."}}],
                    "usage": {"prompt_tokens": 50, "completion_tokens": 5, "total_tokens": 55}
                })
            ))
            .expect(1)
            .mount(&server)
            .await;

        let result = provider.polish("raw text with um filler", PolishMode::Cleanup).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Cleaned text.");
    }

    #[tokio::test]
    async fn test_polish_auth_error() {
        let server = MockServer::start().await;
        let config = test_config(&server.uri());
        let provider = OpenAiCompatProvider::new(config);

        Mock::given(method("POST"))
            .and(path("/chat/completions"))
            .respond_with(ResponseTemplate::new(401).set_body_string("Unauthorized"))
            .expect(1)
            .mount(&server)
            .await;

        let result = provider.polish("test", PolishMode::Cleanup).await;
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("401"), "Expected 401 error, got: {}", err);
    }

    #[tokio::test]
    async fn test_polish_empty_text() {
        let provider = OpenAiCompatProvider::new(test_config("http://localhost"));
        let result = provider.polish("", PolishMode::Cleanup).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }

    #[tokio::test]
    async fn test_no_api_key() {
        let mut config = test_config("http://localhost");
        config.api_key = String::new();
        let provider = OpenAiCompatProvider::new(config);

        let result = provider.polish("test", PolishMode::Cleanup).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_translate_mode() {
        let server = MockServer::start().await;
        let config = test_config(&server.uri());
        let provider = OpenAiCompatProvider::new(config);

        Mock::given(method("POST"))
            .and(path("/chat/completions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::json!({"choices": [{"message": {"content": "Hello"}}]})
            ))
            .expect(1)
            .mount(&server)
            .await;

        let result = provider.polish(
            "你好",
            PolishMode::Translate {
                source_lang: "zh".to_string(),
                target_lang: "en".to_string(),
            },
        ).await;
        assert!(result.is_ok());
    }
}
