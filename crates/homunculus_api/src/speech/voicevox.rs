use crate::error::ApiError;
use crate::prelude::{ApiResult, SpeechApi};
use async_channel::Sender;
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use homunculus_core::prelude::ModModuleSource;
use homunculus_speech::prelude::{RequestSpeak, Subtitle};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

impl SpeechApi {
    /// Sends a request to speak sentences using the VoiceVox engine.
    pub async fn speak_on_voicevox(
        &self,
        vrm: Entity,
        sentences: Vec<String>,
        options: SpeakVoiceVoxOptions,
    ) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                let (tx, rx) = async_channel::unbounded();
                let wait_for_completion = options.wait_for_completion.is_some_and(|wait| wait);
                let finish_signal = wait_for_completion.then_some(tx);
                task.will(
                    Update,
                    once::run(speak_voicevox).with((vrm, sentences, options, finish_signal)),
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpeakVoiceVoxOptions {
    pub speaker: Option<u32>,
    /// The pause duration in seconds before the next sentence.
    pub pause: Option<f32>,
    /// Whether to wait for the speech to finish.
    #[serde(rename = "waitForCompletion")]
    pub wait_for_completion: Option<bool>,
    pub subtitle: Option<SubtitleOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubtitleOptions {
    pub font: Option<ModModuleSource>,
    #[serde(rename = "fontSize")]
    pub font_size: Option<f32>,
    pub color: Option<[f32; 4]>,
}

fn speak_voicevox(
    In((vrm, sentences, options, finish_signal)): In<(
        Entity,
        Vec<String>,
        SpeakVoiceVoxOptions,
        Option<Sender<()>>,
    )>,
    mut commands: Commands,
    entities: Query<Entity>,
) -> ApiResult {
    if !entities.contains(vrm) {
        return Err(ApiError::EntityNotfound);
    }
    commands.entity(vrm).trigger(RequestSpeak {
        sentences,
        pause: options.pause.map(Duration::from_secs_f32),
        speaker: options.speaker.unwrap_or(0),
        finish_signal,
        subtitle: match options.subtitle {
            Some(s) => Some(Subtitle {
                font_path: s.font.map(|id| PathBuf::from(id.to_string())),
                font_size: s.font_size,
                color: s.color,
            }),
            None => None,
        },
    });
    Ok(())
}
