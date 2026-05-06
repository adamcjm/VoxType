pub mod openai_compat;
pub mod prompts;

use async_trait::async_trait;
use crate::config::LlmConfig;
use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolishMode {
    Cleanup,
    Translate { source_lang: String, target_lang: String },
    Format,
}

#[async_trait]
pub trait LlmProvider: Send + Sync {
    fn name(&self) -> &str;
    fn is_available(&self) -> bool;
    async fn polish(&self, text: &str, mode: PolishMode) -> Result<String>;
}

pub struct LlmManager {
    config: LlmConfig,
}

impl LlmManager {
    pub fn new(config: LlmConfig) -> Self {
        Self { config }
    }

    pub async fn polish(&self, text: &str, mode: PolishMode) -> Result<String> {
        let provider = self.create_provider();
        provider.polish(text, mode).await
    }

    fn create_provider(&self) -> Box<dyn LlmProvider> {
        Box::new(openai_compat::OpenAiCompatProvider::new(self.config.clone()))
    }
}
