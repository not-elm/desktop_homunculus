use bevy::ecs::system::SystemParam;
use bevy::math::Vec3;
use bevy::prelude::{Camera, Entity, GlobalTransform, Query, With, Without};
use bevy_vrm1::prelude::{ChildSearcher, HipsBoneEntity};
use bevy_vrm1::vrm::{Vrm, VrmBone};

#[derive(SystemParam)]
pub struct BoneOffsets<'w, 's> {
    vrms: Query<
        'w,
        's,
        (&'static GlobalTransform, &'static HipsBoneEntity),
        (With<Vrm>, Without<Camera>),
    >,
    bones: Query<'w, 's, &'static GlobalTransform, (Without<Vrm>, Without<Camera>)>,
    searcher: ChildSearcher<'w, 's>,
}

impl BoneOffsets<'_, '_> {
    pub fn offset(&self, vrm: Entity, bone: &VrmBone) -> Option<Vec3> {
        let bone_entity = self.searcher.find_by_bone_name(vrm, bone)?;
        let bone_gtf = self.bones.get(bone_entity).ok()?;
        let (gtf, _) = self.vrms.get(vrm).ok()?;
        Some(bone_gtf.translation() - gtf.translation())
    }

    #[inline]
    pub fn hips_offset(&self, vrm: Entity) -> Option<Vec3> {
        let (gtf, HipsBoneEntity(hips)) = self.vrms.get(vrm).ok()?;
        let hips_gf = self.bones.get(*hips).ok()?;
        Some(hips_gf.translation() - gtf.translation())
    }
}
