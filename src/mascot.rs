use crate::mascot::action::MascotActionPlugin;
use crate::mascot::drag::MascotDragPlugin;
use crate::mascot::render_layers::MascotRenderLayersPlugin;
use crate::mascot::sitting::MascotSittingPlugin;
use bevy::app::{App, Plugin};
use bevy::asset::Handle;
use bevy::prelude::{Component, Entity, Gltf, Reflect, ReflectComponent, ReflectDeserialize, ReflectSerialize};
use serde::{Deserialize, Serialize};

mod drag;
pub mod visibility;
mod render_layers;

pub mod sitting;
pub mod action;

#[derive(Component, Reflect, Serialize, Deserialize, Debug)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Mascot;

#[repr(transparent)]
#[derive(Reflect, Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[reflect(Serialize, Deserialize)]
pub struct MascotEntity(pub Entity);

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
struct MascotGltfHandle(Option<Handle<Gltf>>);

pub struct DesktopMascotPlugin;

impl Plugin for DesktopMascotPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Mascot>()
            .register_type::<MascotEntity>()
            .add_plugins((
                MascotDragPlugin,
                MascotRenderLayersPlugin,
                MascotSittingPlugin,
                MascotActionPlugin,
            ));
    }
}


