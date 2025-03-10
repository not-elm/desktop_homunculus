use crate::mascot::Mascot;
use crate::system_param::cameras::Cameras;
use crate::vrm::expressions::VrmExpressionRegistry;
use crate::vrm::extensions::VrmExtensions;
use crate::vrm::humanoid_bone::HumanoidBoneRegistry;
use crate::vrm::loader::{Vrm, VrmHandle};
use crate::vrm::spring_bone::registry::*;
use crate::vrm::VrmPath;
use crate::vrma::load::RequestLoadVrma;
use bevy::app::{App, Update};
use bevy::asset::Assets;
use bevy::core::Name;
use bevy::gltf::GltfNode;
use bevy::log::{error, info};
use bevy::prelude::{Commands, DespawnRecursiveExt, Entity, EventWriter, Plugin, Query, Res, Transform};
use bevy::scene::SceneRoot;

pub struct VrmSpawnPlugin;

impl Plugin for VrmSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_vrm);
    }
}

fn spawn_vrm(
    mut commands: Commands,
    mut ew: EventWriter<RequestLoadVrma>,
    cameras: Cameras,
    node_assets: Res<Assets<GltfNode>>,
    vrm_assets: Res<Assets<Vrm>>,
    handles: Query<(Entity, &VrmHandle, &Transform, &VrmPath)>,
) {
    for (vrm_handle_entity, handle, tf, vrm_path) in handles.iter() {
        let Some(vrm) = vrm_assets.get(handle.0.id()) else {
            continue;
        };
        commands.entity(vrm_handle_entity).despawn_recursive();

        let Some(scene) = vrm.gltf.scenes.first() else {
            continue;
        };
        let extensions = match VrmExtensions::from_gltf(&vrm.gltf) {
            Ok(extensions) => extensions,
            Err(e) => {
                error!("[VRM] {e}");
                continue;
            }
        };

        #[cfg(feature = "develop")]
        create_vrm_json_for_debug(&vrm.gltf, &extensions);

        info!("Spawned mascot({:?}): {:?}\nposition: {:?}", extensions.name(), vrm_path.0, tf.translation);
        let mut cmd = commands.spawn((
            Mascot,
            SceneRoot(scene.clone()),
            vrm_path.clone(),
            *tf,
            cameras.all_layers(),
            VrmExpressionRegistry::new(&extensions, &node_assets, &vrm.gltf.nodes),
            HumanoidBoneRegistry::new(
                &extensions.vrmc_vrm.humanoid.human_bones,
                &node_assets,
                &vrm.gltf.nodes,
            ),
            Name::new(extensions.name().unwrap_or_else(|| "VRM".to_string())),
        ));

        if let Some(spring_bone) = extensions.vrmc_spring_bone.as_ref() {
            cmd.insert((
                SpringJointRegistry::new(
                    &spring_bone.all_joints(),
                    &node_assets,
                    &vrm.gltf.nodes,
                ),
                SpringColliderRegistry::new(
                    &spring_bone.colliders,
                    &node_assets,
                    &vrm.gltf.nodes,
                ),
                SpringNodeRegistry::new(
                    spring_bone,
                    &node_assets,
                    &vrm.gltf.nodes,
                ),
            ));
        }
        ew.send(RequestLoadVrma);
    }
}

#[cfg(feature = "develop")]
fn create_vrm_json_for_debug(
    gltf: &bevy::prelude::Gltf,
    extensions: &VrmExtensions,
) {
    let dir = std::path::PathBuf::from("test").join("vrm");
    if !dir.exists() {
        std::fs::create_dir_all(&dir).unwrap();
    }
    let path = dir.join(format!("{}.json", extensions.name().unwrap_or_default()));
    std::fs::write(path, serde_json::to_string_pretty(&crate::vrm::extensions::obtain_extensions(gltf).unwrap()).unwrap()).unwrap();
}
