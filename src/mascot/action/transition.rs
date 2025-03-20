use crate::mascot::action::MascotActionExt;
use crate::settings::preferences::action::ActionName;
use bevy::app::{App, Plugin};
use bevy::log::info;
use bevy::prelude::Commands;
use bevy_flurx::action::once;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionActionParams {
    pub next: ActionName,
}

pub struct TransitionActionPlugin;

impl TransitionActionPlugin {
    pub const ID: &'static str = "transition";
}

impl Plugin for TransitionActionPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_mascot_action(Self::ID, |mascot, params: TransitionActionParams| {
            once::run(move |mut commands: Commands| {
                info!("Transition {mascot:?} into {}", params.next);
                commands.entity(mascot.0).insert(params.next.clone());
            })
        });
    }
}
