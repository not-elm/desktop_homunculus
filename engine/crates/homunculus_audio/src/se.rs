use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

#[derive(Resource)]
pub struct SeChannel;

#[derive(Event, Debug, Clone)]
pub struct RequestSe {
    pub source: Handle<bevy_kira_audio::AudioSource>,
    pub volume: f64,
    pub speed: f64,
    pub panning: f64,
}

pub(crate) struct SePlugin;

impl Plugin for SePlugin {
    fn build(&self, app: &mut App) {
        app.add_audio_channel::<SeChannel>().add_observer(play_se);
    }
}

pub fn volume_to_decibels(volume: f64) -> Decibels {
    if volume <= 0.0 {
        Decibels(-60.0)
    } else {
        Decibels(20.0 * (volume as f32).log10())
    }
}

fn play_se(trigger: On<RequestSe>, channel: Res<AudioChannel<SeChannel>>) {
    let event = trigger.event();
    channel
        .play(event.source.clone())
        .with_volume(volume_to_decibels(event.volume))
        .with_playback_rate(event.speed)
        .with_panning(event.panning as f32);
}
