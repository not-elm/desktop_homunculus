use crate::menu::TargetMascot;
use bevy::app::{App, Update};
use bevy::math::Vec3;
use bevy::prelude::{EventReader, Plugin, Query, Transform};
use bevy_webview_wry::ipc::{IpcEvent, IpcEventExt};
use serde::Deserialize;

pub struct MenuScalePlugin;

impl Plugin for MenuScalePlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_ipc_event::<ChangeScale>("scale")
            .add_systems(Update, change_scale);
    }
}

#[derive(Debug, Clone, Deserialize)]
struct ChangeScale {
    scale: f32,
}

fn change_scale(
    mut er: EventReader<IpcEvent<ChangeScale>>,
    mut mascots: Query<&mut Transform>,
    webview: Query<&TargetMascot>,
) {
    for event in er.read() {
        let scale = event.payload.scale;
        let Ok(mut tf) = webview
            .get(event.webview_entity)
            .and_then(|target| mascots.get_mut(target.0))
        else {
            continue;
        };
        tf.scale = Vec3::new(scale, scale, 1.);
    }
}
