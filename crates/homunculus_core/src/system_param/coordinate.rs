use crate::prelude::{AppWindows, GlobalViewport, Monitors, PrimaryCamera, window_local_pos};
use bevy::ecs::system::SystemParam;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::*;
use bevy_vrm1::prelude::*;

#[derive(SystemParam)]
pub struct Coordinate<'w, 's, Camera: Component = Camera3d> {
    pub cameras: Cameras<'w, 's, Camera>,
    pub monitors: Monitors<'w, 's>,
    pub windows: AppWindows<'w, 's>,
    primary_camera: Query<'w, 's, Entity, With<PrimaryCamera>>,
}

impl<Camera: Component> Coordinate<'_, '_, Camera> {
    pub fn default_mascot_pos_and_layers(&self) -> Vec3 {
        let entity = self.primary_camera.single().unwrap();
        self.cameras
            .cameras
            .get(entity)
            .ok()
            .and_then(|(_, camera, gtf, _)| {
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
        let (_, _, layers) = self
            .monitors
            .find_monitor_from_name(monitor_name.as_ref()?)?;
        let (_, camera, gtf, _) = self.cameras.find_camera_from_layers(layers)?;
        let world_pos = camera.viewport_to_world_2d(gtf, viewport_pos.xy()).ok()?;
        Some(world_pos.extend(viewport_pos.z))
    }

    pub fn to_world_pos_from_window(
        &self,
        source_window: Entity,
        viewport_pos: Vec2,
        vrm_pos: Vec3,
    ) -> Option<Vec3> {
        let global_cursor = self
            .windows
            .to_global_viewport(source_window, viewport_pos)?;
        let (window_entity, window, _) = self.windows.find_by_global_viewport(global_cursor)?;
        self.cameras.to_world_by_viewport(
            window_entity,
            window_local_pos(window, global_cursor),
            vrm_pos,
        )
    }

    pub fn to_world_2d_by_global(&self, screen_pos: GlobalViewport) -> Option<Vec2> {
        let (window_entity, window, _) = self.windows.find_by_global_viewport(screen_pos)?;
        let viewport_pos = window_local_pos(window, screen_pos);
        self.cameras
            .to_world_2d_pos_from_viewport(window_entity, viewport_pos)
    }

    pub fn to_viewport_by_world(&self, world_pos: Vec3) -> Option<Vec2> {
        let global = self.to_global_by_world(world_pos)?;
        self.to_viewport_by_global(global)
    }

    pub fn to_viewport_by_global(&self, screen_pos: GlobalViewport) -> Option<Vec2> {
        let (_, window, _) = self.windows.find_by_global_viewport(screen_pos)?;
        Some(window_local_pos(window, screen_pos))
    }

    pub fn to_global_by_world(&self, world_pos: Vec3) -> Option<GlobalViewport> {
        let (.., layers) = self.cameras.find_by_world(world_pos)?;
        let viewport = self.cameras.to_viewport_pos(layers, world_pos)?;
        let (window_entity, ..) = self.windows.find_by_layers(layers)?;
        self.windows.to_global_viewport(window_entity, viewport)
    }
}
