use crate::components::GlobalViewport;
use crate::prelude::{AppWindows, window_local_pos};
use crate::system_param::bone_offsets::BoneOffsets;
use bevy::ecs::system::SystemParam;
use bevy::prelude::{Camera, Entity, Query, Transform, With, Without};
use bevy_vrm1::prelude::*;

#[derive(SystemParam)]
pub struct MascotTracker<'w, 's> {
    mascots: Query<'w, 's, &'static Transform, (With<Vrm>, Without<Camera>)>,
    camera: Cameras<'w, 's>,
    offsets: BoneOffsets<'w, 's>,
    windows: AppWindows<'w, 's>,
}

impl MascotTracker<'_, '_> {
    #[inline]
    pub fn tracking_on_sitting(&self, vrm: Entity, pos: GlobalViewport) -> Option<Transform> {
        // TODO: Adjust the position by multiplying by 0.9. It may be misaligned depending on the animation or model.
        self.tracking(vrm, pos, 0.9)
    }

    #[inline]
    pub fn tracking_on_drag(&self, vrm: Entity, pos: GlobalViewport) -> Option<Transform> {
        self.tracking(vrm, pos, 1.0)
    }

    pub fn tracking(&self, vrm: Entity, pos: GlobalViewport, adjust: f32) -> Option<Transform> {
        let tf = self.mascots.get(vrm).ok()?;
        let hips_offset = self.offsets.hips_offset(vrm)?;
        let (window_entity, window, _) = self.windows.find_by_global_viewport(pos)?;
        let viewport_pos = window_local_pos(window, pos);

        let mut cursor_pos =
            self.camera
                .to_world_by_viewport(window_entity, viewport_pos, tf.translation)?;
        let mut new_tf = *tf;
        cursor_pos.y -= hips_offset.y * adjust;
        new_tf.translation = cursor_pos;
        Some(new_tf)
    }
}
