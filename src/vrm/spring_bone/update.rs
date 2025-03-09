use crate::vrm::extensions::vrmc_spring_bone::ColliderShape;
use crate::vrm::spring_bone::{SpringJointProps, SpringJointState, SpringRoot};
use bevy::app::App;
use bevy::math::Vec3;
use bevy::prelude::{Entity, GlobalTransform, Parent, Plugin, PostUpdate, Quat, Query, Res, Transform, Without};
use bevy::time::Time;

pub struct SpringBoneUpdatePlugin;

impl Plugin for SpringBoneUpdatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, update_spring_bones);
    }
}

fn update_spring_bones(
    mut transforms: Query<(&mut Transform, &mut GlobalTransform)>,
    mut joints: Query<(&Parent, &mut SpringJointState, &SpringJointProps), Without<SpringRoot>>,
    spring_roots: Query<&SpringRoot>,
    colliders: Query<&ColliderShape>,
    time: Res<Time>,
) {
    let delta_time = time.delta_secs();
    for spring in spring_roots.iter() {
        for joint in spring.joints.iter().copied() {
            let Ok((parent, mut state, props)) = joints.get_mut(joint) else {
                continue;
            };
            let parent_gtf = transforms.get(parent.get()).map(|(_, gtf)| *gtf).unwrap_or_default();
            let Ok(joint_global_pos) = transforms.get(joint).map(|(_, gtf)| gtf.translation()) else {
                continue;
            };

            let inertia = (state.current_tail - state.prev_tail) * (1. - props.drag_force);
            let stiffness = delta_time * (parent_gtf.rotation() * state.initial_local_rotation * state.bone_axis * props.stiffness);
            let external = delta_time * props.gravity_dir * props.gravity_power;

            let next_tail = state.current_tail + inertia + stiffness + external;
            let mut next_tail = joint_global_pos + (next_tail - joint_global_pos).normalize() * state.bone_length;

            state.prev_tail = state.current_tail;
            state.current_tail = next_tail;

            collision(&mut next_tail, spring.colliders.iter().copied(), joint, &transforms, &colliders);

            let to = (parent_gtf.compute_matrix() * state.initial_local_matrix)
                .inverse()
                .transform_point3(next_tail)
                .normalize();

            let Ok((mut tf, mut gtf)) = transforms.get_mut(joint) else {
                continue;
            };
            tf.rotation = state.initial_local_rotation * Quat::from_rotation_arc(state.bone_axis, to);
            *gtf = parent_gtf.mul_transform(*tf);
        }
    }
}

fn collision(
    next_tail: &mut Vec3,
    collider_entities: impl Iterator<Item=Entity>,
    joint_entity: Entity,
    transforms: &Query<(&mut Transform, &mut GlobalTransform)>,
    colliders: &Query<&ColliderShape>,
) {
    let Ok(joint_shape) = colliders.get(joint_entity) else {
        return;
    };
    for collider in collider_entities {
        let Ok(collider_shape) = colliders.get(collider) else {
            continue;
        };
        let Ok((_, collider_gtf)) = transforms.get(collider) else {
            continue;
        };
        let (dir, distance) = collider_shape.calc_collision(
            *next_tail,
            collider_gtf,
            joint_shape.radius(),
        );
        if distance < 0. {
            *next_tail += dir * distance;
        }
    }
}