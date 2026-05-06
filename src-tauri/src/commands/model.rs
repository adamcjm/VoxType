use crate::model_manager::{self, ModelSize};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ModelStatus {
    pub name: String,
    pub size: String,
    pub filename: String,
    pub approx_size_mb: u64,
    pub quality: String,
    pub downloaded: bool,
    pub actual_size_mb: Option<f64>,
}

#[tauri::command]
pub fn list_models() -> Vec<ModelStatus> {
    model_manager::list_available_models()
        .into_iter()
        .map(|m| {
            let downloaded = model_manager::model_exists(&m.size);
            let actual = if downloaded { model_manager::model_size_mb(&m.size) } else { None };
            ModelStatus {
                name: m.name,
                size: format!("{:?}", m.size),
                filename: m.filename,
                approx_size_mb: m.approx_size_mb,
                quality: m.quality.to_string(),
                downloaded,
                actual_size_mb: actual,
            }
        })
        .collect()
}

#[tauri::command]
pub async fn download_model(size_str: String) -> Result<String, String> {
    let size = parse_size(&size_str)?;
    let path = model_manager::download_model(size)
        .await
        .map_err(|e| e.to_string())?;
    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn delete_model(size_str: String) -> Result<(), String> {
    let size = parse_size(&size_str)?;
    model_manager::delete_model(size).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn model_exists(size_str: String) -> bool {
    parse_size(&size_str).map(|s| model_manager::model_exists(&s)).unwrap_or(false)
}

fn parse_size(s: &str) -> Result<ModelSize, String> {
    match s.to_lowercase().as_str() {
        "tiny" => Ok(ModelSize::Tiny),
        "base" => Ok(ModelSize::Base),
        "small" => Ok(ModelSize::Small),
        "medium" => Ok(ModelSize::Medium),
        "large" => Ok(ModelSize::Large),
        other => Err(format!("Unknown model size: {}. Use tiny/base/small/medium/large.", other)),
    }
}
