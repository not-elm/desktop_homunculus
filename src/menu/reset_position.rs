use crate::menu::TargetMascot;
use bevy::app::App;
use bevy::prelude::{Event, ParallelCommands, Plugin, Query, Transform, Trigger};
use bevy_webview_wry::prelude::IpcTriggerExt;
use serde::Deserialize;

pub struct MenuResetPositionPlugin;

impl Plugin for MenuResetPositionPlugin {
    fn build(&self, app: &mut App) {
        app.add_ipc_trigger::<ResetPosition>("reset_position")
            .add_observer(reset_position);
    }
}

#[derive(Deserialize, Event)]
struct ResetPosition {
    _dummy: bool,
}

fn reset_position(
    trigger: Trigger<ResetPosition>,
    par_commands: ParallelCommands,
    targets: Query<&TargetMascot>,
) {
    if let Ok(TargetMascot(mascot)) = targets.get(trigger.target()) {
        par_commands.command_scope(|mut commands| {
            commands.entity(*mascot).insert(Transform::default());
        });
    }
}
