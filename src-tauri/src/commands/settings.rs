use crate::config::AppConfig;
use std::sync::Arc;
use tauri::State;
use crate::state::AppState;

#[tauri::command]
pub async fn get_settings(state: State<'_, Arc<AppState>>) -> Result<AppConfig, String> {
    let config = state.inner().config.lock().map_err(|e| e.to_string())?;
    Ok(config.clone())
}

#[tauri::command]
pub async fn save_settings(
    state: State<'_, Arc<AppState>>,
    config: AppConfig,
) -> Result<(), String> {
    // Save to disk first
    config.save().map_err(|e| format!("Failed to persist settings: {}", e))?;

    // Update in-memory
    let mut current = state.inner().config.lock().map_err(|e| e.to_string())?;
    *current = config;

    tracing::info!("Settings saved and persisted");
    Ok(())
}

#[tauri::command]
pub async fn needs_onboarding(state: State<'_, Arc<AppState>>) -> Result<bool, String> {
    let config = state.inner().config.lock().map_err(|e| e.to_string())?;
    Ok(config.needs_onboarding())
}
