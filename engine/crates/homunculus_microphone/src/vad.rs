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
}

impl VadStateMachine {
    /// Create a new state machine with the given configuration.
    pub fn new(config: &VadConfig) -> Self {
        Self {
            speech_buffer: Vec::new(),
            silence_samples: 0,
            in_speech: false,
            silence_threshold: (16000.0 * config.silence_ms as f64 / 1000.0) as usize,
            min_chunk_samples: 24_000, // 1.5s at 16kHz
            energy_threshold: config.energy_threshold,
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

        if self.silence_samples < self.silence_threshold {
            return None;
        }

        let chunk = std::mem::take(&mut self.speech_buffer);
        self.in_speech = false;
        self.silence_samples = 0;

        if chunk.len() < self.min_chunk_samples {
            return None;
        }

        let rms = (chunk.iter().map(|s| s * s).sum::<f32>() / chunk.len() as f32).sqrt();
        if rms < self.energy_threshold {
            return None;
        }

        Some(chunk)
    }
}

/// Thread 2: spawn the VAD + chunking thread.
pub fn spawn_vad_thread(
    audio_rx: mpsc::Receiver<Vec<f32>>,
    sample_rate: u32,
    needs_resample: bool,
    cancel: CancellationToken,
    config: VadConfig,
) -> crossbeam_channel::Receiver<Vec<f32>> {
    let (chunk_tx, chunk_rx) = crossbeam_channel::bounded::<Vec<f32>>(1);

    std::thread::Builder::new()
        .name("stt-vad".into())
        .spawn(move || {
            vad_thread_main(audio_rx, sample_rate, needs_resample, cancel, config, chunk_tx);
        })
        .expect("failed to spawn stt-vad thread");

    chunk_rx
}

fn vad_thread_main(
    audio_rx: mpsc::Receiver<Vec<f32>>,
    sample_rate: u32,
    needs_resample: bool,
    cancel: CancellationToken,
    config: VadConfig,
    chunk_tx: crossbeam_channel::Sender<Vec<f32>>,
) {
    let mut resampler = if needs_resample {
        Some(create_resampler(sample_rate))
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

        let samples_16k = match resample(&mut resampler, raw_audio) {
            Some(s) => s,
            None => continue,
        };

        for frame in samples_16k.chunks(VAD_FRAME_SIZE) {
            if frame.len() < VAD_FRAME_SIZE {
                continue;
            }

            let frame_i16: Vec<i16> = frame
                .iter()
                .map(|&s| (s.clamp(-1.0, 1.0) * i16::MAX as f32) as i16)
                .collect();

            let is_voice = vad.is_voice_segment(&frame_i16).unwrap_or(false);

            if let Some(chunk) = state_machine.process_frame(frame, is_voice) {
                send_chunk_discard_oldest(&chunk_tx, chunk);
            }
        }
    }
}

fn create_resampler(source_rate: u32) -> rubato::SincFixedIn<f32> {
    rubato::SincFixedIn::<f32>::new(
        16000.0 / source_rate as f64,
        2.0,
        rubato::SincInterpolationParameters {
            sinc_len: 128,
            f_cutoff: 0.95,
            oversampling_factor: 128,
            interpolation: rubato::SincInterpolationType::Linear,
            window: rubato::WindowFunction::BlackmanHarris2,
        },
        1024,
        1,
    )
    .expect("failed to create resampler")
}

fn resample(resampler: &mut Option<rubato::SincFixedIn<f32>>, raw: Vec<f32>) -> Option<Vec<f32>> {
    match resampler {
        Some(r) => match r.process(&[raw], None) {
            Ok(output) => output.into_iter().next(),
            Err(e) => {
                tracing::error!("resample error: {e}");
                None
            }
        },
        None => Some(raw),
    }
}

fn send_chunk_discard_oldest(tx: &crossbeam_channel::Sender<Vec<f32>>, chunk: Vec<f32>) {
    match tx.try_send(chunk) {
        Ok(()) => {}
        Err(TrySendError::Full(_)) => {
            tracing::debug!("VAD chunk channel full, dropping chunk");
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
}
