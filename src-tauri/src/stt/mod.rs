pub mod config;
pub mod local;
pub mod groq;
pub mod openai_whisper;
pub mod deepgram;

use async_trait::async_trait;
use crate::config::SttConfig;
use crate::error::Result;

// Re-export helper
pub(crate) mod common;

#[async_trait]
pub trait SttProvider: Send + Sync {
    fn name(&self) -> &str;
    fn is_available(&self) -> bool;
    fn supported_languages(&self) -> Vec<&str>;
    async fn transcribe(&self, audio_data: &[u8], format: AudioFormat, language: Option<&str>) -> Result<String>;
}

#[derive(Debug, Clone)]
pub enum AudioFormat {
    Wav { sample_rate: u32, channels: u16 },
    Flac { sample_rate: u32, channels: u16 },
}

impl AudioFormat {
    pub fn mime_type(&self) -> &str {
        match self {
            AudioFormat::Wav { .. } => "audio/wav",
            AudioFormat::Flac { .. } => "audio/flac",
        }
    }

    pub fn extension(&self) -> &str {
        match self {
            AudioFormat::Wav { .. } => "wav",
            AudioFormat::Flac { .. } => "flac",
        }
    }
}

pub struct SttManager {
    config: SttConfig,
}

impl SttManager {
    pub fn new(config: SttConfig) -> Self {
        Self { config }
    }

    pub async fn transcribe(&self, audio: &[u8]) -> Result<String> {
        let provider = self.create_provider();
        provider
            .transcribe(
                audio,
                AudioFormat::Wav { sample_rate: 16000, channels: 1 },
                Some(&self.config.language),
            )
            .await
    }

    fn create_provider(&self) -> Box<dyn SttProvider> {
        match &self.config.provider {
            crate::config::SttProvider::Groq => {
                Box::new(groq::GroqProvider::new(self.config.clone()))
            }
            crate::config::SttProvider::OpenAi => {
                Box::new(openai_whisper::OpenAiWhisperProvider::new(self.config.clone()))
            }
            crate::config::SttProvider::Deepgram => {
                Box::new(deepgram::DeepgramProvider::new(self.config.clone()))
            }
            crate::config::SttProvider::Local => {
                Box::new(local::LocalWhisperProvider::new(self.config.clone()))
            }
            crate::config::SttProvider::Custom => {
                Box::new(openai_whisper::OpenAiWhisperProvider::new_custom(self.config.clone()))
            }
        }
    }
}
