use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SttConfig {
    #[serde(rename = "provider")]
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
    #[serde(rename = "openai")]
    OpenAi,
    Deepgram,
    Local,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    #[serde(rename = "provider")]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub stt: SttConfig,
    pub llm: LlmConfig,
    pub hotkey: HotkeyConfig,
    pub translate: TranslateConfig,
    pub theme: String,
}

impl AppConfig {
    /// Load config from disk
    pub fn load() -> Option<Self> {
        let path = Self::config_path().ok()?;
        let data = std::fs::read_to_string(&path).ok()?;
        serde_json::from_str(&data).ok()
    }

    /// Save config to disk
    pub fn save(&self) -> Result<(), String> {
        let path = Self::config_path()?;
        if let Some(parent) = std::path::Path::new(&path).parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create config dir: {}", e))?;
        }
        let data = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;
        std::fs::write(&path, data)
            .map_err(|e| format!("Failed to write config: {}", e))?;
        Ok(())
    }

    fn config_path() -> Result<String, String> {
        #[cfg(target_os = "macos")]
        {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
            Ok(format!("{}/Library/Application Support/com.voxtype.app/config.json", home))
        }

        #[cfg(target_os = "windows")]
        {
            let appdata = std::env::var("APPDATA").unwrap_or_else(|_| "C:\\".into());
            Ok(format!("{}\\VoxType\\config.json", appdata))
        }

        #[cfg(target_os = "linux")]
        {
            let config = std::env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| {
                let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
                format!("{}/.config", home)
            });
            Ok(format!("{}/voxtype/config.json", config))
        }
    }
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
