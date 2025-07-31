mod audio_query;
mod loader;

use crate::voicevox::loader::VoiceVoxLoaderPlugin;
use async_channel::Sender;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

pub(crate) struct VoiceVoxPlugin;

impl Plugin for VoiceVoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(VoiceVoxLoaderPlugin);
    }
}

#[derive(Event, Debug, Clone, Default)]
pub struct RequestSpeak {
    pub sentences: Vec<String>,
    pub speaker: u32,
    /// The pause duration in seconds before the next sentence.
    pub pause: Option<Duration>,
    pub subtitle: Option<Subtitle>,
    /// Optional sender to signal when the speech has finished.
    pub finish_signal: Option<Sender<()>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subtitle {
    pub font_path: Option<PathBuf>,
    pub font_size: Option<f32>,
    pub color: Option<[f32; 4]>,
}

impl Default for Subtitle {
    fn default() -> Self {
        Self {
            font_path: None,
            font_size: Some(24.0),
            color: Some([1.0, 1.0, 1.0, 1.0]),
        }
    }
}
