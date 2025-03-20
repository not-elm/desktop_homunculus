use crate::mascot::{Mascot, MascotEntity};
use bevy::ecs::system::SystemParam;
use bevy::math::Vec3;
use bevy::prelude::{Camera, GlobalTransform, Query, With, Without};
use bevy_vrma::vrm::VrmHipsBoneTo;

#[derive(SystemParam)]
pub struct BoneOffsets<'w, 's> {
    mascots: Query<
        'w,
        's,
        (&'static GlobalTransform, &'static VrmHipsBoneTo),
        (With<Mascot>, Without<Camera>),
    >,
    bones: Query<'w, 's, &'static GlobalTransform, (Without<Mascot>, Without<Camera>)>,
}

impl BoneOffsets<'_, '_> {
    #[inline]
    pub fn hips_offset(
        &self,
        mascot: MascotEntity,
    ) -> Option<Vec3> {
        let (gf, VrmHipsBoneTo(hips)) = self.mascots.get(mascot.0).ok()?;
        let hips_gf = self.bones.get(*hips).ok()?;
        Some(hips_gf.translation() - gf.translation())
    }
}
