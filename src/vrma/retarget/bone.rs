use crate::system_param::child_searcher::ChildSearcher;
use crate::vrm::spawn::HumanoidBoneNodes;
use crate::vrm::{BonePgRestQuaternion, BoneRestTransform};
use crate::vrma::retarget::CurrentRetargeting;
use crate::vrma::{RetargetSource, RetargetTo};
use bevy::app::{App, Update};
use bevy::core::Name;
use bevy::hierarchy::Children;
use bevy::log::error;
use bevy::math::{Quat, Vec3};
use bevy::prelude::{Added, Changed, Component, Entity, ParallelCommands, Plugin, Query, Reflect, Transform, With, Without};

pub struct VrmaRetargetingBonePlugin;

impl Plugin for VrmaRetargetingBonePlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<RetargetBoneTo>()
            .register_type::<PreviousPosition>()
            .add_systems(Update, (
                retarget_bones_to_mascot,
                bind_bones,
            ));
    }
}

#[derive(Debug, Component, Reflect)]
struct RetargetBoneTo(pub Entity);

#[derive(Debug, Component, Reflect)]
struct PreviousPosition(Vec3);

fn retarget_bones_to_mascot(
    par_commands: ParallelCommands,
    bones: Query<(Entity, &RetargetTo, &HumanoidBoneNodes), Added<Children>>,
    transforms: Query<&Transform>,
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
            let Ok(src_transform) = transforms.get(src_bone_entity) else {
                continue;
            };

            let initial_pos = fix_position(src_transform.translation);
            par_commands.command_scope(|mut commands| {
                commands.entity(src_bone_entity).insert((
                    PreviousPosition(initial_pos),
                    RetargetSource,
                    BoneRestTransform(*src_transform),
                    RetargetBoneTo(dist_bone_entity),
                ));
            });
        }
    });
}

fn bind_bones(
    par_commands: ParallelCommands,
    sources: Query<(
        Entity,
        &RetargetBoneTo,
        &Transform,
        &BoneRestTransform,
        &BonePgRestQuaternion,
    ), (Changed<Transform>, With<CurrentRetargeting>)>,
    dist_bones: Query<(
        &BoneRestTransform,
        &BonePgRestQuaternion,
    ), Without<CurrentRetargeting>>,
) {
    sources.par_iter().for_each(|(entity, retarget_bone_to, src_pose, src_rest, src_pg_rest)| {
        let Ok((dist_rest, dist_pg_rest)) = dist_bones.get(retarget_bone_to.0) else {
            return;
        };
        let src_pos = fix_position(src_pose.translation);
        let anim_q = fix_rotation(src_pg_rest.0 *
            src_pose.rotation *
            src_rest.0.rotation.inverse() *
            src_pg_rest.0.inverse());
        let transform = Transform {
            translation: fix_position(src_pose.translation),
            rotation: dist_pg_rest.0.inverse() * anim_q * dist_pg_rest.0 * dist_rest.0.rotation,
            scale: src_pose.scale,
        };
        par_commands.command_scope(|mut commands| {
            commands.entity(entity).insert(PreviousPosition(src_pos));
            commands.entity(retarget_bone_to.0).insert(transform);
        });
    });
}

#[inline]
fn fix_rotation(quat: Quat) -> Quat {
    let r = Quat::from_rotation_x(std::f32::consts::FRAC_PI_2);
    r * quat * r.inverse()
}

#[inline]
fn fix_position(pos: Vec3) -> Vec3 {
    Vec3::new(pos.x, -pos.z, pos.y)
}