use crate::character::CharacterId;
use bevy::prelude::*;
use std::collections::HashMap;

/// Bidirectional registry mapping [`CharacterId`]s to their ECS entities.
///
/// Automatically maintained by observers on [`CharacterId`] insert/remove.
#[derive(Resource, Debug, Default)]
pub struct CharacterRegistry {
    id_to_entity: HashMap<CharacterId, Entity>,
    entity_to_id: HashMap<Entity, CharacterId>,
}

impl CharacterRegistry {
    /// Look up the entity for a given character ID.
    pub fn get(&self, id: &CharacterId) -> Option<Entity> {
        self.id_to_entity.get(id).copied()
    }

    /// Look up the character ID for a given entity.
    pub fn get_id(&self, entity: Entity) -> Option<&CharacterId> {
        self.entity_to_id.get(&entity)
    }

    /// Returns `true` if the registry contains the given character ID.
    pub fn contains(&self, id: &CharacterId) -> bool {
        self.id_to_entity.contains_key(id)
    }

    /// Iterates over all registered `(CharacterId, Entity)` pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&CharacterId, &Entity)> {
        self.id_to_entity.iter()
    }

    /// Returns the number of registered characters.
    pub fn len(&self) -> usize {
        self.id_to_entity.len()
    }

    /// Returns `true` if no characters are registered.
    pub fn is_empty(&self) -> bool {
        self.id_to_entity.is_empty()
    }
}

pub(crate) struct CharacterRegistryPlugin;

impl Plugin for CharacterRegistryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CharacterRegistry>()
            .add_observer(on_character_inserted)
            .add_observer(on_character_removed);
    }
}

fn on_character_inserted(
    trigger: On<Insert, CharacterId>,
    query: Query<&CharacterId>,
    mut registry: ResMut<CharacterRegistry>,
) {
    let entity = trigger.event_target();
    let Ok(id) = query.get(entity) else {
        return;
    };
    registry.id_to_entity.insert(id.clone(), entity);
    registry.entity_to_id.insert(entity, id.clone());
}

fn on_character_removed(
    trigger: On<Remove, CharacterId>,
    query: Query<&CharacterId>,
    mut registry: ResMut<CharacterRegistry>,
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
    use crate::character::Character;

    #[test]
    fn test_registry_insert_and_lookup() {
        let mut app = App::new();
        app.add_plugins(CharacterRegistryPlugin);

        let id = CharacterId::new("test-character").unwrap();
        let entity = app.world_mut().spawn((Character, id.clone())).id();
        app.update();

        let registry = app.world().resource::<CharacterRegistry>();
        assert_eq!(registry.get(&id), Some(entity));
        assert_eq!(registry.get_id(entity), Some(&id));
        assert!(registry.contains(&id));
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn test_registry_remove_on_despawn() {
        let mut app = App::new();
        app.add_plugins(CharacterRegistryPlugin);

        let id = CharacterId::new("removable").unwrap();
        let entity = app.world_mut().spawn((Character, id.clone())).id();
        app.update();

        app.world_mut().despawn(entity);
        app.update();

        let registry = app.world().resource::<CharacterRegistry>();
        assert_eq!(registry.get(&id), None);
        assert_eq!(registry.get_id(entity), None);
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn test_registry_duplicate_id() {
        let mut app = App::new();
        app.add_plugins(CharacterRegistryPlugin);

        let id = CharacterId::new("shared-id").unwrap();
        let _entity1 = app.world_mut().spawn((Character, id.clone())).id();
        app.update();

        let entity2 = app.world_mut().spawn((Character, id.clone())).id();
        app.update();

        let registry = app.world().resource::<CharacterRegistry>();
        // Last insert wins — entity2 overwrites entity1
        assert_eq!(registry.get(&id), Some(entity2));
    }
}
