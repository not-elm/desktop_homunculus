//! `/cameras` provides the methods for camera control and coordinate transformation.

mod global_viewport;
mod world_2d;

pub use global_viewport::global_viewport;
pub use world_2d::world_2d;

#[cfg(test)]
mod tests {
    use bevy::render::camera::{RenderTarget, Viewport};
    use bevy::render::view::RenderLayers;
    use bevy::window::WindowRef;
    use homunculus_core::prelude::AppWindow;
    use homunculus_effects::{
        App, Camera, Camera3d, Entity, IVec2, OrthographicProjection, Projection, Transform, UVec2,
        Vec3, Window, WindowPosition, default,
    };

    pub fn spawn_window_and_camera(app: &mut App) -> (Entity, Entity) {
        let window = app
            .world_mut()
            .spawn((
                AppWindow,
                Transform::default(),
                Window {
                    position: WindowPosition::At(IVec2::new(0, 0)),
                    ..default()
                },
                RenderLayers::default(),
            ))
            .id();
        let camera = app
            .world_mut()
            .spawn((
                RenderLayers::default(),
                Transform::from_translation(Vec3::new(0., 0., 3.)),
                Camera3d::default(),
                Camera {
                    target: RenderTarget::Window(WindowRef::Entity(window)),
                    viewport: Some(Viewport {
                        physical_size: UVec2::new(1280, 720),
                        ..default()
                    }),
                    ..default()
                },
                Projection::from(OrthographicProjection {
                    scaling_mode: bevy::render::camera::ScalingMode::FixedVertical {
                        viewport_height: 2.0,
                    },
                    ..OrthographicProjection::default_3d()
                }),
            ))
            .id();
        app.update();
        (window, camera)
    }
}
