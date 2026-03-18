//! # Homunculus Speech
//!
//! This crate provides lip-sync functionality for VRM mascot models,
//! enabling them to speak with realistic mouth movements.
//!
//! ## Overview
//!
//! `homunculus_speech` provides mora-based lip-sync for accurate mouth movements
//! synchronized with audio playback. Speech audio and timing data are provided
//! externally (e.g. via the Timeline API or MODs).
//!
//! ## Key Features
//!
//! - **Mora-Based Lip Sync**: Accurate mouth movements based on phonetic timing
//! - **VRM Expression Control**: Direct manipulation of VRM facial expressions
//! - **Speech Queue**: Queue multiple speech requests for sequential playback
//! - **Vowel Animation**: Smooth interpolation between vowel mouth shapes
//!
//! ## Vowel System
//!
//! The lip-sync system uses five Japanese vowel shapes:
//! - **Aa** (あ): Open mouth for 'a' sounds
//! - **Ih** (い): Narrow mouth for 'i' sounds
//! - **Ou** (う): Rounded mouth for 'u' sounds
//! - **Ee** (え): Semi-open mouth for 'e' sounds
//! - **Oh** (お): Rounded open mouth for 'o' sounds

use async_channel::Sender;
use bevy::audio::PlaybackMode;
use bevy::prelude::*;
use bevy_vrm1::prelude::*;
use homunculus_core::prelude::OutputLog;
use serde::*;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

#[derive(Component, Deref, DerefMut)]
pub struct SpeakQueue(pub VecDeque<Speak>);

pub struct Speak {
    pub text: String,
    pub moras: Moras,
    pub wav: Vec<u8>,
    pub finish_signal: Option<Sender<()>>,
}

#[derive(Component, Debug)]
pub struct Moras {
    pub(crate) queue: VecDeque<Mora>,
    pub(crate) transition_duration: f32,
}

impl Moras {
    pub fn new(queue: VecDeque<Mora>, transition_duration: f32) -> Self {
        Self {
            queue,
            transition_duration,
        }
    }
}

#[derive(Reflect, Serialize, Deserialize, Debug, Clone)]
#[reflect(Serialize, Deserialize)]
pub struct Mora {
    pub timer: Timer,
    #[serde(default)]
    pub targets: HashMap<String, f32>,
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

/// Plugin that provides lip-sync functionality for VRM models.
///
/// This plugin provides mora-based lip-sync animation synchronized
/// with audio playback. Speech audio and timing data are provided
/// externally (e.g. via the Timeline API or MODs).
///
/// # Systems
///
/// - `insert_speak_queue`: Adds speech queues to newly initialized VRM models
/// - `pop_speak_queue`: Processes queued speech requests and starts playback
/// - `advance_mora`: Updates lip-sync animations based on mora timing
pub struct HomunculusSpeechPlugin;

impl Plugin for HomunculusSpeechPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Mora>()
            .add_systems(
                Update,
                (insert_speak_queue, (pop_speak_queue, advance_mora).chain()),
            );
    }
}

fn insert_speak_queue(
    mut commands: Commands,
    vrms: Query<Entity, (With<Vrm>, Added<Initialized>)>,
) {
    for vrm in vrms.iter() {
        commands.entity(vrm).try_insert(SpeakQueue(VecDeque::new()));
    }
}

fn pop_speak_queue(
    mut commands: Commands,
    mut audios: ResMut<Assets<AudioSource>>,
    mut vrms: Query<(Entity, &mut SpeakQueue), Without<Moras>>,
) {
    for (vrm, mut queue) in vrms.iter_mut() {
        let Some(speak) = queue.0.pop_front() else {
            continue;
        };
        commands.entity(vrm).try_insert(speak.moras);
        spawn_audio_player(&mut commands, &mut audios, speak.wav, speak.finish_signal);
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
        audio_player.observe(move |_: On<Remove, AudioPlayer>| {
            signal
                .send_blocking(())
                .output_log_if_error("spawn_audio_player:send");
        });
    }
    audio_player.id()
}

/// Minimum weight threshold for blend output. Values below this are filtered out.
const BLEND_WEIGHT_EPSILON: f32 = 0.001;

/// Smoothstep function: 3t^2 - 2t^3
fn smoothstep(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Blend between two target weight maps.
/// `t` ranges from 0.0 (fully current) to 1.0 (fully next).
/// Iterates the union of keys; missing keys default to 0.0.
fn blend_targets(
    current: &HashMap<String, f32>,
    next: &HashMap<String, f32>,
    t: f32,
) -> HashMap<String, f32> {
    let mut result = HashMap::new();
    for key in current.keys().chain(next.keys()) {
        if result.contains_key(key) {
            continue;
        }
        let cur_w = current.get(key).copied().unwrap_or(0.0);
        let next_w = next.get(key).copied().unwrap_or(0.0);
        let blended = cur_w * (1.0 - t) + next_w * t;
        if blended > BLEND_WEIGHT_EPSILON {
            result.insert(key.clone(), blended);
        }
    }
    result
}

fn advance_mora(mut commands: Commands, mut vrms: Query<(Entity, &mut Moras)>, time: Res<Time>) {
    let empty_targets = HashMap::new();

    for (vrm_entity, mut moras) in vrms.iter_mut() {
        let Some(mora) = moras.queue.front() else {
            // Queue empty: close mouth and remove component
            commands.trigger(ModifyExpressions::mouth_weights(
                vrm_entity,
                std::iter::empty::<(&str, f32)>(),
            ));
            commands.entity(vrm_entity).remove::<Moras>();
            continue;
        };

        let remaining = mora.timer.remaining().as_secs_f32();
        let mora_duration = mora.timer.duration().as_secs_f32();
        let current_targets = &mora.targets;
        let transition_duration = moras.transition_duration;

        // Compute effective transition: clamp to 40% of mora duration
        let effective_transition = transition_duration.min(mora_duration * 0.4);

        // Apply weights directly in each branch to avoid unnecessary HashMap allocations
        if current_targets.is_empty() {
            // Pause/silence: no interpolation, mouth closed
            commands.trigger(ModifyExpressions::mouth_weights(
                vrm_entity,
                std::iter::empty::<(&str, f32)>(),
            ));
        } else if remaining <= effective_transition && effective_transition > 0.0 {
            // In transition zone: blend toward next mora's targets
            let next_targets = moras
                .queue
                .get(1)
                .map(|m| &m.targets)
                .unwrap_or(&empty_targets);

            let t = 1.0 - (remaining / effective_transition);
            let t_smooth = smoothstep(t);

            let blended = blend_targets(current_targets, next_targets, t_smooth);
            commands.trigger(ModifyExpressions::mouth_weights(
                vrm_entity,
                blended.iter().map(|(k, &v)| (k.as_str(), v)),
            ));
        } else {
            // Stable: borrow directly, no clone
            commands.trigger(ModifyExpressions::mouth_weights(
                vrm_entity,
                current_targets.iter().map(|(k, &v)| (k.as_str(), v)),
            ));
        }

        // Advance timer
        let delta = time.delta();
        let Some(mora) = moras.queue.front_mut() else {
            continue;
        };
        let remaining = mora.timer.remaining();

        if mora.timer.tick(delta).is_finished() {
            moras.queue.pop_front();
            if let Some(next_mora) = moras.queue.front_mut() {
                next_mora.timer.tick(delta.saturating_sub(remaining));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn map(pairs: &[(&str, f32)]) -> HashMap<String, f32> {
        pairs.iter().map(|(k, v)| (k.to_string(), *v)).collect()
    }

    #[test]
    fn blend_targets_full_current() {
        let current = map(&[("aa", 1.0)]);
        let next = map(&[("ih", 1.0)]);
        let result = blend_targets(&current, &next, 0.0);
        assert!((result["aa"] - 1.0).abs() < 0.001);
        assert!(result.get("ih").is_none() || result["ih"] < 0.001);
    }

    #[test]
    fn blend_targets_full_next() {
        let current = map(&[("aa", 1.0)]);
        let next = map(&[("ih", 1.0)]);
        let result = blend_targets(&current, &next, 1.0);
        assert!(result.get("aa").is_none() || result["aa"] < 0.001);
        assert!((result["ih"] - 1.0).abs() < 0.001);
    }

    #[test]
    fn blend_targets_midpoint() {
        let current = map(&[("aa", 1.0)]);
        let next = map(&[("ih", 1.0)]);
        let result = blend_targets(&current, &next, 0.5);
        assert!((result["aa"] - 0.5).abs() < 0.001);
        assert!((result["ih"] - 0.5).abs() < 0.001);
    }

    #[test]
    fn blend_targets_same_key() {
        let current = map(&[("aa", 1.0)]);
        let next = map(&[("aa", 1.0)]);
        let result = blend_targets(&current, &next, 0.5);
        assert!((result["aa"] - 1.0).abs() < 0.001);
    }

    #[test]
    fn blend_targets_empty_maps() {
        let current = HashMap::new();
        let next = HashMap::new();
        let result = blend_targets(&current, &next, 0.5);
        assert!(result.is_empty());
    }

    #[test]
    fn blend_targets_current_empty() {
        let current = HashMap::new();
        let next = map(&[("oh", 1.0)]);
        let result = blend_targets(&current, &next, 0.7);
        assert!((result["oh"] - 0.7).abs() < 0.001);
    }

    #[test]
    fn blend_targets_near_zero_filtered() {
        let current = map(&[("aa", 0.001)]);
        let next = HashMap::new();
        let result = blend_targets(&current, &next, 0.5);
        // 0.001 * 0.5 = 0.0005 < 0.001, should be filtered
        assert!(result.is_empty());
    }

    #[test]
    fn smoothstep_boundaries() {
        assert!((smoothstep(0.0)).abs() < 0.001);
        assert!((smoothstep(1.0) - 1.0).abs() < 0.001);
        assert!((smoothstep(0.5) - 0.5).abs() < 0.001);
    }

    #[test]
    fn smoothstep_clamps() {
        assert!((smoothstep(-0.5)).abs() < 0.001);
        assert!((smoothstep(1.5) - 1.0).abs() < 0.001);
    }
}
