use crate::menu::TargetMascot;
use bevy::app::App;
use bevy::math::Vec3;
use bevy::prelude::{Event, Plugin, Query, Transform, Trigger};
use bevy_webview_wry::prelude::IpcTriggerExt;
use serde::Deserialize;

pub struct MenuScalePlugin;

impl Plugin for MenuScalePlugin {
    fn build(&self, app: &mut App) {
        app.add_ipc_trigger::<ChangeScale>("scale")
            .add_observer(apply_change_scale);
    }
}

#[derive(Debug, Clone, Deserialize, Event)]
struct ChangeScale {
    scale: f32,
}

fn apply_change_scale(
    trigger: Trigger<ChangeScale>,
    mut mascots: Query<&mut Transform>,
    webview: Query<&TargetMascot>,
) {
    let scale = trigger.scale;
    let Ok(mut tf) = webview
        .get(trigger.target())
        .and_then(|target| mascots.get_mut(target.0))
    else {
        return;
    };
    tf.scale = Vec3::new(scale, scale, 1.);
}
