use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;
use crate::state::AppState;

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
pub async fn get_history(
    state: State<'_, Arc<AppState>>,
    search: Option<String>,
    limit: Option<usize>,
) -> Result<Vec<HistoryItem>, String> {
    state.inner().history.get_all(limit, search.as_deref())
}

#[tauri::command]
pub async fn add_history_item(
    state: State<'_, Arc<AppState>>,
    item: HistoryItem,
) -> Result<(), String> {
    state.inner().history.add(&item)
}

#[tauri::command]
pub async fn remove_history_item(
    state: State<'_, Arc<AppState>>,
    id: String,
) -> Result<(), String> {
    state.inner().history.remove(&id)
}
