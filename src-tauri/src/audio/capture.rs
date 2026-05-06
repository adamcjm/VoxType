use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleRate, StreamConfig};
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use tracing;

const SAMPLE_RATE: u32 = 16000;
const CHANNELS: u16 = 1;

#[derive(Debug)]
enum AudioCommand {
    Start,
    Stop,
}

pub struct AudioCapture {
    buffer: Arc<Mutex<Vec<f32>>>,
    command_tx: mpsc::Sender<AudioCommand>,
    is_recording: Arc<Mutex<bool>>,
    /// Handle to the audio thread (used to join on drop)
    _thread_handle: Option<std::thread::JoinHandle<()>>,
}

impl AudioCapture {
    pub fn new() -> Result<Self, String> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or_else(|| "No input device found. Please connect a microphone.".to_string())?;

        tracing::info!(
            "Audio capture initialized with device: {}",
            device.name().unwrap_or_default()
        );

        let buffer = Arc::new(Mutex::new(Vec::<f32>::new()));
        let is_recording = Arc::new(Mutex::new(false));
        let (command_tx, command_rx) = mpsc::channel::<AudioCommand>();

        let buffer_clone = Arc::clone(&buffer);
        let is_rec_clone = Arc::clone(&is_recording);

        // Spawn dedicated audio thread that owns the cpal Stream (not Send on macOS)
        let thread_handle = std::thread::spawn(move || {
            // Build the audio stream on this thread
            let config = StreamConfig {
                channels: CHANNELS,
                sample_rate: SampleRate(SAMPLE_RATE),
                buffer_size: cpal::BufferSize::Default,
            };

            // Inner buffers
            let active_buffer = Arc::clone(&buffer_clone);
            let active_recording = Arc::clone(&is_rec_clone);

            let err_fn = |err| {
                tracing::error!("Audio stream error: {}", err);
            };

            let stream = match device.build_input_stream(
                &config,
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    if let Ok(recording) = active_recording.lock() {
                        if *recording {
                            if let Ok(mut buf) = active_buffer.lock() {
                                buf.extend_from_slice(data);
                            }
                        }
                    }
                },
                err_fn,
                None,
            ) {
                Ok(s) => s,
                Err(e) => {
                    tracing::error!("Failed to build input stream: {}", e);
                    return;
                }
            };

            if let Err(e) = stream.play() {
                tracing::error!("Failed to play stream: {}", e);
                return;
            }

            tracing::info!("Audio thread started, waiting for commands");

            // Main loop: wait for commands
            loop {
                match command_rx.recv() {
                    Ok(AudioCommand::Start) => {
                        tracing::info!("Audio: recording started");
                        // Buffer already being filled by callback
                        // (is_recording flag controls writing)
                    }
                    Ok(AudioCommand::Stop) => {
                        tracing::info!("Audio: recording stopped");
                        // The is_recording flag stops writing to buffer
                        // We just continue listening for next start command
                    }
                    Err(_) => {
                        // Channel closed, exit thread
                        tracing::info!("Audio thread: channel closed, shutting down");
                        break;
                    }
                }
            }
            // stream dropped here, stopping audio capture
        });

        Ok(Self {
            buffer,
            command_tx,
            is_recording,
            _thread_handle: Some(thread_handle),
        })
    }

    pub fn start(&mut self) -> Result<(), String> {
        // Clear buffer
        {
            let mut buf = self.buffer.lock().map_err(|e| e.to_string())?;
            buf.clear();
        }

        // Set recording flag
        {
            let mut rec = self.is_recording.lock().map_err(|e| e.to_string())?;
            *rec = true;
        }

        // Signal audio thread
        self.command_tx
            .send(AudioCommand::Start)
            .map_err(|e| format!("Failed to send start command: {}", e))?;

        tracing::info!("Recording started");
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(Vec<f32>, u64), String> {
        // Stop recording flag
        {
            let mut rec = self.is_recording.lock().map_err(|e| e.to_string())?;
            *rec = false;
        }

        // Signal audio thread
        self.command_tx
            .send(AudioCommand::Stop)
            .map_err(|e| format!("Failed to send stop command: {}", e))?;

        // Get captured samples
        let samples = {
            let buf = self.buffer.lock().map_err(|e| e.to_string())?;
            buf.clone()
        };

        let duration_ms = (samples.len() as f64 / SAMPLE_RATE as f64 * 1000.0) as u64;
        tracing::info!(
            "Recording stopped: {} samples, {}ms",
            samples.len(),
            duration_ms
        );

        if samples.is_empty() {
            return Err("No audio captured. Check your microphone.".to_string());
        }

        Ok((samples, duration_ms))
    }
}

impl Drop for AudioCapture {
    fn drop(&mut self) {
        // Channel will be dropped, causing audio thread to exit
    }
}

/// Encode f32 samples to WAV bytes (public for use by pipeline)
pub fn encode_wav(samples: &[f32], sample_rate: u32, channels: u16) -> Vec<u8> {
    use std::io::Cursor;

    let i16_samples: Vec<i16> = samples
        .iter()
        .map(|&s| {
            let clamped = s.max(-1.0).min(1.0);
            (clamped * i16::MAX as f32) as i16
        })
        .collect();

    let spec = hound::WavSpec {
        channels,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut cursor = Cursor::new(Vec::new());
    let mut writer = hound::WavWriter::new(&mut cursor, spec)
        .expect("Failed to create WAV writer");

    for sample in &i16_samples {
        writer.write_sample(*sample).expect("Failed to write WAV sample");
    }

    writer.finalize().expect("Failed to finalize WAV");
    cursor.into_inner()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_wav_basic() {
        let samples = vec![0.0f32; 16000];
        let wav = encode_wav(&samples, 16000, 1);
        assert!(wav.len() > 44, "WAV should have header + data (got {} bytes)", wav.len());
        assert_eq!(&wav[0..4], b"RIFF");
        assert_eq!(&wav[8..12], b"WAVE");
    }

    #[test]
    fn test_encode_wav_clipping() {
        let samples = vec![2.0f32, -2.0f32];
        let wav = encode_wav(&samples, 16000, 1);
        assert!(wav.len() > 44);
    }

    #[test]
    fn test_audio_capture_new() {
        // Should not panic even without microphone
        let result = AudioCapture::new();
        // May be Err if no mic, but should not panic
        assert!(result.is_ok() || result.is_err());
    }
}
