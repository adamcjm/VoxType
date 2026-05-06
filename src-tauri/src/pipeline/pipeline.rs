use crate::error::Result;
use crate::stt::SttManager;
use crate::llm::{LlmManager, PolishMode};
use crate::output;
use tracing;

/// Result of the recording pipeline
pub struct PipelineResult {
    pub raw_text: String,
    pub final_text: String,
    pub stt_provider: String,
    pub llm_provider: String,
    pub mode: String,
}

/// Process audio through the full pipeline: STT → LLM Polish → Output
pub async fn run_pipeline(
    audio_data: &[u8],
    stt_manager: &SttManager,
    llm_manager: &LlmManager,
    polish_enabled: bool,
    translate_config: Option<(&str, &str)>,
) -> Result<PipelineResult> {
    // Step 1: STT - transcribe audio
    tracing::info!("Pipeline: Starting STT transcription");
    let raw_text = stt_manager.transcribe(audio_data).await?;
    tracing::info!("Pipeline: STT complete, raw text length: {}", raw_text.len());

    let stt_provider = "groq".to_string(); // TODO: get from config
    let llm_provider = "deepseek".to_string();

    // Step 2: LLM Polish (optional)
    let (final_text, mode) = if polish_enabled {
        tracing::info!("Pipeline: Starting LLM polish");
        let polish_mode = match translate_config {
            Some((src, tgt)) if tgt != src => PolishMode::Translate {
                source_lang: src.to_string(),
                target_lang: tgt.to_string(),
            },
            _ => PolishMode::Cleanup,
        };
        let mode_str = match &polish_mode {
            PolishMode::Cleanup => "cleanup",
            PolishMode::Translate { .. } => "translate",
            PolishMode::Format => "format",
        };
        let text = llm_manager.polish(&raw_text, polish_mode).await?;
        (text, mode_str.to_string())
    } else {
        (raw_text.clone(), "cleanup".to_string())
    };
    tracing::info!("Pipeline: Polish complete, final text length: {}", final_text.len());

    // Step 3: Output - type at cursor
    tracing::info!("Pipeline: Outputting text");
    output::write_text(&final_text).await?;

    Ok(PipelineResult {
        raw_text,
        final_text,
        stt_provider,
        llm_provider,
        mode,
    })
}
