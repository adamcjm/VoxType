use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryItem {
    pub id: String,
    pub raw_text: String,
    pub final_text: String,
    pub stt_provider: String,
    pub llm_provider: String,
    pub app_name: String,
    pub duration_ms: u64,
    pub mode: String,
    pub created_at: String,
}

#[tauri::command]
pub async fn get_history() -> Result<Vec<HistoryItem>, String> {
    Ok(vec![])
}

#[tauri::command]
pub async fn add_history_item(_item: HistoryItem) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub async fn remove_history_item(_id: String) -> Result<(), String> {
    Ok(())
}
