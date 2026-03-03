use bevy::camera::NormalizedRenderTarget;
use bevy::ecs::system::SystemParam;
use bevy::pbr::MeshMaterial3d;
use bevy::picking::pointer::Location;
use bevy::prelude::{
    ContainsEntity, MeshRayCast, MeshRayCastSettings, Query, RayCastVisibility, default,
};
use bevy_vrm1::prelude::{Cameras, MToonMaterial};

#[derive(SystemParam)]
pub struct VrmMeshRayCast<'w, 's> {
    mesh_ray_cast: MeshRayCast<'w, 's>,
    cameras: Cameras<'w, 's>,
    mtoon_materials: Query<'w, 's, &'static MeshMaterial3d<MToonMaterial>>,
}

impl VrmMeshRayCast<'_, '_> {
    pub fn hitting_anyone(&mut self, location: &Location) -> bool {
        if let NormalizedRenderTarget::Window(window) = location.target
            && let Some((_, camera, _, tf, _)) =
                self.cameras.find_camera_from_window(window.entity())
            && let Ok(ray) = camera.viewport_to_world(tf, location.position)
        {
            !self
                .mesh_ray_cast
                .cast_ray(
                    ray,
                    &MeshRayCastSettings {
                        visibility: RayCastVisibility::VisibleInView,
                        filter: &|e| self.mtoon_materials.get(e).is_ok(),
                        ..default()
                    },
                )
                .is_empty()
        } else {
            false
        }
    }

    /// Returns true only if the closest mesh at the pointer location is a VRM mesh.
    /// Returns false if a non-VRM mesh (e.g., webview) is closer to the camera.
    pub fn is_frontmost_hit(&mut self, location: &Location) -> bool {
        if let NormalizedRenderTarget::Window(window) = location.target
            && let Some((_, camera, _, tf, _)) =
                self.cameras.find_camera_from_window(window.entity())
            && let Ok(ray) = camera.viewport_to_world(tf, location.position)
        {
            let hits = self.mesh_ray_cast.cast_ray(
                ray,
                &MeshRayCastSettings {
                    visibility: RayCastVisibility::VisibleInView,
                    ..default()
                },
            );
            hits.first()
                .is_some_and(|(entity, _)| self.mtoon_materials.get(*entity).is_ok())
        } else {
            false
        }
    }
}
