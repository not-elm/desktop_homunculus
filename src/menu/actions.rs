use crate::settings::preferences::action::{ActionName, ActionPreferences, ActionProperties};
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{
    resource_exists_and_changed, Commands, Event, IntoScheduleConfigs, Res, ResMut, Trigger,
};
use bevy_flurx::prelude::once;
use bevy_flurx::task::ReactorTask;
use bevy_webview_wry::prelude::*;
use serde::Deserialize;

pub struct MenuActionsPlugin;

impl Plugin for MenuActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_ipc_trigger::<UpdateAction>("update_action")
            .add_systems(
                Update,
                emit_action_preferences.run_if(resource_exists_and_changed::<ActionPreferences>),
            )
            .add_observer(apply_update_action);
    }
}

#[derive(Deserialize, Clone, Debug, Event)]
struct UpdateAction {
    action: ActionName,
    properties: ActionProperties,
}

#[command]
pub async fn request_send_actions(task: ReactorTask) {
    task.will(Update, once::run(emit_action_preferences)).await;
}

fn emit_action_preferences(mut commands: Commands, actions: Res<ActionPreferences>) {
    commands.trigger(EmitIpcEvent {
        id: "actions".to_string(),
        payload: EventPayload::new(actions.as_ref()),
    });
}

fn apply_update_action(trigger: Trigger<UpdateAction>, mut actions: ResMut<ActionPreferences>) {
    actions.update(trigger.action.clone(), trigger.properties.clone());
}
