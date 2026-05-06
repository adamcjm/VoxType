use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub stt: SttConfig,
    pub llm: LlmConfig,
    pub hotkey: HotkeyConfig,
    pub translate: TranslateConfig,
    pub theme: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SttConfig {
    pub provider: SttProvider,
    pub base_url: String,
    pub api_key: String,
    pub model: String,
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SttProvider {
    Groq,
    OpenAi,
    Deepgram,
    Local,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    pub provider: LlmProvider,
    pub base_url: String,
    pub api_key: String,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u32,
    pub custom_prompt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LlmProvider {
    OpenAI,
    DeepSeek,
    Anthropic,
    Gemini,
    Groq,
    Ollama,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotkeyConfig {
    pub macos: String,
    pub other: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslateConfig {
    pub enabled: bool,
    pub source_lang: String,
    pub target_lang: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            stt: SttConfig {
                provider: SttProvider::Groq,
                base_url: "https://api.groq.com/openai/v1".to_string(),
                api_key: String::new(),
                model: "whisper-large-v3-turbo".to_string(),
                language: "zh".to_string(),
            },
            llm: LlmConfig {
                provider: LlmProvider::DeepSeek,
                base_url: "https://api.deepseek.com/v1".to_string(),
                api_key: String::new(),
                model: "deepseek-chat".to_string(),
                temperature: 0.3,
                max_tokens: 4096,
                custom_prompt: None,
            },
            hotkey: HotkeyConfig {
                macos: "Fn".to_string(),
                other: "RightAlt".to_string(),
            },
            translate: TranslateConfig {
                enabled: false,
                source_lang: "auto".to_string(),
                target_lang: "en".to_string(),
            },
            theme: "system".to_string(),
        }
    }
}
