use crate::avatar::AvatarId;
use bevy::prelude::*;
use std::collections::HashMap;

/// Bidirectional registry mapping [`AvatarId`]s to their ECS entities.
///
/// Automatically maintained by observers on [`AvatarId`] insert/remove.
#[derive(Resource, Debug, Default)]
pub struct AvatarRegistry {
    id_to_entity: HashMap<AvatarId, Entity>,
    entity_to_id: HashMap<Entity, AvatarId>,
}

impl AvatarRegistry {
    /// Look up the entity for a given avatar ID.
    pub fn get(&self, id: &AvatarId) -> Option<Entity> {
        self.id_to_entity.get(id).copied()
    }

    /// Look up the avatar ID for a given entity.
    pub fn get_id(&self, entity: Entity) -> Option<&AvatarId> {
        self.entity_to_id.get(&entity)
    }

    /// Returns `true` if the registry contains the given avatar ID.
    pub fn contains(&self, id: &AvatarId) -> bool {
        self.id_to_entity.contains_key(id)
    }

    /// Iterates over all registered `(AvatarId, Entity)` pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&AvatarId, &Entity)> {
        self.id_to_entity.iter()
    }

    /// Returns the number of registered avatars.
    pub fn len(&self) -> usize {
        self.id_to_entity.len()
    }

    /// Returns `true` if no avatars are registered.
    pub fn is_empty(&self) -> bool {
        self.id_to_entity.is_empty()
    }
}

pub(crate) struct AvatarRegistryPlugin;

impl Plugin for AvatarRegistryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AvatarRegistry>()
            .add_observer(on_avatar_inserted)
            .add_observer(on_avatar_removed);
    }
}

fn on_avatar_inserted(
    trigger: On<Insert, AvatarId>,
    query: Query<&AvatarId>,
    mut registry: ResMut<AvatarRegistry>,
) {
    let entity = trigger.event_target();
    let Ok(id) = query.get(entity) else {
        return;
    };
    registry.id_to_entity.insert(id.clone(), entity);
    registry.entity_to_id.insert(entity, id.clone());
}

fn on_avatar_removed(
    trigger: On<Remove, AvatarId>,
    query: Query<&AvatarId>,
    mut registry: ResMut<AvatarRegistry>,
) {
    let entity = trigger.event_target();
    let Ok(id) = query.get(entity) else {
        return;
    };
    registry.id_to_entity.remove(id);
    registry.entity_to_id.remove(&entity);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::avatar::Avatar;

    #[test]
    fn test_registry_insert_and_lookup() {
        let mut app = App::new();
        app.add_plugins(AvatarRegistryPlugin);

        let id = AvatarId::new("test-avatar").unwrap();
        let entity = app.world_mut().spawn((Avatar, id.clone())).id();
        app.update();

        let registry = app.world().resource::<AvatarRegistry>();
        assert_eq!(registry.get(&id), Some(entity));
        assert_eq!(registry.get_id(entity), Some(&id));
        assert!(registry.contains(&id));
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn test_registry_remove_on_despawn() {
        let mut app = App::new();
        app.add_plugins(AvatarRegistryPlugin);

        let id = AvatarId::new("removable").unwrap();
        let entity = app.world_mut().spawn((Avatar, id.clone())).id();
        app.update();

        app.world_mut().despawn(entity);
        app.update();

        let registry = app.world().resource::<AvatarRegistry>();
        assert_eq!(registry.get(&id), None);
        assert_eq!(registry.get_id(entity), None);
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn test_registry_duplicate_id() {
        let mut app = App::new();
        app.add_plugins(AvatarRegistryPlugin);

        let id = AvatarId::new("shared-id").unwrap();
        let _entity1 = app.world_mut().spawn((Avatar, id.clone())).id();
        app.update();

        let entity2 = app.world_mut().spawn((Avatar, id.clone())).id();
        app.update();

        let registry = app.world().resource::<AvatarRegistry>();
        // Last insert wins — entity2 overwrites entity1
        assert_eq!(registry.get(&id), Some(entity2));
    }
}
