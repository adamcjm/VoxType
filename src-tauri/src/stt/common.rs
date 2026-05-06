/// Common HTTP logic for OpenAI-compatible Whisper API providers.

use crate::error::{Result, VoxTypeError};
use reqwest::{multipart, Client};

/// POST audio to an OpenAI-compatible /audio/transcriptions endpoint.
pub async fn transcribe_openai_compat(
    client: &Client,
    base_url: &str,
    api_key: &str,
    model: &str,
    audio_data: &[u8],
    mime_type: &str,
    extension: &str,
    language: Option<&str>,
) -> Result<String> {
    let url = format!("{}/audio/transcriptions", base_url.trim_end_matches('/'));

    let file_part = multipart::Part::bytes(audio_data.to_vec())
        .file_name(format!("audio.{}", extension))
        .mime_str(mime_type)
        .map_err(|e| VoxTypeError::Stt(format!("Failed to build file part: {}", e)))?;

    let mut form = multipart::Form::new()
        .part("file", file_part)
        .text("model", model.to_string())
        .text("response_format", "json".to_string());

    if let Some(lang) = language {
        form = form.text("language", lang.to_string());
    }

    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .multipart(form)
        .send()
        .await
        .map_err(|e| VoxTypeError::Network(e))?;

    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(VoxTypeError::Stt(format!(
            "STT API error ({}): {}",
            status.as_u16(),
            body
        )));
    }

    let json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| VoxTypeError::Stt(format!("Failed to parse STT response: {}", e)))?;

    json["text"]
        .as_str()
        .map(|s| s.trim().to_string())
        .ok_or_else(|| {
            VoxTypeError::Stt(format!(
                "Unexpected STT response format. Got: {}",
                json
            ))
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, path, header};

    #[tokio::test]
    async fn test_transcribe_openai_compat_success() {
        let server = MockServer::start().await;
        let client = Client::new();

        Mock::given(method("POST"))
            .and(path("/audio/transcriptions"))
            .and(header("Authorization", "Bearer test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::json!({"text": "Hello world"})
            ))
            .expect(1)
            .mount(&server)
            .await;

        let result = transcribe_openai_compat(
            &client,
            &server.uri(),
            "test-key",
            "whisper-v3",
            b"fake wav data",
            "audio/wav",
            "wav",
            Some("en"),
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello world");
    }

    #[tokio::test]
    async fn test_transcribe_openai_compat_auth_error() {
        let server = MockServer::start().await;
        let client = Client::new();

        Mock::given(method("POST"))
            .and(path("/audio/transcriptions"))
            .respond_with(ResponseTemplate::new(401).set_body_string("Unauthorized"))
            .expect(1)
            .mount(&server)
            .await;

        let result = transcribe_openai_compat(
            &client,
            &server.uri(),
            "bad-key",
            "whisper-v3",
            b"fake data",
            "audio/wav",
            "wav",
            None,
        )
        .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_transcribe_openai_compat_with_language() {
        let server = MockServer::start().await;
        let client = Client::new();

        Mock::given(method("POST"))
            .and(path("/audio/transcriptions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::json!({"text": "你好世界"})
            ))
            .expect(1)
            .mount(&server)
            .await;

        let result = transcribe_openai_compat(
            &client,
            &server.uri(),
            "key",
            "whisper-v3",
            b"audio",
            "audio/wav",
            "wav",
            Some("zh"),
        )
        .await;

        assert_eq!(result.unwrap(), "你好世界");
    }
}
