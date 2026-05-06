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
    /// Load config with priority:
    /// 1. `.env` file in project root (developer convenience)
    /// 2. `config.json` in app data dir (user saved via Settings)
    /// 3. Code defaults
    pub fn load() -> Self {
        // Try .env first (developer mode)
        let env_config = Self::from_env_file();

        // Try JSON config (user Settings)
        let json_config = Self::from_json_file();

        // Merge: JSON config takes priority for provider/model settings,
        // but env vars can override API keys
        let mut config = match (env_config, json_config) {
            (Some(env), Some(json)) => {
                // JSON has user's saved preferences, env overrides API keys
                let mut merged = json;
                if !env.stt.api_key.is_empty() {
                    merged.stt.api_key = env.stt.api_key;
                }
                if !env.llm.api_key.is_empty() {
                    merged.llm.api_key = env.llm.api_key;
                }
                merged
            }
            (Some(env), None) => env,
            (None, Some(json)) => json,
            (None, None) => Self::default(),
        };

        // Override with process env vars (highest priority for individual keys)
        Self::apply_env_overrides(&mut config);

        config
    }

    /// Load from `.env` file in project root
    fn from_env_file() -> Option<Self> {
        let cwd = std::env::current_dir().ok()?;
        let env_path = cwd.join(".env");

        if !env_path.exists() {
            return None;
        }

        tracing::info!("Loading config from .env: {}", env_path.display());
        dotenvy::from_path(&env_path).ok()?;

        let stt_provider = std::env::var("VOXTYPE_STT_PROVIDER").unwrap_or_default();
        let stt_key = std::env::var("VOXTYPE_STT_API_KEY").unwrap_or_default();
        let stt_model = std::env::var("VOXTYPE_STT_MODEL").unwrap_or_default();
        let stt_lang = std::env::var("VOXTYPE_STT_LANGUAGE").unwrap_or_else(|_| "zh".into());
        let stt_url = std::env::var("VOXTYPE_STT_BASE_URL");

        let llm_provider = std::env::var("VOXTYPE_LLM_PROVIDER").unwrap_or_default();
        let llm_key = std::env::var("VOXTYPE_LLM_API_KEY").unwrap_or_default();
        let llm_model = std::env::var("VOXTYPE_LLM_MODEL").unwrap_or_default();
        let llm_url = std::env::var("VOXTYPE_LLM_BASE_URL");

        // Require at least one key to be set for .env to be valid
        if stt_key.is_empty() && llm_key.is_empty() {
            return None;
        }

        let mut config = Self::default();

        if !stt_provider.is_empty() {
            config.stt.provider = parse_stt_provider(&stt_provider);
        }
        if !stt_key.is_empty() {
            config.stt.api_key = stt_key;
        }
        if !stt_model.is_empty() {
            config.stt.model = stt_model;
        }
        config.stt.language = stt_lang;
        if let Ok(url) = stt_url {
            if !url.is_empty() { config.stt.base_url = url; }
        }

        if !llm_provider.is_empty() {
            config.llm.provider = parse_llm_provider(&llm_provider);
        }
        if !llm_key.is_empty() {
            config.llm.api_key = llm_key;
        }
        if !llm_model.is_empty() {
            config.llm.model = llm_model;
        }
        if let Ok(url) = llm_url {
            if !url.is_empty() { config.llm.base_url = url; }
        }

        Some(config)
    }

    /// Load from platform-specific config.json
    fn from_json_file() -> Option<Self> {
        let path = Self::config_path().ok()?;
        let data = std::fs::read_to_string(&path).ok()?;
        let config: Self = serde_json::from_str(&data).ok()?;
        tracing::info!("Config loaded from: {}", path);
        Some(config)
    }

    /// Override individual fields from process environment variables (highest priority)
    fn apply_env_overrides(config: &mut Self) {
        if let Ok(key) = std::env::var("VOXTYPE_STT_API_KEY") {
            if !key.is_empty() { config.stt.api_key = key; }
        }
        if let Ok(key) = std::env::var("VOXTYPE_LLM_API_KEY") {
            if !key.is_empty() { config.llm.api_key = key; }
        }
    }

    /// Check if onboarding is needed (no API keys configured)
    pub fn needs_onboarding(&self) -> bool {
        // Local Whisper doesn't need an API key
        let stt_needs_key = self.stt.provider != SttProvider::Local && self.stt.api_key.is_empty();
        let llm_needs_key = self.llm.api_key.is_empty();

        stt_needs_key && llm_needs_key
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
        std::fs::write(&path, &data)
            .map_err(|e| format!("Failed to write config: {}", e))?;
        tracing::info!("Config saved to: {}", path);
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

fn parse_stt_provider(s: &str) -> SttProvider {
    match s.to_lowercase().as_str() {
        "groq" => SttProvider::Groq,
        "openai" => SttProvider::OpenAi,
        "deepgram" => SttProvider::Deepgram,
        "local" => SttProvider::Local,
        "custom" => SttProvider::Custom,
        _ => SttProvider::Groq,
    }
}

fn parse_llm_provider(s: &str) -> LlmProvider {
    match s.to_lowercase().as_str() {
        "openai" => LlmProvider::OpenAI,
        "deepseek" => LlmProvider::DeepSeek,
        "anthropic" => LlmProvider::Anthropic,
        "gemini" => LlmProvider::Gemini,
        "groq" => LlmProvider::Groq,
        "ollama" => LlmProvider::Ollama,
        "custom" => LlmProvider::Custom,
        _ => LlmProvider::DeepSeek,
    }
}
