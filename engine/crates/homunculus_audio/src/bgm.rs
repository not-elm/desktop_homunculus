use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use homunculus_core::prelude::AssetId;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::se::volume_to_decibels;

#[derive(Resource)]
pub struct BgmChannel;

#[derive(Resource)]
pub struct BgmState {
    pub current_asset: Option<AssetId>,
    pub instance_handle: Option<Handle<AudioInstance>>,
    pub is_looping: bool,
    pub target_volume: f64,
    pub target_speed: f64,
    pub is_paused: bool,
}

impl Default for BgmState {
    fn default() -> Self {
        Self {
            current_asset: None,
            instance_handle: None,
            is_looping: false,
            target_volume: 1.0,
            target_speed: 1.0,
            is_paused: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct BgmStatus {
    pub asset: Option<AssetId>,
    pub state: BgmPlaybackState,
    #[serde(rename = "loop")]
    pub is_loop: bool,
    pub volume: f64,
    pub speed: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub enum BgmPlaybackState {
    Playing,
    Paused,
    Stopped,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct FadeTween {
    pub duration_secs: f64,
    pub easing: Option<Easing>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
#[serde(rename_all = "camelCase")]
pub enum Easing {
    #[default]
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
}

impl From<&FadeTween> for AudioTween {
    fn from(tween: &FadeTween) -> Self {
        let duration = Duration::from_secs_f64(tween.duration_secs);
        let easing = match tween.easing.unwrap_or_default() {
            Easing::Linear => AudioEasing::Linear,
            Easing::EaseIn => AudioEasing::InPowi(2),
            Easing::EaseOut => AudioEasing::OutPowi(2),
            Easing::EaseInOut => AudioEasing::InOutPowi(2),
        };
        AudioTween::new(duration, easing)
    }
}

// --- Events ---

#[derive(Event, Debug, Clone)]
pub struct RequestBgmPlay {
    pub source: Handle<bevy_kira_audio::AudioSource>,
    pub asset_id: AssetId,
    pub is_loop: bool,
    pub volume: f64,
    pub speed: f64,
    pub fade_in: Option<FadeTween>,
}

#[derive(Event, Debug, Clone, Default)]
pub struct RequestBgmStop {
    pub fade_out: Option<FadeTween>,
}

#[derive(Event, Debug, Clone)]
pub struct RequestBgmPause;

#[derive(Event, Debug, Clone)]
pub struct RequestBgmResume;

#[derive(Event, Debug, Clone, Default)]
pub struct RequestBgmUpdate {
    pub volume: Option<f64>,
    pub speed: Option<f64>,
    pub tween: Option<FadeTween>,
}

pub(crate) struct BgmPlugin;

impl Plugin for BgmPlugin {
    fn build(&self, app: &mut App) {
        app.add_audio_channel::<BgmChannel>()
            .init_resource::<BgmState>()
            .add_observer(play_bgm)
            .add_observer(stop_bgm)
            .add_observer(pause_bgm)
            .add_observer(resume_bgm)
            .add_observer(update_bgm);
    }
}

// --- Observers ---

fn play_bgm(
    trigger: On<RequestBgmPlay>,
    channel: Res<AudioChannel<BgmChannel>>,
    mut state: ResMut<BgmState>,
) {
    let event = trigger.event();

    // Stop current BGM if playing
    if state.instance_handle.is_some() {
        channel.stop();
    }

    let mut cmd = channel.play(event.source.clone());
    cmd.with_volume(volume_to_decibels(event.volume))
        .with_playback_rate(event.speed);

    if event.is_loop {
        cmd.looped();
    }

    if let Some(ref fade_in) = event.fade_in {
        cmd.fade_in(AudioTween::from(fade_in));
    }

    let handle = cmd.handle();

    state.current_asset = Some(event.asset_id.clone());
    state.instance_handle = Some(handle);
    state.is_looping = event.is_loop;
    state.target_volume = event.volume;
    state.target_speed = event.speed;
    state.is_paused = false;
}

fn stop_bgm(
    trigger: On<RequestBgmStop>,
    channel: Res<AudioChannel<BgmChannel>>,
    mut state: ResMut<BgmState>,
) {
    let event = trigger.event();

    if let Some(ref fade_out) = event.fade_out {
        channel.stop().fade_out(AudioTween::from(fade_out));
    } else {
        channel.stop();
    }

    state.current_asset = None;
    state.instance_handle = None;
    state.is_looping = false;
    state.target_volume = 1.0;
    state.target_speed = 1.0;
    state.is_paused = false;
}

fn pause_bgm(
    _trigger: On<RequestBgmPause>,
    channel: Res<AudioChannel<BgmChannel>>,
    mut state: ResMut<BgmState>,
) {
    channel.pause();
    state.is_paused = true;
}

fn resume_bgm(
    _trigger: On<RequestBgmResume>,
    channel: Res<AudioChannel<BgmChannel>>,
    mut state: ResMut<BgmState>,
) {
    channel.resume();
    state.is_paused = false;
}

fn update_bgm(
    trigger: On<RequestBgmUpdate>,
    channel: Res<AudioChannel<BgmChannel>>,
    mut state: ResMut<BgmState>,
) {
    let event = trigger.event();
    let tween = event
        .tween
        .as_ref()
        .map(AudioTween::from)
        .unwrap_or_default();

    if let Some(volume) = event.volume {
        channel
            .set_volume(volume_to_decibels(volume))
            .fade_in(tween.clone());
        state.target_volume = volume;
    }

    if let Some(speed) = event.speed {
        channel.set_playback_rate(speed).fade_in(tween);
        state.target_speed = speed;
    }
}
