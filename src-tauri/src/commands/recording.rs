use tauri::State;
use std::sync::Arc;
use crate::state::AppState;

#[tauri::command]
pub async fn start_recording(_state: State<'_, Arc<AppState>>) -> Result<(), String> {
    tracing::info!("Start recording command received");
    Ok(())
}

#[tauri::command]
pub async fn stop_recording(_state: State<'_, Arc<AppState>>) -> Result<String, String> {
    tracing::info!("Stop recording command received");
    Ok("Transcription placeholder".to_string())
}
