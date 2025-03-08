use crate::application_windows::PrimaryCamera;
use crate::system_param::cameras::Cameras;
use crate::system_param::monitors::Monitors;
use bevy::ecs::system::SystemParam;
use bevy::math::Vec3;
use bevy::prelude::{Entity, Query, Vec3Swizzles, With};

#[derive(SystemParam)]
pub struct Coordinate<'w, 's> {
    pub cameras: Cameras<'w, 's>,
    pub monitors: Monitors<'w, 's>,
    primary_camera: Query<'w, 's, Entity, With<PrimaryCamera>>,
}

impl Coordinate<'_, '_> {
    pub fn default_mascot_pos_and_layers(&self) -> Vec3 {
        let entity = self.primary_camera.single();
        self.cameras
            .cameras
            .get(entity)
            .ok()
            .and_then(|(camera, gtf, _)| {
                let pos = camera.ndc_to_world(gtf, Vec3::default().with_y(0.))?;
                Some(pos.with_z(0.))
            })
            .unwrap_or_default()
    }

    pub fn mascot_position(
        &self,
        viewport_pos: Vec3,
        monitor_name: &Option<String>,
    ) -> Option<Vec3> {
        let (_, _, layers) = self.monitors.find_monitor_from_name(monitor_name.as_ref()?)?;
        let (camera, gtf, _) = self.cameras.find_camera_from_layers(layers)?;
        let world_pos = camera.viewport_to_world_2d(gtf, viewport_pos.xy()).ok()?;
        Some(world_pos.extend(viewport_pos.z))
    }
}