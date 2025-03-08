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
        Some((self.cameras.to_world_pos_from_viewport(new_layers, new_viewport)?, new_layers))
    }

    pub fn default_mascot_pos_and_layers(&self) -> (Vec3, RenderLayers) {
        let entity = self.primary_camera.single();
        let (pos, layers) = self.cameras
            .cameras
            .get(entity)
            .ok()
            .and_then(|(camera, gtf, layers)| {
                let pos = camera.ndc_to_world(gtf, Vec3::default().with_y(0.))?;
                Some((pos, layers.clone()))
            })
            .unwrap_or_default();
        (pos.with_z(0.), layers)
    }

    pub fn initial_mascot_pos_and_layers(
        &self,
        ndc: Vec3,
        monitor_name: &Option<String>,
    ) -> Option<(Vec3, RenderLayers)> {
        let (_, _, layers) = self.monitors.find_monitor_from_name(monitor_name.as_ref()?)?;
        let world_pos = self.cameras.to_world_pos(layers, ndc)?;
        Some((world_pos, layers.clone()))
    }
}