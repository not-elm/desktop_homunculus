//! Schema migration for the character tables.
//!
//! Converts legacy preference keys (`persona::`, `transform::`, `name::`)
//! into structured rows in the `characters` table.  The migration is
//! idempotent — it only runs when `schema_version` is absent or below the
//! target version.

use crate::PrefsDatabase;

/// Current schema version. Increment when adding new migration phases.
const TARGET_VERSION: i64 = 1;

/// Runs pending migrations if the database is behind [`TARGET_VERSION`].
///
/// Called from [`PrefsDatabase::new()`] after tables are created.
/// In-memory databases skip migration because they never contain legacy data.
pub(crate) fn run_if_needed(db: &PrefsDatabase) -> Result<(), rusqlite::Error> {
    let current = current_version(db)?;
    if current >= TARGET_VERSION {
        return Ok(());
    }
    db.0.execute_batch("BEGIN")?;
    let result = migrate_and_set_version(db);
    match result {
        Ok(()) => db.0.execute_batch("COMMIT").map(|_| ()),
        Err(e) => {
            let _ = db.0.execute_batch("ROLLBACK");
            Err(e)
        }
    }
}

fn migrate_and_set_version(db: &PrefsDatabase) -> Result<(), rusqlite::Error> {
    migrate_v0_to_v1(db)?;
    set_version(db, TARGET_VERSION)
}

/// Reads the current schema version (0 if no rows exist).
fn current_version(db: &PrefsDatabase) -> Result<i64, rusqlite::Error> {
    let mut stmt = db.0.prepare("SELECT version FROM schema_version LIMIT 1")?;
    let mut rows = stmt.query([])?;
    match rows.next()? {
        Some(row) => row.get(0),
        None => Ok(0),
    }
}

/// Persists the schema version number.
fn set_version(db: &PrefsDatabase, version: i64) -> Result<(), rusqlite::Error> {
    db.0.execute("DELETE FROM schema_version", [])?;
    db.0.execute(
        "INSERT INTO schema_version (version) VALUES (?1)",
        rusqlite::params![version],
    )?;
    Ok(())
}

/// V0 → V1: populate `characters` from legacy preference keys.
fn migrate_v0_to_v1(db: &PrefsDatabase) -> Result<(), rusqlite::Error> {
    phase_a_personas(db)?;
    phase_b_transforms(db)?;
    phase_c_names(db)?;
    phase_d_extensions(db)?;
    Ok(())
}

/// Phase A — Create character rows from `persona::{asset_id}` keys.
fn phase_a_personas(db: &PrefsDatabase) -> Result<(), rusqlite::Error> {
    let entries = list_entries_with_prefix(db, "persona::")?;
    for (key, value) in entries {
        let asset_id = &key["persona::".len()..];
        let Some(character_id) = strip_vrm_prefix(asset_id) else {
            continue;
        };
        db.0.execute(
            "INSERT OR IGNORE INTO characters (id, persona) VALUES (?1, ?2)",
            rusqlite::params![character_id, value],
        )?;
    }
    Ok(())
}

/// Phase B — Update transforms from `transform::{asset_id}` keys.
fn phase_b_transforms(db: &PrefsDatabase) -> Result<(), rusqlite::Error> {
    let entries = list_entries_with_prefix(db, "transform::")?;
    for (key, value) in entries {
        let asset_id = &key["transform::".len()..];
        let Some(character_id) = strip_vrm_prefix(asset_id) else {
            continue;
        };
        db.0.execute(
            "UPDATE characters SET transform = ?1 WHERE id = ?2",
            rusqlite::params![value, character_id],
        )?;
    }
    Ok(())
}

/// Phase C — Update names from `name::{asset_id}::{lang}` keys.
///
/// For each character, prefers the `en` name; falls back to the first
/// alphabetically sorted key.
fn phase_c_names(db: &PrefsDatabase) -> Result<(), rusqlite::Error> {
    let entries = list_entries_with_prefix(db, "name::")?;
    let grouped = group_name_entries(&entries);
    for (character_id, best_name) in grouped {
        db.0.execute(
            "UPDATE characters SET name = ?1 WHERE id = ?2",
            rusqlite::params![best_name, character_id],
        )?;
    }
    Ok(())
}

/// Phase D — Migrate mod extension keys.
///
/// Currently a no-op placeholder.  When mods start persisting per-character
/// extension data under a well-known key prefix, this phase will move those
/// values into the `character_extensions` table.
fn phase_d_extensions(_db: &PrefsDatabase) -> Result<(), rusqlite::Error> {
    Ok(())
}

/// Strips the `"vrm:"` prefix from an asset ID, returning the character ID.
fn strip_vrm_prefix(asset_id: &str) -> Option<&str> {
    asset_id.strip_prefix("vrm:")
}

/// Returns all preference entries whose key starts with `prefix`, sorted by key.
fn list_entries_with_prefix(
    db: &PrefsDatabase,
    prefix: &str,
) -> Result<Vec<(String, String)>, rusqlite::Error> {
    let mut stmt = db
        .0
        .prepare("SELECT key, value FROM preferences WHERE key LIKE ?1 || '%' ORDER BY key ASC")?;
    let rows = stmt
        .query_map(rusqlite::params![prefix], |row| {
            let key: String = row.get(0)?;
            let value: String = row.get(1)?;
            Ok((key, value))
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(rows)
}

/// Groups `name::{asset_id}::{lang}` entries by character ID, picking the best name.
///
/// Prefers the `en` variant; otherwise falls back to the first key alphabetically
/// (which is already guaranteed because entries arrive sorted by key ASC).
fn group_name_entries(entries: &[(String, String)]) -> Vec<(&str, &str)> {
    let mut result: Vec<(&str, &str)> = Vec::new();
    for (key, value) in entries {
        let Some(rest) = key.strip_prefix("name::") else {
            continue;
        };
        let Some(sep_pos) = rest.rfind("::") else {
            continue;
        };
        let asset_id = &rest[..sep_pos];
        let lang = &rest[sep_pos + 2..];
        let Some(character_id) = strip_vrm_prefix(asset_id) else {
            continue;
        };

        if let Some(entry) = result.iter_mut().find(|(id, _)| *id == character_id) {
            // Replace only if this is the `en` variant
            if lang == "en" {
                entry.1 = value.as_str();
            }
        } else {
            // First entry for this character — use it as the default
            result.push((character_id, value.as_str()));
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_db() -> PrefsDatabase {
        PrefsDatabase::open_in_memory()
    }

    #[test]
    fn current_version_empty_is_zero() {
        let db = test_db();
        assert_eq!(current_version(&db).unwrap(), 0);
    }

    #[test]
    fn set_and_read_version() {
        let db = test_db();
        set_version(&db, 1).unwrap();
        assert_eq!(current_version(&db).unwrap(), 1);
    }

    #[test]
    fn set_version_replaces_existing() {
        let db = test_db();
        set_version(&db, 1).unwrap();
        set_version(&db, 2).unwrap();
        assert_eq!(current_version(&db).unwrap(), 2);
    }

    #[test]
    fn run_if_needed_skips_when_at_target() {
        let db = test_db();
        set_version(&db, TARGET_VERSION).unwrap();
        run_if_needed(&db).unwrap();
        assert_eq!(current_version(&db).unwrap(), TARGET_VERSION);
    }

    #[test]
    fn phase_a_creates_characters_from_persona_keys() {
        let db = test_db();
        let persona = r#"{"profile":"cheerful"}"#;
        db.save(
            "persona::vrm:elmer",
            rusqlite::types::Value::Text(persona.to_string()),
            "json",
        )
        .unwrap();

        phase_a_personas(&db).unwrap();

        let repo = crate::characters::CharactersTable(&db);
        let row = repo.find_by_id("elmer").unwrap().unwrap();
        assert_eq!(row.persona, persona);
    }

    #[test]
    fn phase_a_skips_non_vrm_asset_ids() {
        let db = test_db();
        db.save(
            "persona::sound:bell",
            rusqlite::types::Value::Text("{}".to_string()),
            "json",
        )
        .unwrap();

        phase_a_personas(&db).unwrap();

        let repo = crate::characters::CharactersTable(&db);
        assert!(repo.list_all().unwrap().is_empty());
    }

    #[test]
    fn phase_b_updates_transforms() {
        let db = test_db();
        // Phase A first to create the character
        db.save(
            "persona::vrm:elmer",
            rusqlite::types::Value::Text("{}".to_string()),
            "json",
        )
        .unwrap();
        phase_a_personas(&db).unwrap();

        let transform = r#"{"x":1.0,"y":2.0,"z":3.0}"#;
        db.save(
            "transform::vrm:elmer",
            rusqlite::types::Value::Text(transform.to_string()),
            "json",
        )
        .unwrap();
        phase_b_transforms(&db).unwrap();

        let repo = crate::characters::CharactersTable(&db);
        let row = repo.find_by_id("elmer").unwrap().unwrap();
        assert_eq!(row.transform, transform);
    }

    #[test]
    fn phase_c_prefers_en_name() {
        let db = test_db();
        db.save(
            "persona::vrm:elmer",
            rusqlite::types::Value::Text("{}".to_string()),
            "json",
        )
        .unwrap();
        phase_a_personas(&db).unwrap();

        db.save(
            "name::vrm:elmer::ja",
            rusqlite::types::Value::Text("エルマー".to_string()),
            "string",
        )
        .unwrap();
        db.save(
            "name::vrm:elmer::en",
            rusqlite::types::Value::Text("Elmer".to_string()),
            "string",
        )
        .unwrap();
        phase_c_names(&db).unwrap();

        let repo = crate::characters::CharactersTable(&db);
        let row = repo.find_by_id("elmer").unwrap().unwrap();
        assert_eq!(row.name, "Elmer");
    }

    #[test]
    fn phase_c_falls_back_to_first_alphabetically() {
        let db = test_db();
        db.save(
            "persona::vrm:elmer",
            rusqlite::types::Value::Text("{}".to_string()),
            "json",
        )
        .unwrap();
        phase_a_personas(&db).unwrap();

        db.save(
            "name::vrm:elmer::ja",
            rusqlite::types::Value::Text("エルマー".to_string()),
            "string",
        )
        .unwrap();
        db.save(
            "name::vrm:elmer::zh",
            rusqlite::types::Value::Text("埃尔默".to_string()),
            "string",
        )
        .unwrap();
        phase_c_names(&db).unwrap();

        let repo = crate::characters::CharactersTable(&db);
        let row = repo.find_by_id("elmer").unwrap().unwrap();
        // `ja` < `zh` alphabetically, so `ja` is first
        assert_eq!(row.name, "エルマー");
    }

    #[test]
    fn full_migration_end_to_end() {
        let db = test_db();

        // Seed legacy prefs
        db.save(
            "persona::vrm:elmer",
            rusqlite::types::Value::Text(r#"{"profile":"cheerful"}"#.to_string()),
            "json",
        )
        .unwrap();
        db.save(
            "transform::vrm:elmer",
            rusqlite::types::Value::Text(r#"{"x":1}"#.to_string()),
            "json",
        )
        .unwrap();
        db.save(
            "name::vrm:elmer::en",
            rusqlite::types::Value::Text("Elmer".to_string()),
            "string",
        )
        .unwrap();
        db.save(
            "persona::vrm:maid",
            rusqlite::types::Value::Text(r#"{"profile":"polite"}"#.to_string()),
            "json",
        )
        .unwrap();

        run_if_needed(&db).unwrap();

        let repo = crate::characters::CharactersTable(&db);
        let all = repo.list_all().unwrap();
        assert_eq!(all.len(), 2);

        let elmer = repo.find_by_id("elmer").unwrap().unwrap();
        assert_eq!(elmer.persona, r#"{"profile":"cheerful"}"#);
        assert_eq!(elmer.transform, r#"{"x":1}"#);
        assert_eq!(elmer.name, "Elmer");

        let maid = repo.find_by_id("maid").unwrap().unwrap();
        assert_eq!(maid.persona, r#"{"profile":"polite"}"#);
        assert_eq!(maid.transform, "{}"); // default, no transform key
        assert_eq!(maid.name, ""); // default, no name key

        assert_eq!(current_version(&db).unwrap(), 1);
    }

    #[test]
    fn migration_is_idempotent() {
        let db = test_db();
        db.save(
            "persona::vrm:elmer",
            rusqlite::types::Value::Text("{}".to_string()),
            "json",
        )
        .unwrap();

        run_if_needed(&db).unwrap();
        run_if_needed(&db).unwrap(); // second run should be a no-op

        let repo = crate::characters::CharactersTable(&db);
        assert_eq!(repo.list_all().unwrap().len(), 1);
        assert_eq!(current_version(&db).unwrap(), 1);
    }

    #[test]
    fn strip_vrm_prefix_works() {
        assert_eq!(strip_vrm_prefix("vrm:elmer"), Some("elmer"));
        assert_eq!(strip_vrm_prefix("vrm:a/b"), Some("a/b"));
        assert_eq!(strip_vrm_prefix("sound:bell"), None);
        assert_eq!(strip_vrm_prefix(""), None);
    }

    #[test]
    fn group_name_entries_en_preferred() {
        let entries = vec![
            ("name::vrm:e::de".to_string(), "Deutsch".to_string()),
            ("name::vrm:e::en".to_string(), "English".to_string()),
            ("name::vrm:e::ja".to_string(), "日本語".to_string()),
        ];
        let grouped = group_name_entries(&entries);
        assert_eq!(grouped.len(), 1);
        assert_eq!(grouped[0], ("e", "English"));
    }

    #[test]
    fn group_name_entries_fallback_first_alpha() {
        let entries = vec![
            ("name::vrm:e::ja".to_string(), "日本語".to_string()),
            ("name::vrm:e::zh".to_string(), "中文".to_string()),
        ];
        let grouped = group_name_entries(&entries);
        assert_eq!(grouped.len(), 1);
        // `ja` comes first alphabetically
        assert_eq!(grouped[0], ("e", "日本語"));
    }

    #[test]
    fn group_name_entries_multiple_characters() {
        let entries = vec![
            ("name::vrm:a::en".to_string(), "Alice".to_string()),
            ("name::vrm:b::ja".to_string(), "ボブ".to_string()),
        ];
        let grouped = group_name_entries(&entries);
        assert_eq!(grouped.len(), 2);
        assert_eq!(grouped[0], ("a", "Alice"));
        assert_eq!(grouped[1], ("b", "ボブ"));
    }

    #[test]
    fn group_name_entries_skips_non_vrm() {
        let entries = vec![("name::sound:bell::en".to_string(), "Bell".to_string())];
        let grouped = group_name_entries(&entries);
        assert!(grouped.is_empty());
    }

    #[test]
    fn group_name_entries_skips_malformed_keys() {
        let entries = vec![("name::vrm:e".to_string(), "no lang".to_string())];
        let grouped = group_name_entries(&entries);
        assert!(grouped.is_empty());
    }
}
