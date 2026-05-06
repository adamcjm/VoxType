/// Basic audio preprocessing: noise gate and gain normalization
pub struct AudioPreprocessor {
    /// Amplitude below this is considered noise and zeroed out
    noise_gate_threshold: f32,
    /// Target peak amplitude (0.0 - 1.0)
    target_peak: f32,
}

impl Default for AudioPreprocessor {
    fn default() -> Self {
        Self {
            noise_gate_threshold: 0.005,
            target_peak: 0.8,
        }
    }
}

impl AudioPreprocessor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_noise_gate(mut self, threshold: f32) -> Self {
        self.noise_gate_threshold = threshold;
        self
    }

    pub fn with_target_peak(mut self, target: f32) -> Self {
        self.target_peak = target.clamp(0.1, 1.0);
        self
    }

    /// Apply noise gate: zero out samples below threshold
    pub fn apply_noise_gate(&self, samples: &mut [f32]) {
        for sample in samples.iter_mut() {
            if sample.abs() < self.noise_gate_threshold {
                *sample = 0.0;
            }
        }
    }

    /// Normalize gain to target peak amplitude
    pub fn normalize_gain(&self, samples: &mut [f32]) {
        if samples.is_empty() {
            return;
        }

        let peak = samples
            .iter()
            .map(|&s| s.abs())
            .fold(0.0f32, f32::max);

        if peak > 0.0 && peak < self.target_peak {
            let gain = self.target_peak / peak;
            for sample in samples.iter_mut() {
                *sample *= gain;
            }
        }
    }

    /// Full preprocessing pipeline: noise gate → gain normalization
    pub fn process(&self, samples: &mut [f32]) {
        self.apply_noise_gate(samples);
        self.normalize_gain(samples);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noise_gate_strips_quiet_samples() {
        let preprocessor = AudioPreprocessor::default();
        let mut samples = vec![0.001f32, 0.01, 0.002, 0.1, -0.003, -0.5];
        preprocessor.apply_noise_gate(&mut samples);
        assert_eq!(samples[0], 0.0);
        assert_eq!(samples[2], 0.0);
        assert_eq!(samples[3], 0.1); // Above threshold
        assert_eq!(samples[4], 0.0);
        assert_eq!(samples[5], -0.5); // Above threshold
    }

    #[test]
    fn test_normalize_gain_amplifies_quiet() {
        let preprocessor = AudioPreprocessor::default();
        let mut samples = vec![0.0f32; 100];
        samples[50] = 0.2; // Peak is 0.2, target is 0.8
        preprocessor.normalize_gain(&mut samples);
        assert!((samples[50] - 0.8).abs() < 0.01);
    }

    #[test]
    fn test_process_empty() {
        let preprocessor = AudioPreprocessor::default();
        let mut samples: Vec<f32> = vec![];
        preprocessor.process(&mut samples);
        assert!(samples.is_empty());
    }

    #[test]
    fn test_normalize_gain_no_change_if_loud() {
        let preprocessor = AudioPreprocessor::default();
        let mut samples = vec![0.9f32, -0.9];
        preprocessor.normalize_gain(&mut samples);
        // Peak is 0.9 > target 0.8, should not amplify
        assert!((samples[0] - 0.9).abs() < 0.01);
    }
}
