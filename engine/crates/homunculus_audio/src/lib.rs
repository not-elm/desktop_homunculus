pub mod bgm;
pub mod se;

use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;

pub mod prelude {
    pub use bevy_kira_audio::AudioSource as KiraAudioSource;

    pub use crate::{
        HomunculusAudioPlugin,
        bgm::{
            BgmChannel, BgmPlaybackState, BgmState, BgmStatus, Easing, FadeTween, RequestBgmPause,
            RequestBgmPlay, RequestBgmResume, RequestBgmStop, RequestBgmUpdate,
        },
        se::{RequestSe, SeChannel},
    };
}

pub struct HomunculusAudioPlugin;

impl Plugin for HomunculusAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((AudioPlugin, se::SePlugin, bgm::BgmPlugin));
    }
}
