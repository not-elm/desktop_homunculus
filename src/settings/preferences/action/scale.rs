use crate::mascot::MascotEntity;
use crate::settings::preferences::action::ExecuteMascotAction;
use bevy::app::Update;
use bevy::prelude::{Commands, In, Query, Reflect, Transform, Vec3};
use bevy_flurx::action::once;
use bevy_flurx::prelude::ReactorTask;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Reflect, Default)]
pub struct ScaleAction {
    pub scale: Vec3,
}

impl ExecuteMascotAction for ScaleAction {
    async fn execute(&self, mascot: MascotEntity, task: &ReactorTask) {
        task.will(Update, once::run(change_scale).with((mascot, self.scale))).await;
    }
}

fn change_scale(
    In((mascot, scale)): In<(MascotEntity, Vec3)>,
    mut commands: Commands,
    transforms: Query<&Transform>,
) {
    if let Ok(transform) = transforms.get(mascot.0) {
        commands.entity(mascot.0).insert(Transform {
            scale,
            ..*transform
        });
    }
}