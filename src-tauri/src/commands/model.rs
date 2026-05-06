use serde::{Deserialize, Serialize};
use crate::model_manager::{self, ModelSize};

// ── Model Manager (Whisper download/delete) ──

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
            ModelStatus { name: m.name, size: format!("{:?}", m.size), filename: m.filename, approx_size_mb: m.approx_size_mb, quality: m.quality.to_string(), downloaded, actual_size_mb: actual }
        }).collect()
}

#[tauri::command]
pub async fn download_model(size_str: String) -> Result<String, String> {
    let size = parse_size(&size_str)?;
    let path = model_manager::download_model(size).await.map_err(|e| e.to_string())?;
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
        "tiny" => Ok(ModelSize::Tiny), "base" => Ok(ModelSize::Base),
        "small" => Ok(ModelSize::Small), "medium" => Ok(ModelSize::Medium),
        "large" => Ok(ModelSize::Large),
        o => Err(format!("Unknown model size: {}. Use tiny/base/small/medium/large.", o)),
    }
}

// ── API Model Fetching ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub display_name: String,
}

/// Fetch available STT models from a provider.
/// Uses OpenAI-compatible /models endpoint.
#[tauri::command]
pub async fn fetch_stt_models(
    provider: String,
    base_url: String,
    api_key: String,
) -> Result<Vec<ModelInfo>, String> {
    let url = format!("{}/models", base_url.trim_end_matches('/'));

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .map_err(|e| format!("Failed to fetch models: {}", e))?;

    if !response.status().is_success() {
        let status = response.status().as_u16();
        let body = response.text().await.unwrap_or_default();
        // Return hardcoded list as fallback
        tracing::warn!("STT models API error ({}): {}", status, &body[..body.len().min(200)]);
        return Ok(hardcoded_stt_models(&provider));
    }

    let json: serde_json::Value = response.json().await
        .map_err(|e| format!("Failed to parse models response: {}", e))?;

    let models = if let Some(data) = json["data"].as_array() {
        data.iter()
            .filter_map(|m| m["id"].as_str())
            .filter(|id| id.contains("whisper") || id.contains("nova") || id.contains("stt"))
            .map(|id| ModelInfo {
                display_name: id.to_string(),
                id: id.to_string(),
            })
            .collect()
    } else {
        Vec::new()
    };

    if models.is_empty() {
        Ok(hardcoded_stt_models(&provider))
    } else {
        Ok(models)
    }
}

/// Fetch available LLM models from a provider.
/// Uses OpenAI-compatible /models endpoint.
#[tauri::command]
pub async fn fetch_llm_models(
    base_url: String,
    api_key: String,
) -> Result<Vec<ModelInfo>, String> {
    let url = format!("{}/models", base_url.trim_end_matches('/'));

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .map_err(|e| format!("Failed to fetch models: {}", e))?;

    if !response.status().is_success() {
        let body = response.text().await.unwrap_or_default();
        tracing::warn!("LLM models API error: {}", &body[..body.len().min(200)]);
        return Ok(hardcoded_llm_models());
    }

    let json: serde_json::Value = response.json().await
        .map_err(|e| format!("Failed to parse models response: {}", e))?;

    let models: Vec<ModelInfo> = if let Some(data) = json["data"].as_array() {
        data.iter()
            .filter_map(|m| m["id"].as_str())
            .filter(|id| !id.starts_with("dall-e") && !id.starts_with("tts-") && !id.starts_with("whisper") && !id.starts_with("text-moderation"))
            .map(|id| ModelInfo {
                display_name: id.to_string(),
                id: id.to_string(),
            })
            .take(30)
            .collect()
    } else {
        Vec::new()
    };

    if models.is_empty() {
        Ok(hardcoded_llm_models())
    } else {
        Ok(models)
    }
}

fn hardcoded_stt_models(provider: &str) -> Vec<ModelInfo> {
    match provider {
        "groq" => vec![
            model("whisper-large-v3-turbo"),
            model("whisper-large-v3"),
            model("distil-whisper-large-v3-en"),
        ],
        "openai" => vec![model("whisper-1")],
        "deepgram" => vec![
            model("nova-2"),
            model("nova-3"),
            model("whisper"),
        ],
        "local" => vec![
            model("ggml-tiny.bin"),
            model("ggml-base.bin"),
            model("ggml-small.bin"),
            model("ggml-medium.bin"),
            model("ggml-large-v3.bin"),
        ],
        _ => vec![model("whisper-large-v3-turbo")],
    }
}

fn hardcoded_llm_models() -> Vec<ModelInfo> {
    vec![
        model("deepseek-chat"),
        model("deepseek-reasoner"),
        model("gpt-4o-mini"),
        model("gpt-4o"),
        model("gemini-2.0-flash"),
        model("gemini-1.5-pro"),
        model("llama-3.3-70b-versatile"),
        model("claude-3-haiku-20240307"),
    ]
}

fn model(id: &str) -> ModelInfo {
    ModelInfo { id: id.into(), display_name: id.into() }
}
