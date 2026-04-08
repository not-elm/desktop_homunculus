//! Persona startup flow.
//!
//! Runs once at application launch to migrate legacy persona data.

use bevy::prelude::*;
use homunculus_prefs::PrefsDatabase;

/// Startup system that migrates legacy `persona::*` preference keys
/// to the `personas` table.
///
/// After this task, personas exist only as DB records.
/// Spawning into the ECS world is handled by mods via
/// `POST /personas/{id}/spawn`.
pub fn migrate_personas(prefs: NonSend<PrefsDatabase>) {
    if let Err(e) = prefs.migrate_personas() {
        warn!("Persona migration failed: {e}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migrate_personas_converts_legacy_keys() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_non_send_resource(PrefsDatabase::open_in_memory())
            .add_systems(Startup, migrate_personas);

        // Insert a legacy persona::* key before the first update
        {
            let prefs = app.world().non_send_resource::<PrefsDatabase>();
            let legacy_json = serde_json::json!({
                "displayName": "Legacy Char",
                "age": 25
            });
            prefs.save_as("persona::vrm:legacy", &legacy_json).unwrap();
        }

        app.update();

        // The persona should have been migrated into the personas table
        let prefs = app.world().non_send_resource::<PrefsDatabase>();
        let personas = prefs.list_personas().unwrap();
        assert!(
            !personas.is_empty(),
            "At least one persona should exist in DB after migration"
        );
    }

    #[test]
    fn migrate_personas_succeeds_with_no_legacy_data() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_non_send_resource(PrefsDatabase::open_in_memory())
            .add_systems(Startup, migrate_personas);

        // Should not panic or error even with no legacy keys
        app.update();

        let prefs = app.world().non_send_resource::<PrefsDatabase>();
        let result = prefs.list_personas().unwrap();
        assert!(result.is_empty());
    }
}
