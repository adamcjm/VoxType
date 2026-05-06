use async_trait::async_trait;
use crate::config::SttConfig;
use crate::error::{Result, VoxTypeError};
use crate::stt::{AudioFormat, SttProvider};
use std::process::Command;
use tempfile::NamedTempFile;
use std::io::Write;
use tracing;
use tokio::task;

pub struct LocalWhisperProvider {
    #[allow(dead_code)]
    config: SttConfig,
}

impl LocalWhisperProvider {
    pub fn new(config: SttConfig) -> Self {
        Self { config }
    }

    /// Find whisper.cpp binary location
    fn whisper_binary() -> Option<String> {
        // Check bundled whisper binary first
        let bundled = Self::model_dir().join("whisper");
        if bundled.exists() {
            return Some(bundled.to_string_lossy().to_string());
        }

        // Check if whisper is in PATH
        for bin in &["whisper", "whisper-cpp", "whisper.cpp"] {
            if Command::new("which").arg(bin).output().is_ok() {
                return Some(bin.to_string());
            }
        }

        None
    }

    /// Path to model directory
    fn model_dir() -> std::path::PathBuf {
        #[cfg(target_os = "macos")]
        {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
            std::path::PathBuf::from(format!(
                "{}/Library/Application Support/com.voxtype.app/models",
                home
            ))
        }

        #[cfg(target_os = "windows")]
        {
            let appdata = std::env::var("APPDATA").unwrap_or_else(|_| "C:\\".into());
            std::path::PathBuf::from(format!("{}\\VoxType\\models", appdata))
        }

        #[cfg(target_os = "linux")]
        {
            let data = std::env::var("XDG_DATA_HOME").unwrap_or_else(|_| {
                let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
                format!("{}/.local/share", home)
            });
            std::path::PathBuf::from(format!("{}/voxtype/models", data))
        }
    }

    /// Find an available model file
    fn find_model() -> Option<std::path::PathBuf> {
        let dir = Self::model_dir();
        let candidates = &[
            "ggml-small.bin",
            "ggml-base.bin",
            "ggml-tiny.bin",
            "ggml-medium.bin",
            "ggml-large-v3-turbo.bin",
        ];

        for name in candidates {
            let path = dir.join(name);
            if path.exists() {
                tracing::info!("Found whisper model: {}", path.display());
                return Some(path);
            }
        }

        None
    }
}

#[async_trait]
impl SttProvider for LocalWhisperProvider {
    fn name(&self) -> &str { "Local Whisper" }

    fn is_available(&self) -> bool {
        Self::find_model().is_some()
    }

    fn supported_languages(&self) -> Vec<&str> {
        vec!["zh", "en", "ja", "ko", "de", "fr", "es", "pt", "ru", "ar", "hi"]
    }

    async fn transcribe(&self, audio_data: &[u8], _format: AudioFormat, language: Option<&str>) -> Result<String> {
        let whisper_bin = Self::whisper_binary()
            .ok_or_else(|| VoxTypeError::Stt(
                "whisper.cpp not found. Install whisper.cpp or download from GitHub.".to_string()
            ))?;

        let model_path = Self::find_model()
            .ok_or_else(|| VoxTypeError::Stt(
                "No whisper model found. Download a model in Settings → STT → Local Whisper.".to_string()
            ))?;

        let lang = language.unwrap_or("auto");

        // Write audio to temp file
        let mut tmp_wav = NamedTempFile::new()
            .map_err(|e| VoxTypeError::Stt(format!("Failed to create temp file: {}", e)))?;
        tmp_wav.write_all(audio_data)
            .map_err(|e| VoxTypeError::Stt(format!("Failed to write temp audio: {}", e)))?;
        let wav_path = tmp_wav.path().to_string_lossy().to_string();

        // Output path
        let output_base = tmp_wav.path().with_extension("").to_string_lossy().to_string();

        tracing::info!(
            "Running whisper: {} -m {} -l {} -f {} -otxt -of {}",
            whisper_bin, model_path.display(), lang, wav_path, output_base
        );

        let whisper_bin_clone = whisper_bin.clone();
        let model_path_str = model_path.to_string_lossy().to_string();
        let lang_owned = lang.to_string();
        let wav_path_clone = wav_path.clone();
        let output_base_clone = output_base.clone();

        let output = task::spawn_blocking(move || {
            Command::new(&whisper_bin_clone)
                .arg("-m").arg(&model_path_str)
                .arg("-l").arg(&lang_owned)
                .arg("-f").arg(&wav_path_clone)
                .arg("-otxt")
                .arg("-of").arg(&output_base_clone)
                .arg("--no-timestamps")
                .output()
        })
        .await
        .map_err(|e| VoxTypeError::Stt(format!("Whisper task panicked: {}", e)))?
        .map_err(|e| VoxTypeError::Stt(format!("Failed to run whisper: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            tracing::error!("Whisper stderr: {}", stderr);
            return Err(VoxTypeError::Stt(format!(
                "Whisper process failed: {}",
                &stderr.lines().last().unwrap_or("unknown error")
            )));
        }

        // Read output file
        let txt_path = format!("{}.txt", output_base);
        let text = std::fs::read_to_string(&txt_path)
            .map_err(|e| VoxTypeError::Stt(format!("Failed to read whisper output: {}", e)))?;

        // Cleanup temp files
        let _ = std::fs::remove_file(&wav_path);
        let _ = std::fs::remove_file(&txt_path);

        Ok(text.trim().to_string())
    }
}
