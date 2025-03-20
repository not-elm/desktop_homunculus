use crate::mascot::action::MascotActionExt;
use crate::mascot::MascotEntity;
use bevy::app::App;
use bevy::prelude::{Commands, In, Plugin, Query, Transform, Vec3};
use bevy_flurx::action::once;
use bevy_flurx::prelude::Omit;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleActionParams {
    pub scale: Vec3,
}

pub struct ScaleActionPlugin;

impl ScaleActionPlugin {
    pub const ID: &'static str = "scale";
}

impl Plugin for ScaleActionPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_mascot_action(Self::ID, |mascot, params: ScaleActionParams| {
            once::run(scale).with((mascot, params.scale)).omit()
        });
    }
}

fn scale(
    In((mascot, scale)): In<(MascotEntity, Vec3)>,
    mut commands: Commands,
    transforms: Query<&Transform>,
) {
    let Ok(tf) = transforms.get(mascot.0) else {
        return;
    };
    commands.entity(mascot.0).insert(Transform { scale, ..*tf });
}
