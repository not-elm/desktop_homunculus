use crate::mascot::{Mascot, MascotEntity};
use crate::system_param::bone_offsets::BoneOffsets;
use crate::system_param::cameras::Cameras;
use bevy::ecs::system::SystemParam;
use bevy::math::Vec2;
use bevy::prelude::{Camera, Query, Transform, With, Without};
use bevy::render::view::RenderLayers;

#[derive(SystemParam)]
pub struct MascotTracker<'w, 's> {
    mascots: Query<'w, 's, (
        &'static mut Transform,
        &'static RenderLayers,
    ), (With<Mascot>, Without<Camera>)>,
    camera: Cameras<'w, 's>,
    offsets: BoneOffsets<'w, 's>,
}

impl MascotTracker<'_, '_> {
    #[inline]
    pub fn tracking_on_sitting(
        &self,
        mascot: MascotEntity,
        view_port_pos: Vec2,
    ) -> Option<Transform> {
        // TODO: Adjust the position by multiplying by 0.9. It may be misaligned depending on the animation or model.
        self.tracking(mascot, view_port_pos, 0.9)
    }

    #[inline]
    pub fn tracking_on_drag(
        &self,
        mascot: MascotEntity,
        view_port_pos: Vec2,
    ) -> Option<Transform> {
        self.tracking(mascot, view_port_pos, 1.0)
    }

    fn tracking(
        &self,
        mascot: MascotEntity,
        view_port_pos: Vec2,
        adjust: f32,
    ) -> Option<Transform> {
        let (tf, render_layers) = self.mascots.get(mascot.0).ok()?;
        let hips_offset = self.offsets.hips_offset(mascot)?;
        let mut cursor_pos = self.camera.to_world_pos(render_layers, view_port_pos)?;

        let mut new_tf = *tf;
        cursor_pos.y -= hips_offset.y * adjust;
        new_tf.translation = cursor_pos;
        Some(new_tf)
    }
}