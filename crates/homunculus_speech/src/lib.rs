//! # Homunculus Speech
//!
//! This crate provides speech synthesis and lip-sync functionality for VRM mascot
//! models, enabling them to speak with realistic mouth movements and optional
//! subtitle display.
//!
//! ## Overview
//!
//! `homunculus_speech` integrates text-to-speech synthesis with VRM expression
//! controls to create realistic speaking animations. The system uses VoiceVox
//! for speech generation and mora-based lip-sync for accurate mouth movements.
//!
//! ## Key Features
//!
//! - **VoiceVox Integration**: High-quality Japanese text-to-speech synthesis
//! - **Mora-Based Lip Sync**: Accurate mouth movements based on phonetic timing
//! - **VRM Expression Control**: Direct manipulation of VRM facial expressions
//! - **Subtitle Support**: Optional subtitle display with customizable styling
//! - **Speech Queue**: Queue multiple speech requests for sequential playback
//! - **Vowel Animation**: Smooth interpolation between vowel mouth shapes
//!
//! ## How Speech Works
//!
//! 1. **Text Input**: Text is sent to VoiceVox for synthesis
//! 2. **Audio Generation**: VoiceVox returns audio data and timing information
//! 3. **Mora Parsing**: Timing data is converted to mora-based lip-sync data
//! 4. **Queue Management**: Speech requests are queued for sequential playback
//! 5. **Expression Animation**: VRM facial expressions animate based on vowel timing
//! 6. **Audio Playback**: Synthesized audio plays synchronized with lip movements
//!
//! ## Vowel System
//!
//! The lip-sync system uses five Japanese vowel shapes:
//! - **Aa** (あ): Open mouth for 'a' sounds
//! - **Ih** (い): Narrow mouth for 'i' sounds  
//! - **Ou** (う): Rounded mouth for 'u' sounds
//! - **Ee** (え): Semi-open mouth for 'e' sounds
//! - **Oh** (お): Rounded open mouth for 'o' sounds

// use crate::loader::LipSyncLoaderPlugin;
use crate::prelude::{RequestShowSubtitle, Subtitle};
use crate::subtitle::SpeakSubtitlesPlugin;
use crate::voicevox::VoiceVoxPlugin;
use async_channel::Sender;
use bevy::audio::PlaybackMode;
use bevy::prelude::*;
use bevy_vrm1::prelude::*;
use homunculus_core::prelude::OutputLog;
use serde::*;
use std::collections::VecDeque;
use std::sync::Arc;

// mod loader;
// mod mfcc;
mod subtitle;
mod voicevox;

pub mod prelude {
    pub use crate::{
        // loader::{LipSync, LipSyncHandle},
        subtitle::RequestShowSubtitle,
        voicevox::{RequestSpeak, Subtitle},
    };
}

#[derive(Component, Deref, DerefMut)]
pub struct SpeakQueue(pub VecDeque<Speak>);

pub struct Speak {
    pub text: String,
    pub moras: Moras,
    pub wav: Vec<u8>,
    pub subtitle: Option<Subtitle>,
    pub finish_signal: Option<Sender<()>>,
}

#[derive(Component, Debug, Deref, DerefMut)]
pub struct Moras(pub(crate) VecDeque<Mora>);

#[derive(Reflect, Serialize, Deserialize, Debug, Clone)]
#[reflect(Serialize, Deserialize)]
pub struct Mora {
    pub timer: Timer,
    pub vowel: Option<VowelName>,
}

#[derive(Reflect, Clone, Debug, Serialize, Deserialize, Copy)]
#[reflect(Serialize, Deserialize)]
pub enum VowelName {
    Aa,
    Ih,
    Ou,
    Ee,
    Oh,
}

impl VowelName {
    pub const fn as_str(&self) -> &'static str {
        match self {
            VowelName::Aa => "aa",
            VowelName::Ih => "ih",
            VowelName::Ou => "ou",
            VowelName::Ee => "ee",
            VowelName::Oh => "oh",
        }
    }
}

/// Plugin that provides speech synthesis and lip-sync functionality for VRM models.
///
/// This plugin integrates VoiceVox text-to-speech with VRM expression controls
/// to create realistic speaking animations with accurate lip-sync based on
/// phonetic timing data.
///
/// # Components
///
/// The plugin includes:
/// - `VoiceVoxPlugin`: Handles text-to-speech synthesis and mora generation
/// - `SpeakSubtitlesPlugin`: Manages subtitle display during speech
/// - Speech queue management for sequential speech playback
/// - Mora-based lip-sync animation system
///
/// # Systems
///
/// - `insert_speak_queue`: Adds speech queues to newly initialized VRM models
/// - `pop_speak_queue`: Processes queued speech requests and starts playback
/// - `advance_mora`: Updates lip-sync animations based on mora timing
///
/// # Expression Integration
///
/// The plugin directly manipulates VRM expression weights to create
/// mouth movements that correspond to the five Japanese vowel sounds,
/// providing realistic lip-sync animation synchronized with audio playback.
///
/// # Audio Processing
///
/// Speech audio is generated by VoiceVox and played through Bevy's audio
/// system, while timing information is used to drive the lip-sync animation
/// in parallel with audio playback.
pub struct HomunculusSpeechPlugin;

impl Plugin for HomunculusSpeechPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((VoiceVoxPlugin, SpeakSubtitlesPlugin))
            .register_type::<Mora>()
            .add_systems(Update, (advance_mora, insert_speak_queue, pop_speak_queue));
    }
}

fn insert_speak_queue(
    mut commands: Commands,
    vrms: Query<Entity, (With<Vrm>, Added<Initialized>)>,
) {
    for vrm in vrms.iter() {
        commands.entity(vrm).insert(SpeakQueue(VecDeque::new()));
    }
}

fn pop_speak_queue(
    mut commands: Commands,
    mut audios: ResMut<Assets<AudioSource>>,
    mut ew: EventWriter<RequestShowSubtitle>,
    mut vrms: Query<(Entity, &mut SpeakQueue), Without<Moras>>,
) {
    for (vrm, mut queue) in vrms.iter_mut() {
        let Some(speak) = queue.0.pop_front() else {
            continue;
        };
        commands.entity(vrm).try_insert(speak.moras);
        let player = spawn_audio_player(&mut commands, &mut audios, speak.wav, speak.finish_signal);
        if let Some(subtitle) = speak.subtitle {
            ew.write(RequestShowSubtitle {
                audio_source: player,
                vrm,
                text: speak.text,
                font_path: subtitle.font_path,
                font_size: subtitle.font_size,
                color: subtitle.color,
            });
        }
    }
}

fn spawn_audio_player(
    commands: &mut Commands,
    audios: &mut Assets<AudioSource>,
    wav: Vec<u8>,
    finish_signal: Option<Sender<()>>,
) -> Entity {
    let mut audio_player = commands.spawn((
        AudioPlayer(audios.add(AudioSource {
            bytes: Arc::from(wav),
        })),
        PlaybackSettings {
            mode: PlaybackMode::Despawn,
            ..default()
        },
    ));
    if let Some(signal) = finish_signal {
        audio_player.observe(move |_: Trigger<OnRemove, AudioPlayer>| {
            println!("Audio player despawned, sending finish signal");
            signal
                .send_blocking(())
                .output_log_if_error("spawn_audio_player:send");
        });
    }
    audio_player.id()
}

fn advance_mora(
    mut commands: Commands,
    mut vrms: Query<(Entity, &mut Moras)>,
    mut transforms: Query<&mut Transform>,
    searcher: ChildSearcher,
    time: Res<Time>,
) {
    for (vrm_entity, mut moras) in vrms.iter_mut() {
        let Some(expressions_root) = searcher.find_expressions_root(vrm_entity) else {
            continue;
        };
        let Some(mut mora) = moras.pop_front() else {
            reset_vowels(expressions_root, &mut commands, &searcher);
            commands.entity(vrm_entity).remove::<Moras>();
            continue;
        };

        let delta = time.delta();
        let remaining = mora.timer.remaining();
        let timer = mora.timer.tick(delta);
        let weight = timer.fraction();
        if let Some(vowel) = &mora.vowel {
            update_vowel(
                expressions_root,
                vowel.as_str(),
                &mut transforms,
                &searcher,
                weight,
            );
        }
        if timer.finished() {
            reset_vowels(expressions_root, &mut commands, &searcher);
            if let Some(next_mora) = moras.front_mut() {
                next_mora.timer.tick(delta.saturating_sub(remaining));
            }
        } else {
            moras.push_front(mora);
        }
    }
}

fn reset_vowels(expressions_root: Entity, commands: &mut Commands, searcher: &ChildSearcher) {
    for name in ["aa", "ih", "ou", "ee", "oh"] {
        let Some(expression) = searcher.find_by_name(expressions_root, name) else {
            return;
        };
        commands.entity(expression).insert(Transform::default());
    }
}

fn update_vowel(
    expressions_root: Entity,
    name: &str,
    transforms: &mut Query<&mut Transform>,
    searcher: &ChildSearcher,
    weight: f32,
) {
    let Some(expression) = searcher.find_by_name(expressions_root, name) else {
        return;
    };
    let Ok(mut tf) = transforms.get_mut(expression) else {
        return;
    };
    tf.translation.x = weight;
}
