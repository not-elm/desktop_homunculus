use crate::vrm::VrmBone;
use bevy::core::Name;
use bevy::ecs::system::SystemParam;
use bevy::prelude::{Children, Entity, Query};

#[derive(SystemParam)]
pub struct ChildSearcher<'w, 's> {
    entities: Query<'w, 's, (
        Option<&'static Name>,
        Option<&'static VrmBone>,
        Option<&'static Children>,
    )>,
}

impl ChildSearcher<'_, '_> {
    pub fn find_from_name(
        &self,
        root: Entity,
        target_name: &str,
    ) -> Option<Entity> {
        find_entity(target_name, false, root, &self.entities)
    }

    pub fn find_from_bone_name(
        &self,
        root: Entity,
        target_name: &VrmBone,
    ) -> Option<Entity> {
        find_entity(target_name, true, root, &self.entities)
    }
}

fn find_entity(
    target_name: &str,
    is_bone: bool,
    entity: Entity,
    entities: &Query<(
        Option<&Name>,
        Option<&VrmBone>,
        Option<&Children>,
    )>,
) -> Option<Entity> {
    let (name, bone, children) = entities.get(entity).ok()?;
    if is_bone {
        if bone.is_some_and(|bone| bone.0 == target_name) {
            return Some(entity);
        }
    } else if name.is_some_and(|name| name.as_str() == target_name) {
        return Some(entity);
    }

    for child in children? {
        if let Some(entity) = find_entity(target_name, is_bone, *child, entities) {
            return Some(entity);
        }
    }
    None
}