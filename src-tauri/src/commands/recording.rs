use tauri::State;
use std::sync::Arc;
use crate::state::AppState;
use crate::audio::{VadDetector, AudioPreprocessor, capture::encode_wav};
use crate::pipeline;
use tracing;

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

    // Step 1: Stop capture and get samples
    let (samples, _duration_ms) = {
        let app = state.inner();
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

    // Step 5: Get config and build managers (all sync, drop lock before async)
    let (stt_config, llm_config, translate_config) = {
        let app = state.inner();
        let config = app.config.lock().map_err(|e| format!("Lock error: {}", e))?;
        (
            config.stt.clone(),
            config.llm.clone(),
            config.translate.clone(),
        )
    };

    let stt_manager = crate::stt::SttManager::new(stt_config);
    let polish_enabled = !llm_config.api_key.is_empty();
    let llm_manager = crate::llm::LlmManager::new(llm_config);

    let translate_opt = if translate_config.enabled {
        Some((translate_config.source_lang, translate_config.target_lang))
    } else {
        None
    };

    // Step 6: Run pipeline (async, no locks held)
    let (raw_text, final_text) = pipeline::run_pipeline(
        &wav_data,
        &stt_manager,
        &llm_manager,
        polish_enabled,
        translate_opt.as_ref().map(|(s, t)| (s.as_str(), t.as_str())),
    ).await.map_err(|e| format!("Pipeline error: {}", e))?;

    tracing::info!("Pipeline complete: raw={} chars, final={} chars",
        raw_text.len(), final_text.len());

    Ok(final_text)
}

#[tauri::command]
pub fn list_audio_devices() -> Result<Vec<crate::audio::device::AudioDeviceInfo>, String> {
    Ok(crate::audio::device::list_input_devices())
}
