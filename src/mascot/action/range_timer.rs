use crate::mascot::action::MascotActionExt;
use bevy::log::info;
use bevy::prelude::Plugin;
use bevy_flurx::action::delay;
use bevy_flurx::prelude::Omit;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
pub struct RangeTimerActionParams {
    pub min_sec: f32,
    pub max_sec: f32,
}

pub struct RangeTimerActionPlugin;

impl RangeTimerActionPlugin {
    pub const ID: &'static str = "range_timer";
}

impl Plugin for RangeTimerActionPlugin {
    fn build(
        &self,
        app: &mut bevy::app::App,
    ) {
        app.add_mascot_action(Self::ID, |_, params: RangeTimerActionParams| {
            let time = rand::random_range(params.min_sec..=params.max_sec);
            info!("Range Timer wait for {} seconds", time);
            delay::time().with(Duration::from_secs_f32(time)).omit()
        });
    }
}
