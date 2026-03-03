use crate::error::ApiError;
use crate::prelude::{ApiResult, SpeechApi};
use async_channel::Sender;
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use homunculus_speech::{Mora, Moras, Speak, SpeakQueue};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

/// A single keyframe in a speech timeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct TimelineKeyframe {
    /// Duration of this keyframe in seconds.
    pub duration: f32,
    /// Expression targets to set during this keyframe.
    /// Keys are expression names (e.g. "aa", "ih", "happy"), values are weights (0.0-1.0).
    #[serde(default)]
    pub targets: HashMap<String, f32>,
}

/// Options for the timeline speech API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct SpeakTimelineOptions {
    /// If true, the request blocks until audio playback completes.
    /// Defaults to true.
    pub wait_for_completion: Option<bool>,
    /// Duration in seconds for smoothstep blending between adjacent keyframes.
    /// Defaults to 0.05 (50ms). Clamped to 40% of each mora's duration.
    pub transition_duration: Option<f32>,
}

impl SpeechApi {
    /// Plays audio with synchronized expression keyframes.
    pub async fn speak_with_timeline(
        &self,
        vrm: Entity,
        wav: Vec<u8>,
        keyframes: Vec<TimelineKeyframe>,
        options: SpeakTimelineOptions,
    ) -> ApiResult {
        // Validate WAV header
        if wav.len() < 4 || &wav[..4] != b"RIFF" {
            return Err(ApiError::InvalidInput(
                "Invalid WAV data: missing RIFF header".to_string(),
            ));
        }

        // Validate keyframe durations
        for kf in &keyframes {
            if kf.duration < 0.0 {
                return Err(ApiError::InvalidInput(
                    "Keyframe duration must not be negative".to_string(),
                ));
            }
        }

        // Validate transition duration
        if let Some(td) = options.transition_duration
            && td < 0.0
        {
            return Err(ApiError::InvalidInput(
                "transitionDuration must not be negative".to_string(),
            ));
        }

        // Convert keyframes to Moras
        let transition_duration = options.transition_duration.unwrap_or(0.05);
        let moras = keyframes_to_moras(keyframes, transition_duration);

        self.0
            .schedule(move |task| async move {
                let (tx, rx) = async_channel::unbounded();
                let wait_for_completion = options.wait_for_completion.unwrap_or(true);
                let finish_signal = wait_for_completion.then_some(tx);
                task.will(
                    Update,
                    once::run(enqueue_timeline_speak).with((vrm, wav, moras, finish_signal)),
                )
                .await?;
                if wait_for_completion {
                    task.will(
                        Update,
                        side_effect::tokio::spawn(async move { rx.recv().await }),
                    )
                    .await?;
                }
                Ok(())
            })
            .await?
    }
}

fn keyframes_to_moras(keyframes: Vec<TimelineKeyframe>, transition_duration: f32) -> Moras {
    let queue: VecDeque<Mora> = keyframes
        .into_iter()
        .map(|kf| Mora {
            timer: Timer::from_seconds(kf.duration, TimerMode::Once),
            targets: kf.targets,
        })
        .collect();
    Moras::new(queue, transition_duration)
}

fn enqueue_timeline_speak(
    In((vrm, wav, moras, finish_signal)): In<(Entity, Vec<u8>, Moras, Option<Sender<()>>)>,
    mut query: Query<&mut SpeakQueue>,
) -> ApiResult {
    let mut speak_queue = query.get_mut(vrm).map_err(|_| ApiError::EntityNotFound)?;
    speak_queue.0.push_back(Speak {
        text: String::new(),
        moras,
        wav,
        finish_signal,
    });
    Ok(())
}
