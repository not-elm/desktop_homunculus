use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::session::SharedSttSession;

/// Thread 1 -> Thread 2 PCM frame channel capacity.
pub const AUDIO_CHANNEL_CAPACITY: usize = 512;

/// Return value from the cpal capture thread.
pub struct CaptureHandle {
    pub audio_rx: mpsc::Receiver<Vec<f32>>,
    pub sample_rate: u32,
    pub needs_resample: bool,
}

/// Capture-related errors.
#[derive(Debug, thiserror::Error)]
pub enum CaptureError {
    #[error("No microphone device found")]
    NoMicrophone,
    #[error("No supported audio config: {0}")]
    NoSupportedConfig(String),
    #[error("Failed to spawn capture thread: {0}")]
    ThreadSpawn(String),
}

/// Retrieve the default input device.
pub fn get_input_device() -> Result<cpal::Device, CaptureError> {
    let host = cpal::default_host();
    host.default_input_device().ok_or(CaptureError::NoMicrophone)
}

/// Thread 1: spawn the cpal audio capture thread.
pub fn spawn_capture_thread(
    device: cpal::Device,
    cancel: CancellationToken,
    session: SharedSttSession,
) -> Result<CaptureHandle, CaptureError> {
    let (config, needs_resample) = select_input_config(&device)?;
    let sample_rate = config.sample_rate.0;
    let channels = config.channels as usize;
    let (tx, rx) = mpsc::sync_channel::<Vec<f32>>(AUDIO_CHANNEL_CAPACITY);

    let session_clone = session.clone();
    std::thread::Builder::new()
        .name("stt-capture".into())
        .spawn(move || {
            let error_session = session_clone.clone();
            let stream = match build_input_stream_adaptive(
                &device,
                &config,
                tx,
                channels,
                move |err| {
                    tracing::error!("cpal error: {err}");
                    if let Ok(mut session) = error_session.0.try_lock() {
                        session.fail("device_lost".into(), format!("Audio device error: {err}"));
                    }
                },
            ) {
                Ok(s) => s,
                Err(e) => {
                    tracing::error!("Failed to build input stream: {e}");
                    if let Ok(mut session) = session_clone.0.try_lock() {
                        session
                            .fail("device_lost".into(), format!("Failed to build input stream: {e}"));
                    }
                    return;
                }
            };
            if let Err(e) = stream.play() {
                tracing::error!("Failed to start input stream: {e}");
                if let Ok(mut session) = session_clone.0.try_lock() {
                    session
                        .fail("device_lost".into(), format!("Failed to start input stream: {e}"));
                }
                return;
            }
            // Block until cancellation — poll the token since `cancelled()` is async.
            while !cancel.is_cancelled() {
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
        })
        .map_err(|e| CaptureError::ThreadSpawn(e.to_string()))?;

    Ok(CaptureHandle {
        audio_rx: rx,
        sample_rate,
        needs_resample,
    })
}

fn select_input_config(
    device: &cpal::Device,
) -> Result<(cpal::StreamConfig, bool), CaptureError> {
    let target = cpal::StreamConfig {
        channels: 1,
        sample_rate: cpal::SampleRate(16000),
        buffer_size: cpal::BufferSize::Default,
    };
    if let Ok(configs) = device.supported_input_configs() {
        for range in configs {
            if range.min_sample_rate().0 <= 16000
                && range.max_sample_rate().0 >= 16000
                && range.channels() >= 1
            {
                return Ok((target, false));
            }
        }
    }
    let default = device
        .default_input_config()
        .map_err(|e| CaptureError::NoSupportedConfig(e.to_string()))?;
    Ok((default.config(), true))
}

fn build_input_stream_adaptive<E>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    tx: mpsc::SyncSender<Vec<f32>>,
    channels: usize,
    error_callback: E,
) -> Result<cpal::Stream, cpal::BuildStreamError>
where
    E: FnMut(cpal::StreamError) + Send + 'static,
{
    let supported = device
        .default_input_config()
        .map(|c| c.sample_format())
        .unwrap_or(cpal::SampleFormat::F32);
    match supported {
        cpal::SampleFormat::F32 => device.build_input_stream(
            config,
            move |data: &[f32], _| {
                let mono = downmix_to_mono(data, channels);
                let _ = tx.try_send(mono);
            },
            error_callback,
            None,
        ),
        cpal::SampleFormat::I16 => device.build_input_stream(
            config,
            move |data: &[i16], _| {
                let f32_data: Vec<f32> = data.iter().map(|&s| s as f32 / i16::MAX as f32).collect();
                let mono = downmix_to_mono(&f32_data, channels);
                let _ = tx.try_send(mono);
            },
            error_callback,
            None,
        ),
        _ => Err(cpal::BuildStreamError::StreamConfigNotSupported),
    }
}

#[inline]
fn downmix_to_mono(data: &[f32], channels: usize) -> Vec<f32> {
    if channels == 1 {
        data.to_vec()
    } else {
        data.chunks(channels)
            .map(|frame| frame.iter().sum::<f32>() / channels as f32)
            .collect()
    }
}
