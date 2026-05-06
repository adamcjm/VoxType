/// Voice Activity Detection based on energy (RMS amplitude)
///
/// Detects whether an audio buffer contains speech or silence.
/// Uses a simple energy threshold approach.
#[derive(Debug, Clone)]
pub struct VadDetector {
    /// Minimum RMS energy to be considered as speech (0.0 - 1.0)
    speech_threshold: f32,
    /// Minimum duration (in samples) of speech to trigger
    min_speech_samples: usize,
    /// Minimum duration (in samples) of silence to end speech
    min_silence_samples: usize,
    /// Sample rate for time calculations
    sample_rate: u32,
}

impl Default for VadDetector {
    fn default() -> Self {
        Self {
            speech_threshold: 0.02,
            min_speech_samples: 800,   // ~50ms at 16kHz
            min_silence_samples: 4800,  // ~300ms at 16kHz
            sample_rate: 16000,
        }
    }
}

impl VadDetector {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            sample_rate,
            ..Default::default()
        }
    }

    pub fn with_threshold(mut self, threshold: f32) -> Self {
        self.speech_threshold = threshold;
        self
    }

    /// Detect speech segments in an audio buffer.
    ///
    /// Returns a Vec of (start_sample, end_sample) pairs.
    pub fn detect_speech_segments(&self, samples: &[f32]) -> Vec<(usize, usize)> {
        if samples.is_empty() {
            return vec![];
        }

        let rms_values = self.compute_frame_rms(samples);
        let mut segments = Vec::new();
        let mut in_speech = false;
        let mut speech_start = 0usize;
        let mut silence_count = 0usize;
        let frame_size = self.sample_rate as usize / 100; // 10ms frames

        for (i, &rms) in rms_values.iter().enumerate() {
            let sample_pos = i * frame_size;

            if rms > self.speech_threshold {
                if !in_speech {
                    speech_start = sample_pos;
                    in_speech = true;
                }
                silence_count = 0;
            } else if in_speech {
                silence_count += frame_size;
                if silence_count >= self.min_silence_samples {
                    let speech_end = sample_pos - silence_count;
                    if speech_end - speech_start >= self.min_speech_samples {
                        segments.push((speech_start, speech_end));
                    }
                    in_speech = false;
                    silence_count = 0;
                }
            }
        }

        // Handle trailing speech
        if in_speech {
            let end = samples.len();
            if end - speech_start >= self.min_speech_samples {
                segments.push((speech_start, end));
            }
        }

        segments
    }

    /// Check if buffer contains speech (used for real-time detection)
    pub fn has_speech(&self, samples: &[f32]) -> bool {
        let rms = self.compute_rms(samples);
        rms > self.speech_threshold
    }

    /// Compute RMS (root mean square) energy
    pub fn compute_rms(&self, samples: &[f32]) -> f32 {
        if samples.is_empty() {
            return 0.0;
        }
        let sum_sq: f32 = samples.iter().map(|&s| s * s).sum();
        (sum_sq / samples.len() as f32).sqrt()
    }

    /// Compute RMS for overlapping frames
    fn compute_frame_rms(&self, samples: &[f32]) -> Vec<f32> {
        let frame_size = self.sample_rate as usize / 100; // 10ms frames
        samples
            .chunks(frame_size)
            .map(|chunk| self.compute_rms(chunk))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_silence_has_no_speech() {
        let vad = VadDetector::default();
        let silence = vec![0.001f32; 8000]; // Very quiet
        assert!(!vad.has_speech(&silence));
    }

    #[test]
    fn test_loud_signal_has_speech() {
        let vad = VadDetector::default();
        let speech: Vec<f32> = (0..8000)
            .map(|i| (i as f32 * 0.01).sin() * 0.5)
            .collect();
        assert!(vad.has_speech(&speech));
    }

    #[test]
    fn test_empty_buffer() {
        let vad = VadDetector::default();
        assert!(!vad.has_speech(&[]));
        assert!(vad.detect_speech_segments(&[]).is_empty());
    }

    #[test]
    fn test_detect_speech_segments() {
        let vad = VadDetector::default();
        // 2 seconds silence, 2 seconds speech, 1 second silence
        let silence1 = vec![0.001f32; 32000];
        let speech: Vec<f32> = (0..32000)
            .map(|i| (i as f32 * 0.01).sin() * 0.5)
            .collect();
        let silence2 = vec![0.001f32; 16000];

        let mut all = silence1.clone();
        all.extend(&speech);
        all.extend(&silence2);

        let segments = vad.detect_speech_segments(&all);
        assert!(!segments.is_empty(), "Should detect speech segment");
        assert!(segments[0].0 >= 32000, "Speech should start after silence");
        assert!(
            segments[0].1 <= 64000,
            "Speech should end before trailing silence"
        );
    }

    #[test]
    fn test_rms() {
        let vad = VadDetector::default();
        // Sine wave amplitude 0.5 → RMS ≈ 0.5/√2 ≈ 0.353
        let samples: Vec<f32> = (0..16000)
            .map(|i| (i as f32 * 0.01).sin() * 0.5)
            .collect();
        let rms = vad.compute_rms(&samples);
        assert!(rms > 0.3 && rms < 0.4);
    }
}
