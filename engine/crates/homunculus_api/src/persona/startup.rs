//! Persona startup flow.
//!
//! Runs once at application launch to migrate legacy persona data and
//! restore all persisted personas into the ECS world.

use bevy::prelude::*;
use homunculus_core::prelude::{Persona, PersonaIndex, PersonaState};
use homunculus_prefs::PrefsDatabase;

/// Startup system that loads persisted personas into the ECS world.
///
/// 1. Migrates legacy `persona::*` preference keys to the `personas` table.
/// 2. Loads all personas from the database.
/// 3. Spawns an ECS entity for each persona with [`Persona`], [`PersonaState`],
///    [`Name`], and [`Transform`] components, and updates [`PersonaIndex`].
///
/// VRM attachment is intentionally **not** performed here — mods are
/// responsible for attaching VRMs via their service scripts.
pub fn load_personas(
    mut commands: Commands,
    mut index: ResMut<PersonaIndex>,
    prefs: NonSend<PrefsDatabase>,
) {
    run_migration(&prefs);
    let personas = match prefs.list_personas() {
        Ok(p) => p,
        Err(e) => {
            error!("Failed to load personas from database: {e}");
            return;
        }
    };

    for persona in personas {
        spawn_persona(&mut commands, &mut index, persona);
    }
    info!("Loaded {} persona(s) from database", index.0.len());
}

/// Runs the one-time migration from legacy `persona::*` preference keys.
fn run_migration(prefs: &PrefsDatabase) {
    if let Err(e) = prefs.migrate_personas() {
        warn!("Persona migration failed: {e}");
    }
}

/// Spawns a single persona entity and registers it in the [`PersonaIndex`].
fn spawn_persona(commands: &mut Commands, index: &mut PersonaIndex, persona: Persona) {
    let display_name = persona
        .name
        .clone()
        .unwrap_or_else(|| persona.id.0.clone());
    let transform = extract_transform(&persona);
    let persona_id = persona.id.clone();

    let entity = commands
        .spawn((
            persona,
            PersonaState::default(),
            Name::new(display_name),
            transform,
        ))
        .id();

    index.insert(persona_id, entity);
}

/// Extracts a [`Transform`] from the persona's metadata `"transform"` key.
///
/// Returns [`Transform::default()`] if the key is absent or cannot be deserialized.
fn extract_transform(persona: &Persona) -> Transform {
    persona
        .metadata
        .get("transform")
        .and_then(|v| serde_json::from_value::<Transform>(v.clone()).ok())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use homunculus_core::prelude::PersonaId;

    #[test]
    fn extract_transform_returns_default_when_missing() {
        let persona = Persona::default();
        assert_eq!(extract_transform(&persona), Transform::default());
    }

    #[test]
    fn extract_transform_deserializes_from_metadata() {
        let expected = Transform::from_xyz(1.0, 2.0, 3.0);
        let json_value = serde_json::to_value(&expected).unwrap();
        let persona = Persona {
            metadata: [("transform".to_string(), json_value)].into_iter().collect(),
            ..default()
        };
        assert_eq!(extract_transform(&persona), expected);
    }

    #[test]
    fn load_personas_spawns_entities_from_db() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<PersonaIndex>()
            .insert_non_send_resource(PrefsDatabase::open_in_memory())
            .add_systems(Startup, load_personas);

        let prefs = app.world().non_send_resource::<PrefsDatabase>();
        let persona = Persona {
            id: PersonaId::new("test-persona"),
            name: Some("Test".to_string()),
            ..default()
        };
        prefs.save_persona(&persona).unwrap();

        app.update();

        let index = app.world().resource::<PersonaIndex>();
        assert!(index.get(&PersonaId::new("test-persona")).is_some());

        let entity = index.get(&PersonaId::new("test-persona")).unwrap();
        let world = app.world();
        let loaded = world.entity(entity).get::<Persona>().unwrap();
        assert_eq!(loaded.id, PersonaId::new("test-persona"));
        assert_eq!(loaded.name, Some("Test".to_string()));

        let state = world.entity(entity).get::<PersonaState>().unwrap();
        assert_eq!(*state, PersonaState::default());

        let name = world.entity(entity).get::<Name>().unwrap();
        assert_eq!(name.as_str(), "Test");
    }

    #[test]
    fn load_personas_uses_id_as_name_fallback() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<PersonaIndex>()
            .insert_non_send_resource(PrefsDatabase::open_in_memory())
            .add_systems(Startup, load_personas);

        let prefs = app.world().non_send_resource::<PrefsDatabase>();
        let persona = Persona {
            id: PersonaId::new("nameless"),
            name: None,
            ..default()
        };
        prefs.save_persona(&persona).unwrap();

        app.update();

        let index = app.world().resource::<PersonaIndex>();
        let entity = index.get(&PersonaId::new("nameless")).unwrap();
        let name = app.world().entity(entity).get::<Name>().unwrap();
        assert_eq!(name.as_str(), "nameless");
    }

    #[test]
    fn load_personas_restores_transform_from_metadata() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<PersonaIndex>()
            .insert_non_send_resource(PrefsDatabase::open_in_memory())
            .add_systems(Startup, load_personas);

        let expected = Transform::from_xyz(10.0, 20.0, 30.0);
        let prefs = app.world().non_send_resource::<PrefsDatabase>();
        let persona = Persona {
            id: PersonaId::new("positioned"),
            ..default()
        };
        prefs.save_persona(&persona).unwrap();
        prefs
            .save_persona_metadata(
                &PersonaId::new("positioned"),
                "transform",
                &serde_json::to_value(&expected).unwrap(),
            )
            .unwrap();

        app.update();

        let index = app.world().resource::<PersonaIndex>();
        let entity = index.get(&PersonaId::new("positioned")).unwrap();
        let transform = app.world().entity(entity).get::<Transform>().unwrap();
        assert_eq!(*transform, expected);
    }

    #[test]
    fn load_personas_migrates_legacy_keys() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<PersonaIndex>()
            .insert_non_send_resource(PrefsDatabase::open_in_memory())
            .add_systems(Startup, load_personas);

        let prefs = app.world().non_send_resource::<PrefsDatabase>();
        // Insert a legacy persona::* key
        let legacy_json = serde_json::json!({
            "displayName": "Legacy Char",
            "age": 25
        });
        prefs
            .save_as("persona::vrm:legacy", &legacy_json)
            .unwrap();

        app.update();

        // The persona should have been migrated and spawned
        let index = app.world().resource::<PersonaIndex>();
        assert!(
            !index.0.is_empty(),
            "At least one persona should be spawned after migration"
        );
    }
}
