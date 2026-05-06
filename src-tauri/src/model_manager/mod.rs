/// Whisper model download and management.
use crate::error::{Result, VoxTypeError};
use std::path::PathBuf;
use std::fs;
use reqwest::Client;
use tokio::io::AsyncWriteExt;
use tracing;

#[derive(Debug, Clone)]
pub struct ModelInfo {
    pub name: String,
    pub size: ModelSize,
    pub filename: String,
    pub approx_size_mb: u64,
    pub quality: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ModelSize {
    Tiny,
    Base,
    Small,
    Medium,
    Large,
}

impl ModelSize {
    pub fn hf_filename(&self) -> &str {
        match self {
            ModelSize::Tiny => "ggml-tiny.bin",
            ModelSize::Base => "ggml-base.bin",
            ModelSize::Small => "ggml-small.bin",
            ModelSize::Medium => "ggml-medium.bin",
            ModelSize::Large => "ggml-large-v3.bin",
        }
    }
}

pub fn list_available_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo { name: "Tiny".into(), size: ModelSize::Tiny, filename: "ggml-tiny.bin".into(), approx_size_mb: 78, quality: "Fast, lower accuracy" },
        ModelInfo { name: "Base".into(), size: ModelSize::Base, filename: "ggml-base.bin".into(), approx_size_mb: 148, quality: "Balanced" },
        ModelInfo { name: "Small".into(), size: ModelSize::Small, filename: "ggml-small.bin".into(), approx_size_mb: 466, quality: "Recommended" },
        ModelInfo { name: "Medium".into(), size: ModelSize::Medium, filename: "ggml-medium.bin".into(), approx_size_mb: 1530, quality: "High accuracy" },
        ModelInfo { name: "Large v3".into(), size: ModelSize::Large, filename: "ggml-large-v3.bin".into(), approx_size_mb: 3100, quality: "Best accuracy" },
    ]
}

/// Check if a specific model file exists on disk
pub fn model_exists(size: &ModelSize) -> bool {
    model_path(size).exists()
}

/// Get the path where a model file should be stored
pub fn model_path(size: &ModelSize) -> PathBuf {
    let dir = model_dir();
    dir.join(size.hf_filename())
}

/// Get the whisper model directory
fn model_dir() -> PathBuf {
    crate::paths::models_dir()
}

/// Download a whisper model from HuggingFace.
/// Returns the final file path on success.
pub async fn download_model(size: ModelSize) -> Result<PathBuf> {
    let dir = model_dir();
    fs::create_dir_all(&dir).map_err(|e|
        VoxTypeError::Stt(format!("Failed to create model directory: {}", e))
    )?;

    let url = format!(
        "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/{}",
        size.hf_filename()
    );

    let dest = model_path(&size);
    let tmp = dest.with_extension("download");

    tracing::info!("Downloading model {} from {}", size.hf_filename(), url);

    let client = Client::new();
    let response = client.get(&url).send().await.map_err(|e|
        VoxTypeError::Stt(format!("Failed to start model download: {}", e))
    )?;

    if !response.status().is_success() {
        return Err(VoxTypeError::Stt(format!(
            "Model download failed ({}): Model '{}' not found.",
            response.status().as_u16(),
            size.hf_filename()
        )));
    }

    let total = response.content_length().unwrap_or(0);
    let mut downloaded: u64 = 0;
    let mut file = tokio::fs::File::create(&tmp).await.map_err(|e|
        VoxTypeError::Stt(format!("Failed to create download file: {}", e))
    )?;

    let mut stream = response.bytes_stream();

    use futures_util::StreamExt;
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e|
            VoxTypeError::Stt(format!("Download error: {}", e))
        )?;
        file.write_all(&chunk).await.map_err(|e|
            VoxTypeError::Stt(format!("Write error: {}", e))
        )?;
        downloaded += chunk.len() as u64;

        if total > 0 && downloaded % (1024 * 1024 * 5) == 0 {
            let pct = (downloaded as f64 / total as f64 * 100.0) as u32;
            tracing::info!("Model download: {}% ({:.1}MB / {:.1}MB)",
                pct, downloaded as f64 / 1e6, total as f64 / 1e6
            );
        }
    }

    file.flush().await.map_err(|e|
        VoxTypeError::Stt(format!("Flush error: {}", e))
    )?;

    // Atomic rename
    fs::rename(&tmp, &dest).map_err(|e|
        VoxTypeError::Stt(format!("Failed to finalize model file: {}", e))
    )?;

    tracing::info!("Model downloaded to {}", dest.display());
    Ok(dest)
}

/// Delete a model file
pub fn delete_model(size: ModelSize) -> Result<()> {
    let path = model_path(&size);
    if path.exists() {
        fs::remove_file(&path).map_err(|e|
            VoxTypeError::Stt(format!("Failed to delete model: {}", e))
        )?;
    }
    Ok(())
}

/// Get size of downloaded model in MB
pub fn model_size_mb(size: &ModelSize) -> Option<f64> {
    model_path(size).metadata().ok().map(|m| m.len() as f64 / 1e6)
}
