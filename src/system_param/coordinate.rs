use crate::application_windows::PrimaryCamera;
use crate::system_param::cameras::Cameras;
use crate::system_param::monitors::Monitors;
use bevy::ecs::system::SystemParam;
use bevy::math::Vec3;
use bevy::prelude::{Entity, Query, With};
use bevy::render::view::RenderLayers;

#[derive(SystemParam)]
pub struct Coordinate<'w, 's> {
    pub cameras: Cameras<'w, 's>,
    pub monitors: Monitors<'w, 's>,
    primary_camera: Query<'w, 's, Entity, With<PrimaryCamera>>,
}

impl Coordinate<'_, '_> {
    #[inline]
    pub fn new_render_layers_if_overall_monitor(
        &self,
        current_render_layers: &RenderLayers,
        world_pos: Vec3,
    ) -> Option<(Vec3, &RenderLayers)> {
        let viewport_pos = self.cameras.to_viewport_pos(current_render_layers, world_pos)?;
        let (new_viewport, new_layers) = self.monitors.new_render_layers_if_overall_monitor(current_render_layers, viewport_pos)?;
        Some((self.cameras.to_world_pos(new_layers, new_viewport)?, new_layers))
    }

    /// If the passed position is outside the screen, return the default position and layers of the mascot.
    pub fn initial_mascot_pos_and_layers(&self, load_pos: Vec3) -> (Vec3, RenderLayers) {
        match self.cameras.find_camera_from_world_pos(load_pos) {
            Some((_, _, layers)) => (load_pos, layers.clone()),
            None => {
                let entity = self.primary_camera.single();
                let (pos, layers) = self.cameras.cameras.get(entity)
                    .ok()
                    .and_then(|(camera, gtf, layers)| {
                        let pos = camera.viewport_to_world_2d(gtf, camera.viewport.as_ref().unwrap().physical_size.as_vec2() / 2.).ok()?;
                        Some((pos, layers.clone()))
                    }).unwrap_or_default();
                (pos.extend(0.), layers)
            }
        }
    }
}