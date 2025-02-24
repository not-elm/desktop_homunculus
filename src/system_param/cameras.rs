use crate::mascot::Mascot;
use bevy::ecs::system::SystemParam;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Camera, Entity, GlobalTransform, Query, With, Without};
use bevy::render::camera::RenderTarget;
use bevy::render::view::RenderLayers;
use bevy::window::WindowRef;

pub type CameraQuery<'w> = (&'w Camera, &'w GlobalTransform, &'w RenderLayers);

#[derive(SystemParam)]
pub struct Cameras<'w, 's> {
    pub cameras: Query<'w, 's, CameraQuery<'static>, (With<Camera>, Without<Mascot>)>,
}

impl Cameras<'_, '_> {
    #[inline]
    pub fn find_camera(&self, window_entity: Entity) -> Option<CameraQuery> {
        self
            .cameras
            .iter()
            .find(|(camera, _, _)| {
                matches!(camera.target, RenderTarget::Window(WindowRef::Entity(entity)) if entity == window_entity)
            })
    }

    #[inline]
    pub fn find_camera_from_layers(&self, layers: &RenderLayers) -> Option<CameraQuery> {
        let (camera, _, _) = self
            .cameras
            .iter()
            .find(|(_, _, layer)| {
                layer == &layers
            })?;
        if let RenderTarget::Window(WindowRef::Entity(window_entity)) = camera.target {
            self.find_camera(window_entity)
        } else {
            None
        }
    }

    #[inline]
    pub fn to_viewport_pos(&self, layers: &RenderLayers, world_pos: Vec3) -> Option<Vec2> {
        let (camera, camera_tf, _) = self.find_camera_from_layers(layers)?;
        camera.world_to_viewport(camera_tf, world_pos).ok()
    }

    #[inline]
    pub fn to_world_pos(&self, layers: &RenderLayers, viewport_pos: Vec2) -> Option<Vec3> {
        let (camera, camera_tf, _) = self.find_camera_from_layers(layers)?;
        let ray = camera.viewport_to_world(camera_tf, viewport_pos).unwrap();
        Some(ray.get_point(camera_tf.translation().distance(Vec3::ZERO)))
    }
}