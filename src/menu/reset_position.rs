use crate::menu::TargetMascot;
use bevy::app::{App, Update};
use bevy::prelude::{EventReader, ParallelCommands, Plugin, Query, Transform};
use bevy_webview_wry::ipc::IpcEvent;
use bevy_webview_wry::prelude::IpcEventExt;
use serde::Deserialize;

pub struct MenuResetPositionPlugin;

impl Plugin for MenuResetPositionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_ipc_event::<ResetPosition>("reset_position")
            .add_systems(Update, reset_position);
    }
}

#[derive(Deserialize)]
struct ResetPosition {
    _dummy: bool,
}

fn reset_position(
    mut er: EventReader<IpcEvent<ResetPosition>>,
    par_commands: ParallelCommands,
    targets: Query<&TargetMascot>,
) {
    er.par_read().for_each(|event| {
        if let Ok(TargetMascot(mascot)) = targets.get(event.webview_entity) {
            par_commands.command_scope(|mut commands| {
                commands.entity(*mascot).insert(Transform::default());
            });
        }
    });
}