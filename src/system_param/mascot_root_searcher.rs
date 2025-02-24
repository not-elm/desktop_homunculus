use crate::mascot::Mascot;
use bevy::ecs::system::SystemParam;
use bevy::prelude::{Entity, Parent, Query, With};

#[derive(SystemParam)]
pub struct MascotRootSearcher<'w, 's> {
    mascots: Query<'w, 's, Entity, With<Mascot>>,
    meshes: Query<'w, 's, &'static Parent>,
}

impl MascotRootSearcher<'_, '_> {
    pub fn find_root(&self, mesh_entity: Entity) -> Option<Entity> {
        let mut parent = self.meshes.get(mesh_entity).ok()?;
        loop {
            if let Ok(mascot) = self.mascots.get(parent.get()) {
                return Some(mascot);
            }
            parent = self.meshes.get(parent.get()).ok()?;
        }
    }
}