use crate::mascot::action::MascotActionExt;
use crate::mascot::MascotEntity;
use bevy::prelude::*;
use bevy_flurx::action::once;
use bevy_flurx::prelude::{ActionSeed, Omit};
use bevy_vrma::vrma::animation::play::PlayVrma;
use bevy_vrma::vrma::{VrmaEntity, VrmaPath};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect(Serialize, Deserialize)]
pub struct MascotVrmaActionParams {
    pub vrma_path: VrmaPath,
    pub repeat: bool,
}

pub struct MascotVrmaActionPlugin;

impl MascotVrmaActionPlugin {
    pub const ID: &'static str = "vrma";
}

impl Plugin for MascotVrmaActionPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_mascot_action(Self::ID, vrma_animation_action);
    }
}

fn vrma_animation_action(
    mascot: MascotEntity,
    params: MascotVrmaActionParams,
) -> ActionSeed {
    once::run(vrma_animation).with((mascot, params)).omit()
}

fn vrma_animation(
    In((mascot, params)): In<(MascotEntity, MascotVrmaActionParams)>,
    mut commands: Commands,
    vrma: Query<(Entity, &VrmaPath)>,
) {
    if let Some(vrma_entity) = find_vrma_from_path_buff(&params.vrma_path, vrma) {
        info!("Play VRMA({:?}) repeat={}", params.vrma_path, params.repeat);
        commands.entity(mascot.0).trigger(PlayVrma {
            vrma: VrmaEntity(vrma_entity),
            repeat: params.repeat,
        });
    }
}

fn find_vrma_from_path_buff(
    vrma_path: &VrmaPath,
    vrma: Query<(Entity, &VrmaPath)>,
) -> Option<Entity> {
    vrma.iter()
        .find_map(|(entity, path)| (path == vrma_path).then_some(entity))
}

#[cfg(test)]
mod tests {
    use crate::mascot::action::vrma::find_vrma_from_path_buff;
    use crate::tests::{test_app, TestResult};
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::{Commands, Entity, Query};
    use bevy_vrma::vrma::VrmaPath;
    use std::path::PathBuf;

    #[test]
    fn find_vrma_from_path() -> TestResult {
        let mut app = test_app();
        let entity = app.world_mut().run_system_once(|mut commands: Commands| {
            commands.spawn(VrmaPath(PathBuf::from("/root"))).id()
        })?;
        let target_vrma: Option<Entity> =
            app.world_mut()
                .run_system_once(|vrma: Query<(Entity, &VrmaPath)>| {
                    find_vrma_from_path_buff(&VrmaPath(PathBuf::from("/root")), vrma)
                })?;
        assert_eq!(target_vrma, Some(entity));
        Ok(())
    }
}
