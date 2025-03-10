use crate::system_param::child_searcher::ChildSearcher;
use crate::vrm::humanoid_bone::{Hips, HumanoidBoneRegistry};
use crate::vrm::{BoneRestGlobalTransform, BoneRestTransform};
use crate::vrma::retarget::CurrentRetargeting;
use crate::vrma::{RetargetSource, RetargetTo};
use bevy::app::{App, Update};
use bevy::core::Name;
use bevy::hierarchy::Children;
use bevy::log::error;
use bevy::math::Vec3;
use bevy::prelude::{Added, Changed, Component, Entity, GlobalTransform, ParallelCommands, Plugin, Query, Reflect, Transform, With, Without};

pub struct VrmaRetargetingBonePlugin;

impl Plugin for VrmaRetargetingBonePlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<RetargetBoneTo>()
            .add_systems(Update, (
                retarget_bones_to_mascot,
                bind_bone_rotations,
            ));
    }
}

#[derive(Debug, Component, Reflect)]
struct RetargetBoneTo(pub Entity);

fn retarget_bones_to_mascot(
    par_commands: ParallelCommands,
    bones: Query<(Entity, &RetargetTo, &HumanoidBoneRegistry), Added<Children>>,
    transforms: Query<(&Transform, &GlobalTransform)>,
    names: Query<&Name>,
    searcher: ChildSearcher,
) {
    bones.par_iter().for_each(|(entity, retarget, humanoid_bones)| {
        for (bone, name) in humanoid_bones.iter() {
            let Some(src_bone_entity) = searcher.find_from_name(entity, name.as_str()) else {
                continue;
            };
            let Some(dist_bone_entity) = searcher.find_from_bone_name(retarget.0, bone) else {
                let dist_name = names.get(retarget.0).unwrap();
                error!("[Bone] {dist_name}'s {bone} not found");
                continue;
            };
            let Ok((src_tf, src_gtf)) = transforms.get(src_bone_entity) else {
                continue;
            };

            par_commands.command_scope(|mut commands| {
                commands.entity(src_bone_entity).insert((
                    RetargetSource,
                    BoneRestTransform(*src_tf),
                    BoneRestGlobalTransform(*src_gtf),
                    RetargetBoneTo(dist_bone_entity),
                ));
            });
        }
    });
}

fn bind_bone_rotations(
    par_commands: ParallelCommands,
    sources: Query<(
        &RetargetBoneTo,
        &Transform,
        &BoneRestGlobalTransform,
        Option<&Hips>,
    ), (Changed<Transform>, With<CurrentRetargeting>)>,
    dist_bones: Query<(
        &Transform,
        &BoneRestGlobalTransform,
    ), Without<CurrentRetargeting>>,
) {
    sources.par_iter().for_each(|(retarget_bone_to, src_pose_tf, src_rest_gtf, maybe_hips)| {
        let Ok((dist_pose_tf, dist_rest_gtf)) = dist_bones.get(retarget_bone_to.0) else {
            return;
        };
        let transform = Transform {
            rotation: src_pose_tf.rotation,
            translation: if maybe_hips.is_some() {
                calc_hips_position(
                    src_rest_gtf.0.translation(),
                    src_pose_tf.translation,
                    dist_rest_gtf.0.translation(),
                )
            } else {
                dist_pose_tf.translation
            },
            scale: dist_pose_tf.scale,
        };
        par_commands.command_scope(|mut commands| {
            commands.entity(retarget_bone_to.0).insert(transform);
        });
    });
}

#[inline]
fn calc_scaling(
    dist_rest_global_pos: Vec3,
    source_rest_global_pos: Vec3,
) -> f32 {
    dist_rest_global_pos.y / source_rest_global_pos.y
}

#[inline]
fn calc_delta(
    source_pose_pos: Vec3,
    source_rest_global_pos: Vec3,
) -> Vec3 {
    source_pose_pos - source_rest_global_pos
}

fn calc_hips_position(
    source_rest_global_pos: Vec3,
    source_pose_pos: Vec3,
    dist_rest_global_pos: Vec3,
) -> Vec3 {
    let delta = calc_delta(source_pose_pos, source_rest_global_pos);
    let scaling = calc_scaling(dist_rest_global_pos, source_rest_global_pos);
    dist_rest_global_pos + delta * scaling
}

#[cfg(test)]
mod tests {
    use crate::vrma::retarget::bone::{calc_delta, calc_scaling};
    use bevy::math::Vec3;

    #[test]
    fn test_scaling() {
        let scalling = calc_scaling(Vec3::splat(1.), Vec3::splat(2.));
        assert!((scalling - 0.5) < 0.001);
    }

    #[test]
    fn test_delta() {
        let delta = calc_delta(Vec3::splat(1.), Vec3::splat(2.));
        assert_eq!(delta, Vec3::splat(-1.));
    }
}