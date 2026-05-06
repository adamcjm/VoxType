use tauri::State;
use std::sync::Arc;
use crate::state::AppState;
use crate::audio::{VadDetector, AudioPreprocessor, capture::encode_wav};
use crate::commands::history::HistoryItem;
use tracing;

#[tauri::command]
pub async fn toggle_recording(
    state: State<'_, Arc<AppState>>,
) -> Result<String, String> {
    let is_rec = {
        let app = state.inner();
        let capture = app.audio_capture.lock().map_err(|e| format!("Lock error: {}", e))?;
        capture.is_recording()
    };

    if is_rec {
        stop_recording(state).await
    } else {
        start_recording(state).await?;
        Ok("recording".to_string())
    }
}

#[tauri::command]
pub async fn start_recording(
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    tracing::info!("Start recording command received");

    let app = state.inner();
    let mut capture = app.audio_capture
        .lock()
        .map_err(|e| format!("Lock error: {}", e))?;

    capture.start().map_err(|e| format!("Failed to start recording: {}", e))?;
    Ok(())
}

#[tauri::command]
pub async fn stop_recording(
    state: State<'_, Arc<AppState>>,
) -> Result<String, String> {
    tracing::info!("Stop recording command received");

    let app = state.inner();

    // Step 1: Stop capture and get samples
    let (samples, duration_ms) = {
        let mut capture = app.audio_capture
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        capture.stop()
            .map_err(|e| format!("Failed to stop recording: {}", e))?
    };

    // Step 2: VAD
    let vad = VadDetector::default();
    if !vad.has_speech(&samples) {
        return Err("No speech detected. Please speak into the microphone.".to_string());
    }

    // Step 3: Preprocessing
    let preprocessor = AudioPreprocessor::new();
    let mut processed_samples = samples;
    preprocessor.process(&mut processed_samples);

    // Step 4: Encode to WAV
    let wav_data = encode_wav(&processed_samples, 16000, 1);

    // Step 5: Get config and build managers
    let (stt_config, llm_config, translate_config) = {
        let config = app.config.lock().map_err(|e| format!("Lock error: {}", e))?;
        (
            config.stt.clone(),
            config.llm.clone(),
            config.translate.clone(),
        )
    };

    let stt_manager = crate::stt::SttManager::new(stt_config.clone());
    let polish_enabled = !llm_config.api_key.is_empty();
    let llm_manager = crate::llm::LlmManager::new(llm_config.clone());

    let translate_opt: Option<(String, String)> = if translate_config.enabled {
        Some((translate_config.source_lang, translate_config.target_lang))
    } else {
        None
    };

    // Step 6: Run pipeline (async, no locks held)
    let result = crate::pipeline::run_pipeline(
        &wav_data,
        &stt_manager,
        &llm_manager,
        polish_enabled,
        translate_opt.as_ref().map(|(s, t)| (s.as_str(), t.as_str())),
    )
    .await
    .map_err(|e| format!("Pipeline error: {}", e))?;

    tracing::info!(
        "Pipeline complete: raw={} chars, final={} chars",
        result.raw_text.len(),
        result.final_text.len()
    );

    // Step 7: Save to history
    let history_item = HistoryItem {
        id: uuid::Uuid::new_v4().to_string(),
        raw_text: result.raw_text,
        final_text: result.final_text.clone(),
        stt_provider: result.stt_provider,
        llm_provider: result.llm_provider,
        app_name: "Unknown".to_string(),
        duration_ms,
        mode: result.mode,
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    match app.history.add(&history_item) {
        Ok(()) => tracing::info!("History saved"),
        Err(e) => tracing::warn!("Failed to save history: {}", e),
    }

    Ok(result.final_text)
}

#[tauri::command]
pub fn list_audio_devices() -> Result<Vec<crate::audio::device::AudioDeviceInfo>, String> {
    Ok(crate::audio::device::list_input_devices())
}
