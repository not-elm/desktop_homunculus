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
    pub fn all_layers(&self) -> RenderLayers {
        self.cameras.iter().fold(RenderLayers::none(), |l1, (_, _, l2)| {
            l1 | l2.clone()
        })
    }

    #[inline]
    pub fn find_camera_from_window(&self, window_entity: Entity) -> Option<CameraQuery> {
        self
            .cameras
            .iter()
            .find(|(camera, _, _)| {
                matches!(camera.target, RenderTarget::Window(WindowRef::Entity(entity)) if entity == window_entity)
            })
    }

    #[inline]
    pub fn find_camera_from_world_pos(&self, world_pos: Vec3) -> Option<CameraQuery> {
        self
            .cameras
            .iter()
            .find(|(camera, gtf, _)| {
                camera
                    .logical_viewport_rect()
                    .is_some_and(|viewport| {
                        let Ok(pos) = camera.world_to_viewport(gtf, world_pos) else {
                            return false;
                        };
                        viewport.contains(pos)
                    })
            })
    }

    #[inline]
    pub fn find_camera_from_layers(&self, layers: &RenderLayers) -> Option<CameraQuery> {
        self
            .cameras
            .iter()
            .find(|(_, _, layer)| {
                layers.intersects(layer)
            })
    }

    #[inline]
    pub fn to_viewport_pos(&self, layers: &RenderLayers, world_pos: Vec3) -> Option<Vec2> {
        let (camera, camera_tf, _) = self.find_camera_from_layers(layers)?;
        camera.world_to_viewport(camera_tf, world_pos).ok()
    }

    #[inline]
    pub fn to_world_pos_from_viewport(&self, window_entity: Entity, viewport_pos: Vec2) -> Option<Vec3> {
        let (camera, camera_tf, _) = self.find_camera_from_window(window_entity)?;
        let pos = camera.viewport_to_world_2d(camera_tf, viewport_pos).unwrap();
        Some(pos.extend(0.))
    }
}

#[cfg(test)]
mod tests {
    use crate::system_param::cameras::Cameras;
    use crate::tests::{test_app, TestResult};
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::{Camera, Commands, GlobalTransform};
    use bevy::render::view::RenderLayers;

    #[test]
    fn test_all_layers() -> TestResult {
        let mut app = test_app();
        app.world_mut().run_system_once(|mut commands: Commands| {
            commands.spawn((
                Camera::default(),
                GlobalTransform::default(),
                RenderLayers::layer(1)
            ));
            commands.spawn((
                Camera::default(),
                GlobalTransform::default(),
                RenderLayers::layer(2)
            ));
        })?;
        app.update();

        let layers = app.world_mut().run_system_once(|cameras: Cameras| {
            cameras.all_layers()
        })?;
        assert_eq!(layers, RenderLayers::from_layers(&[1, 2]));
        Ok(())
    }
}