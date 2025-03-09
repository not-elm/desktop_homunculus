use crate::vrm::extensions::vrmc_spring_bone::{Collider, ColliderShape, Spring, SpringJoint, VRMCSpringBone};
use crate::vrm::spring_bone::SpringJointProps;
use bevy::app::App;
use bevy::asset::{Assets, Handle};
use bevy::core::Name;
use bevy::gltf::GltfNode;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy::utils::HashMap;

pub struct SpringBoneRegistryPlugin;

impl Plugin for SpringBoneRegistryPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<SpringColliderRegistry>()
            .register_type::<SpringJointRegistry>()
            .register_type::<SpringNodeRegistry>();
    }
}

#[derive(Component, Deref, Reflect, PartialEq, Clone)]
#[reflect(Component)]
pub struct SpringColliderRegistry(HashMap<Name, ColliderShape>);

impl SpringColliderRegistry {
    pub fn new(
        colliders: &[Collider],
        node_assets: &Assets<GltfNode>,
        nodes: &[Handle<GltfNode>],
    ) -> Self {
        Self(colliders
            .iter()
            .filter_map(|collider| {
                let node_handle = nodes.get(collider.node)?;
                let node = node_assets.get(node_handle)?;
                Some((Name::new(node.name.clone()), collider.shape))
            })
            .collect()
        )
    }
}

#[derive(Component, Deref, Reflect)]
pub struct SpringJointRegistry(HashMap<Name, SpringJointProps>);

impl SpringJointRegistry {
    pub fn new(
        joints: &[SpringJoint],
        node_assets: &Assets<GltfNode>,
        nodes: &[Handle<GltfNode>],
    ) -> Self {
        Self(joints
            .iter()
            .filter_map(|joint| {
                let node_handle = nodes.get(joint.node)?;
                let node = node_assets.get(node_handle)?;
                let dir = joint.gravity_dir?;
                Some((Name::new(node.name.clone()), SpringJointProps {
                    drag_force: joint.drag_force?,
                    gravity_power: joint.gravity_power?,
                    hit_radius: joint.hit_radius?,
                    stiffness: joint.stiffness?,
                    gravity_dir: Vec3::new(dir[0], dir[1], dir[2]),
                }))
            })
            .collect()
        )
    }
}

#[derive(Component, Reflect, Debug, Default)]
pub struct SpringNode {
    pub center: Option<Name>,
    pub joints: Vec<Name>,
    pub colliders: Vec<Name>,
}

#[derive(Component, Deref, Reflect)]
pub struct SpringNodeRegistry(pub Vec<SpringNode>);

impl SpringNodeRegistry {
    pub fn new(
        spring_bone: &VRMCSpringBone,
        node_assets: &Assets<GltfNode>,
        nodes: &[Handle<GltfNode>],
    ) -> Self {
        Self(spring_bone
            .springs
            .iter()
            .map(|spring| SpringNode {
                joints: spring
                    .joints
                    .iter()
                    .filter_map(|joint| get_node_name(joint.node, node_assets, nodes))
                    .collect(),
                colliders: collider_names(spring_bone, spring, node_assets, nodes),
                center: spring
                    .center
                    .and_then(|index| get_node_name(index, node_assets, nodes)),
            })
            .collect()
        )
    }
}

fn collider_names(
    spring_bone: &VRMCSpringBone,
    spring: &Spring,
    node_assets: &Assets<GltfNode>,
    nodes: &[Handle<GltfNode>],
) -> Vec<Name> {
    let Some(collider_groups) = spring.collider_groups.as_ref() else {
        return vec![];
    };
    spring_bone
        .spring_colliders(collider_groups)
        .iter()
        .flat_map(|collider| get_node_name(collider.node, node_assets, nodes))
        .collect()
}

fn get_node_name(
    node_index: usize,
    node_assets: &Assets<GltfNode>,
    nodes: &[Handle<GltfNode>],
) -> Option<Name> {
    let node_handle = nodes.get(node_index)?;
    let node = node_assets.get(node_handle)?;
    Some(Name::new(node.name.clone()))
}