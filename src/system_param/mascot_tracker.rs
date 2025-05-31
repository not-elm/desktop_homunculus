use crate::mascot::{Mascot, MascotEntity};
use crate::system_param::bone_offsets::BoneOffsets;
use crate::system_param::windows::{window_local_pos, Windows};
use crate::system_param::GlobalScreenPos;
use bevy::ecs::system::SystemParam;
use bevy::prelude::{Camera, Query, Transform, With, Without};
use bevy_vrm1::system_param::cameras::Cameras;

#[derive(SystemParam)]
pub struct MascotTracker<'w, 's> {
    mascots: Query<'w, 's, &'static Transform, (With<Mascot>, Without<Camera>)>,
    camera: Cameras<'w, 's>,
    offsets: BoneOffsets<'w, 's>,
    windows: Windows<'w, 's>,
}

impl MascotTracker<'_, '_> {
    #[inline]
    pub fn tracking_on_sitting(
        &self,
        mascot: MascotEntity,
        pos: GlobalScreenPos,
    ) -> Option<Transform> {
        // TODO: Adjust the position by multiplying by 0.9. It may be misaligned depending on the animation or model.
        self.tracking(mascot, pos, 0.9)
    }

    #[inline]
    pub fn tracking_on_drag(
        &self,
        mascot: MascotEntity,
        pos: GlobalScreenPos,
    ) -> Option<Transform> {
        self.tracking(mascot, pos, 1.0)
    }

    pub fn tracking(
        &self,
        mascot: MascotEntity,
        pos: GlobalScreenPos,
        adjust: f32,
    ) -> Option<Transform> {
        let tf = self.mascots.get(mascot.0).ok()?;
        let hips_offset = self.offsets.hips_offset(mascot)?;
        let (window_entity, window, _) = self.windows.find_window_from_global_screen_pos(pos)?;
        let viewport_pos = window_local_pos(window, pos);
        let mut cursor_pos = self
            .camera
            .to_world_pos_from_viewport(window_entity, viewport_pos)?;
        let mut new_tf = *tf;
        cursor_pos.y -= hips_offset.y * adjust;
        new_tf.translation = cursor_pos;
        Some(new_tf)
    }
}
