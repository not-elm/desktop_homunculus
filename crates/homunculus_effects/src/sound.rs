use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub(crate) struct SoundEffectsPlugin;

impl Plugin for SoundEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_if_finished)
            .add_observer(apply_request_sound_effects);
    }
}

#[derive(Event, Serialize, Deserialize)]
pub struct RequestSoundEffect {
    pub sound_path: PathBuf,
}

#[derive(Component)]
struct SoundEffect;

fn apply_request_sound_effects(
    trigger: Trigger<RequestSoundEffect>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        SoundEffect,
        Name::new(format!("SoundEffect({})", trigger.sound_path.display())),
        AudioPlayer::new(asset_server.load(trigger.sound_path.as_path())),
    ));
}

fn despawn_if_finished(mut commands: Commands, effects: Query<(Entity, &AudioSink)>) {
    for (entity, sink) in effects.iter() {
        if sink.empty() {
            commands.entity(entity).try_despawn();
        }
    }
}
