use crate::system_param::child_searcher::ChildSearcher;
use crate::vrm::extensions::VrmNode;
use crate::vrm::{BonePgRestQuaternion, BoneRestTransform, VrmBone, VrmHipsBoneTo};
use bevy::asset::{Assets, Handle};
use bevy::core::Name;
use bevy::gltf::GltfNode;
use bevy::hierarchy::Children;
use bevy::math::Quat;
use bevy::prelude::{Added, App, Commands, Component, Deref, Entity, Plugin, Query, Reflect, Transform, Update};
use bevy::utils::HashMap;

pub struct VrmAttachBonesPlugin;

impl Plugin for VrmAttachBonesPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<HumanoidBoneNodes>()
            .add_systems(Update, attach_bones);
    }
}

#[derive(Component, Deref, Reflect)]
pub struct HumanoidBoneNodes(HashMap<VrmBone, Name>);

impl HumanoidBoneNodes {
    pub fn new(
        bones: &HashMap<String, VrmNode>,
        node_assets: &Assets<GltfNode>,
        nodes: &[Handle<GltfNode>],
    ) -> Self {
        Self(bones
            .iter()
            .filter_map(|(name, target_node)| {
                let node_handle = nodes.get(target_node.node)?;
                let node = node_assets.get(node_handle)?;
                Some((VrmBone(name.clone()), Name::new(node.name.clone())))
            })
            .collect()
        )
    }
}

fn attach_bones(
    mut commands: Commands,
    searcher: ChildSearcher,
    mascots: Query<(Entity, &HumanoidBoneNodes), Added<Children>>,
    transforms: Query<(&Transform, Option<&Children>)>,
) {
    for (mascot_entity, humanoid_bones) in mascots.iter() {
        for (bone, name) in humanoid_bones.iter() {
            let Some(bone_entity) = searcher.find_from_name(mascot_entity, name.as_str()) else {
                continue;
            };
            commands.entity(bone_entity).insert(bone.clone());
            // Use hips when sitting on window.
            if bone.0 == "hips" {
                commands.entity(mascot_entity).insert(VrmHipsBoneTo(bone_entity));
                recursive_attach_pg_resets(
                    &mut commands,
                    bone_entity,
                    Quat::IDENTITY,
                    &transforms,
                );
            }
        }
    }
}

fn recursive_attach_pg_resets(
    commands: &mut Commands,
    bone_entity: Entity,
    rest: Quat,
    transforms: &Query<(&Transform, Option<&Children>)>,
) {
    let Ok((tf, children)) = transforms.get(bone_entity) else {
        return;
    };
    let pg_rest = BonePgRestQuaternion(tf.rotation * rest);
    commands.entity(bone_entity).insert((
        BoneRestTransform(*tf),
        pg_rest,
    ));
    if let Some(children) = children {
        for child in children.iter() {
            recursive_attach_pg_resets(commands, *child, pg_rest.0, transforms);
        }
    }
}
