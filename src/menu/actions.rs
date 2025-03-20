use crate::menu::Menu;
use crate::settings::preferences::action::{ActionName, ActionPreferences, ActionProperties};
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{
    resource_exists_and_changed, EventReader, IntoSystemConfigs, Query, Res, ResMut, With,
};
use bevy_flurx::prelude::once;
use bevy_flurx::task::ReactorTask;
use bevy_webview_wry::prelude::*;
use serde::Deserialize;

pub struct MenuActionsPlugin;

impl Plugin for MenuActionsPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_ipc_event::<UpdateAction>("update_action")
            .add_systems(
                Update,
                (
                    send_action_preferences
                        .run_if(resource_exists_and_changed::<ActionPreferences>),
                    update_action,
                ),
            );
    }
}

#[derive(Deserialize, Clone, Debug)]
struct UpdateAction {
    action: ActionName,
    properties: ActionProperties,
}

#[command]
pub async fn request_send_actions(task: ReactorTask) {
    task.will(Update, once::run(send_action_preferences)).await;
}

fn send_action_preferences(
    mut webviews: Query<&mut EventEmitter, With<Menu>>,
    actions: Res<ActionPreferences>,
) {
    for mut emitter in webviews.iter_mut() {
        emitter.emit("actions", actions.as_ref());
    }
}

fn update_action(
    mut er: EventReader<IpcEvent<UpdateAction>>,
    mut actions: ResMut<ActionPreferences>,
) {
    for event in er.read() {
        let args = event.payload.clone();
        actions.update(args.action, args.properties.clone());
    }
}
