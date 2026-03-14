use crossbeam_channel::TrySendError;
use rubato::Resampler;
use std::sync::mpsc;
use tokio_util::sync::CancellationToken;

/// VAD configuration.
#[derive(Clone, Debug)]
pub struct VadConfig {
    pub silence_ms: u32,
    pub energy_threshold: f32,
}

impl Default for VadConfig {
    fn default() -> Self {
        Self {
            silence_ms: 700,
            energy_threshold: 0.01,
        }
    }
}

impl VadConfig {
    /// Load VAD configuration from `~/.homunculus/config.toml`, falling back to defaults.
    pub fn from_config() -> Self {
        match homunculus_utils::config::HomunculusConfig::load() {
            Ok(config) => Self {
                silence_ms: config.stt.silence_ms.unwrap_or(700),
                energy_threshold: config.stt.energy_threshold.unwrap_or(0.01),
            },
            Err(_) => Self::default(),
        }
    }
}

/// VAD state machine that buffers speech and emits chunks on silence boundaries.
pub struct VadStateMachine {
    speech_buffer: Vec<f32>,
    silence_samples: usize,
    in_speech: bool,
    silence_threshold: usize,
    min_chunk_samples: usize,
    energy_threshold: f32,
    frame_i16_buf: Vec<i16>,
}

impl VadStateMachine {
    /// Create a new state machine with the given configuration.
    pub fn new(config: &VadConfig) -> Self {
        const VAD_FRAME_SIZE: usize = 320;
        Self {
            speech_buffer: Vec::new(),
            silence_samples: 0,
            in_speech: false,
            silence_threshold: (16000.0 * config.silence_ms as f64 / 1000.0).min(usize::MAX as f64)
                as usize,
            min_chunk_samples: 24_000, // 1.5s at 16kHz
            energy_threshold: config.energy_threshold,
            frame_i16_buf: vec![0i16; VAD_FRAME_SIZE],
        }
    }

    /// Process a single VAD frame. Returns a completed speech chunk when silence is detected.
    pub fn process_frame(&mut self, frame: &[f32], is_voice: bool) -> Option<Vec<f32>> {
        if is_voice {
            self.in_speech = true;
            self.silence_samples = 0;
            self.speech_buffer.extend_from_slice(frame);
            return None;
        }

        if !self.in_speech {
            return None;
        }

        self.silence_samples += frame.len();
        self.speech_buffer.extend_from_slice(frame);

        if self.silence_samples >= self.silence_threshold {
            return self.finalize_chunk();
        }

        None
    }

    /// Convert an `f32` frame to `i16` in-place, reusing the internal buffer.
    pub fn convert_frame_to_i16(&mut self, frame: &[f32]) -> &[i16] {
        self.frame_i16_buf.resize(frame.len(), 0);
        for (dst, &src) in self.frame_i16_buf.iter_mut().zip(frame.iter()) {
            *dst = (src.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
        }
        &self.frame_i16_buf
    }

    /// Take the buffered speech, reset state, and return the chunk if it meets
    /// minimum length and energy requirements.
    pub fn finalize_chunk(&mut self) -> Option<Vec<f32>> {
        let chunk = std::mem::take(&mut self.speech_buffer);
        self.in_speech = false;
        self.silence_samples = 0;

        if chunk.len() < self.min_chunk_samples {
            return None;
        }

        if rms_energy(&chunk) < self.energy_threshold {
            return None;
        }

        Some(chunk)
    }
}

/// Compute the RMS (root-mean-square) energy of a sample buffer.
pub fn rms_energy(samples: &[f32]) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }
    (samples.iter().map(|s| s * s).sum::<f32>() / samples.len() as f32).sqrt()
}

/// Convert an `f32` audio frame to `i16`, clamping values to `[-1.0, 1.0]`.
pub fn convert_f32_to_i16(frame: &[f32]) -> Vec<i16> {
    frame
        .iter()
        .map(|&s| (s.clamp(-1.0, 1.0) * i16::MAX as f32) as i16)
        .collect()
}

/// Accumulates variable-length audio samples and resamples them in fixed-size
/// chunks via `SincFixedIn`. Returns concatenated resampled output.
struct ResampleAccumulator {
    resampler: rubato::SincFixedIn<f32>,
    buf: Vec<f32>,
}

impl ResampleAccumulator {
    fn new(source_rate: u32) -> Self {
        const CHUNK_SIZE: usize = 1024;
        const NUM_CHANNELS: usize = 1;
        let resampler = rubato::SincFixedIn::<f32>::new(
            16000.0 / source_rate as f64,
            2.0,
            rubato::SincInterpolationParameters {
                sinc_len: 128,
                f_cutoff: 0.95,
                oversampling_factor: 128,
                interpolation: rubato::SincInterpolationType::Linear,
                window: rubato::WindowFunction::BlackmanHarris2,
            },
            CHUNK_SIZE,
            NUM_CHANNELS,
        )
        .expect("failed to create resampler");
        Self {
            resampler,
            buf: Vec::new(),
        }
    }

    /// Accumulate samples, drain and resample in fixed-size chunks, return
    /// all resampled output concatenated into a single buffer.
    fn push(&mut self, samples: &[f32]) -> Vec<f32> {
        self.buf.extend_from_slice(samples);
        let mut output = Vec::new();
        let n = self.resampler.input_frames_next();
        while self.buf.len() >= n {
            let chunk: Vec<f32> = self.buf.drain(..n).collect();
            match self.resampler.process(&[chunk], None) {
                Ok(resampled) => {
                    if let Some(ch) = resampled.into_iter().next() {
                        output.extend(ch);
                    }
                }
                Err(e) => tracing::error!("resample error: {e}"),
            }
        }
        output
    }
}

/// Thread 2: spawn the VAD + chunking thread.
pub fn spawn_vad_thread(
    audio_rx: mpsc::Receiver<Vec<f32>>,
    sample_rate: u32,
    needs_resample: bool,
    cancel: CancellationToken,
    config: VadConfig,
) -> Result<crossbeam_channel::Receiver<Vec<f32>>, crate::error::PipelineError> {
    let (chunk_tx, chunk_rx) = crossbeam_channel::bounded::<Vec<f32>>(1);

    std::thread::Builder::new()
        .name("stt-vad".into())
        .spawn(move || {
            vad_thread_main(
                audio_rx,
                sample_rate,
                needs_resample,
                cancel,
                config,
                chunk_tx,
            );
        })
        .map_err(|e| crate::error::PipelineError::Vad(e.to_string()))?;

    Ok(chunk_rx)
}

fn vad_thread_main(
    audio_rx: mpsc::Receiver<Vec<f32>>,
    sample_rate: u32,
    needs_resample: bool,
    cancel: CancellationToken,
    config: VadConfig,
    chunk_tx: crossbeam_channel::Sender<Vec<f32>>,
) {
    let mut accumulator = if needs_resample {
        Some(ResampleAccumulator::new(sample_rate))
    } else {
        None
    };

    let mut vad = webrtc_vad::Vad::new_with_rate_and_mode(
        webrtc_vad::SampleRate::Rate16kHz,
        webrtc_vad::VadMode::Quality,
    );

    let mut state_machine = VadStateMachine::new(&config);

    const VAD_FRAME_SIZE: usize = 320; // 20ms at 16kHz

    loop {
        if cancel.is_cancelled() {
            break;
        }

        let raw_audio = match audio_rx.recv() {
            Ok(data) => data,
            Err(_) => break,
        };

        let samples_16k = match &mut accumulator {
            Some(acc) => {
                let out = acc.push(&raw_audio);
                if out.is_empty() {
                    continue;
                }
                out
            }
            None => raw_audio,
        };

        for frame in samples_16k.chunks(VAD_FRAME_SIZE) {
            if frame.len() < VAD_FRAME_SIZE {
                continue;
            }

            let frame_i16 = state_machine.convert_frame_to_i16(frame);
            let is_voice = vad.is_voice_segment(frame_i16).unwrap_or(false);

            if let Some(chunk) = state_machine.process_frame(frame, is_voice) {
                try_send_chunk(&chunk_tx, chunk);
            }
        }
    }
}

fn try_send_chunk(tx: &crossbeam_channel::Sender<Vec<f32>>, chunk: Vec<f32>) {
    match tx.try_send(chunk) {
        Ok(()) => {}
        Err(TrySendError::Full(_)) => {
            tracing::warn!("VAD chunk channel full, dropping chunk");
        }
        Err(TrySendError::Disconnected(_)) => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignores_short_chunks() {
        let config = VadConfig {
            silence_ms: 700,
            energy_threshold: 0.01,
        };
        let mut sm = VadStateMachine::new(&config);
        let speech_frame = vec![0.5; 320];
        for _ in 0..25 {
            sm.process_frame(&speech_frame, true);
        }
        let silent_frame = vec![0.0; 320];
        let silence_frames = (16000.0 * 0.7 / 320.0) as usize + 1;
        let mut result = None;
        for _ in 0..silence_frames {
            if let Some(chunk) = sm.process_frame(&silent_frame, false) {
                result = Some(chunk);
            }
        }
        assert!(result.is_none(), "short chunk should be dropped");
    }

    #[test]
    fn emits_valid_chunk() {
        let config = VadConfig {
            silence_ms: 700,
            energy_threshold: 0.001,
        };
        let mut sm = VadStateMachine::new(&config);
        let speech_frame = vec![0.1; 320];
        for _ in 0..100 {
            sm.process_frame(&speech_frame, true);
        }
        let silent_frame = vec![0.0; 320];
        let silence_frames = (16000.0 * 0.7 / 320.0) as usize + 1;
        let mut result = None;
        for _ in 0..silence_frames {
            if let Some(chunk) = sm.process_frame(&silent_frame, false) {
                result = Some(chunk);
            }
        }
        assert!(result.is_some(), "valid chunk should be emitted");
    }

    #[test]
    fn drops_low_energy() {
        let config = VadConfig {
            silence_ms: 700,
            energy_threshold: 0.5,
        };
        let mut sm = VadStateMachine::new(&config);
        let quiet_frame = vec![0.001; 320];
        for _ in 0..100 {
            sm.process_frame(&quiet_frame, true);
        }
        let silent_frame = vec![0.0; 320];
        let silence_frames = (16000.0 * 0.7 / 320.0) as usize + 1;
        let mut result = None;
        for _ in 0..silence_frames {
            if let Some(chunk) = sm.process_frame(&silent_frame, false) {
                result = Some(chunk);
            }
        }
        assert!(result.is_none(), "low energy chunk should be dropped");
    }

    #[test]
    fn vad_config_defaults() {
        let config = VadConfig::default();
        assert_eq!(config.silence_ms, 700);
        assert!((config.energy_threshold - 0.01).abs() < f32::EPSILON);
    }

    #[test]
    fn accumulator_exact_chunk() {
        let mut acc = ResampleAccumulator::new(48000);
        let n = acc.resampler.input_frames_next();
        // Feed exactly one chunk of silence
        let input = vec![0.0f32; n];
        let output = acc.push(&input);
        // 48kHz -> 16kHz = ratio 1/3, so ~n/3 output samples
        assert!(!output.is_empty(), "exact chunk should produce output");
        assert!(acc.buf.is_empty(), "no remainder after exact chunk");
    }

    #[test]
    fn accumulator_undersized_no_output() {
        let mut acc = ResampleAccumulator::new(48000);
        let n = acc.resampler.input_frames_next();
        // Feed half a chunk — not enough for process()
        let input = vec![0.0f32; n / 2];
        let output = acc.push(&input);
        assert!(
            output.is_empty(),
            "undersized input should produce no output"
        );
        assert_eq!(acc.buf.len(), n / 2, "remainder should be buffered");
    }

    #[test]
    fn accumulator_multi_push() {
        let mut acc = ResampleAccumulator::new(48000);
        let n = acc.resampler.input_frames_next();
        // Push in 3 batches of n/2 each = 1.5 chunks total
        let half = vec![0.0f32; n / 2];
        let out1 = acc.push(&half); // 0.5 chunks buffered
        assert!(out1.is_empty());
        let out2 = acc.push(&half); // 1.0 chunks → drain, 0 remainder
        assert!(
            !out2.is_empty(),
            "should produce output after accumulating full chunk"
        );
        let out3 = acc.push(&half); // 0.5 chunks buffered again
        assert!(out3.is_empty());
        assert_eq!(acc.buf.len(), n / 2);
    }

    #[test]
    fn accumulator_oversized_concatenates() {
        let mut acc = ResampleAccumulator::new(48000);
        let n = acc.resampler.input_frames_next();
        // Feed 2.5 chunks at once
        let input = vec![0.0f32; n * 2 + n / 2];
        let output = acc.push(&input);
        // Should have drained twice, remainder = n/2
        assert!(!output.is_empty(), "oversized input should produce output");
        assert_eq!(acc.buf.len(), n / 2, "remainder after 2 full drains");
        // Output should be from 2 process() calls concatenated
        // Each produces ~n/3 samples, so total ~2*n/3
        assert!(
            output.len() > n / 3,
            "output should contain results from multiple process() calls"
        );
    }
}
