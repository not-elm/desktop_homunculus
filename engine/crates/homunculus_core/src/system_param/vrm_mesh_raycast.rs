use bevy::camera::NormalizedRenderTarget;
use bevy::ecs::system::SystemParam;
use bevy::pbr::MeshMaterial3d;
use bevy::picking::mesh_picking::ray_cast::RayMeshHit;
use bevy::picking::pointer::Location;
use bevy::prelude::{
    ContainsEntity, Entity, MeshRayCast, MeshRayCastSettings, Query, RayCastVisibility, default,
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

    /// Returns true only if the closest *opaque* mesh at the pointer location is a VRM mesh.
    ///
    /// The `should_skip` callback is called for each hit in depth order. If it returns
    /// `true`, that hit is skipped (treated as transparent). The first non-skipped hit
    /// is checked for MToon material.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Skip transparent webview pixels:
    /// vrm_ray_cast.is_frontmost_hit(&location, |entity, hit| {
    ///     // return true to skip this hit (e.g., transparent webview pixel)
    ///     false
    /// });
    /// ```
    pub fn is_frontmost_hit(
        &mut self,
        location: &Location,
        should_skip: impl Fn(Entity, &RayMeshHit) -> bool,
    ) -> bool {
        if let NormalizedRenderTarget::Window(window) = location.target
            && let Some((_, camera, _, tf, _)) =
                self.cameras.find_camera_from_window(window.entity())
            && let Ok(ray) = camera.viewport_to_world(tf, location.position)
        {
            let hits = self.mesh_ray_cast.cast_ray(
                ray,
                &MeshRayCastSettings {
                    visibility: RayCastVisibility::VisibleInView,
                    early_exit_test: &|_| false,
                    ..default()
                },
            );
            hits.iter()
                .find(|(entity, hit)| !should_skip(*entity, hit))
                .is_some_and(|(entity, _)| self.mtoon_materials.get(*entity).is_ok())
        } else {
            false
        }
    }
}
