use crate::mascot::action::MascotActionExt;
use crate::mascot::MascotEntity;
use crate::settings::preferences::action::{ActionName, ActionPreferences, ActionTags};
use bevy::app::App;
use bevy::log::info;
use bevy::prelude::{Commands, In, Plugin, Query, Res};
use bevy_flurx::action::once;
use bevy_flurx::prelude::Omit;

pub struct AutoTransitionPlugin;

impl AutoTransitionPlugin {
    pub const ID: &'static str = "auto_transition";
}

impl Plugin for AutoTransitionPlugin {
    fn build(&self, app: &mut App) {
        app.add_mascot_action(Self::ID, |mascot, _: ()| {
            once::run(auto_transition).with(mascot).omit()
        });
    }
}

fn auto_transition(
    In(mascot): In<MascotEntity>,
    mut commands: Commands,
    mascots: Query<(&ActionName, &ActionTags)>,
    actions: Res<ActionPreferences>,
) {
    let Ok((current_name, current_tags)) = mascots.get(mascot.0) else {
        return;
    };

    if let Some(name) = actions.random_next_action(current_tags, current_name) {
        info!("Auto transition from [{current_name}] to [{name}]");
        commands.entity(mascot.0).insert(name.clone());
    }
}
