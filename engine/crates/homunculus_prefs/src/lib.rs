//! # Homunculus Preferences
//!
//! This crate provides a persistent preferences system for the Desktop Homunculus
//! application, using SQLite as the backend storage for user settings and
//! application state.
//!
//! ## Overview
//!
//! `homunculus_prefs` implements a key-value preference storage system that allows
//! the application to persist user settings, VRM model transformations, and other
//! configuration data across application sessions.
//!
//! ## Key Features
//!
//! - **SQLite Backend**: Reliable, file-based storage with ACID guarantees
//! - **JSON Serialization**: Automatic serialization/deserialization of complex data types
//! - **Fallback Support**: In-memory database fallback if file system access fails
//! - **Type-Safe Loading**: Generic methods for loading typed preferences
//! - **VRM Transform Tracking**: Specialized support for persisting VRM model positions (bevy feature)
//!
//! ## Database Location
//!
//! The preference database is stored in the application's data directory:
//! - **All platforms**: `~/.homunculus/preferences.db`
//!
//! ## Error Handling
//!
//! If the file-based database cannot be opened, the system automatically falls
//! back to an in-memory database to ensure the application continues functioning.

#[cfg(feature = "bevy")]
mod vrm_transform;

#[cfg(feature = "bevy")]
use crate::vrm_transform::PrefsVrmTransformPlugin;
#[cfg(feature = "bevy")]
use bevy::prelude::*;
#[cfg(feature = "bevy")]
use homunculus_core::prelude::{Gender, Persona, PersonaId};

use std::collections::HashMap;

use homunculus_utils::path::homunculus_dir;
pub use rusqlite::types::Value as SqlValue;

use serde::Serialize;
use serde::de::DeserializeOwned;

pub mod prelude {
    #[cfg(feature = "bevy")]
    pub use crate::HomunculusPrefsPlugin;
    pub use crate::{PrefsDatabase, PrefsKeys};
}

/// Plugin that provides persistent preferences storage using SQLite.
///
/// This plugin sets up the preferences database and provides systems for
/// persisting user settings, VRM model transformations, and other application
/// state across sessions.
///
/// # Included Plugins
///
/// - `PrefsVrmTransformPlugin`: Handles automatic persistence of VRM model positions
#[cfg(feature = "bevy")]
pub struct HomunculusPrefsPlugin;

#[cfg(feature = "bevy")]
impl Plugin for HomunculusPrefsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PrefsVrmTransformPlugin)
            .insert_non_send_resource(PrefsDatabase::default());
    }
}

pub struct PrefsKeys;

impl PrefsKeys {
    /// Preferences key for a VRM's transform, keyed by asset ID.
    ///
    /// # Example
    ///
    /// ```
    /// use homunculus_prefs::PrefsKeys;
    /// assert_eq!(PrefsKeys::asset_transform("vrm:elmer"), "transform::vrm:elmer");
    /// ```
    pub fn asset_transform(asset_id: &str) -> String {
        format!("transform::{asset_id}")
    }

    /// Preferences key for the shadow panel's alpha (opacity) value.
    pub const SHADOW_PANEL_ALPHA: &'static str = "shadow_panel::alpha";
}

/// An imported asset record stored in the `imported_assets` table.
///
/// Represents a user-imported file (VRM model, animation, sound, etc.) that has
/// been copied into the application's managed storage. Optionally linked to a
/// persona via `persona_id` (cascade-deleted when the persona is removed).
#[derive(Debug, Clone)]
pub struct ImportedAsset {
    pub id: String,
    pub persona_id: Option<String>,
    pub path: String,
    pub asset_type: String,
    pub description: Option<String>,
    pub source_path: Option<String>,
    pub created_at: Option<String>,
}

pub struct PrefsDatabase(pub rusqlite::Connection);

impl PrefsDatabase {
    const DB_NAME: &str = "preferences";

    /// Opens (or creates) the named SQLite database file in `~/.homunculus/`.
    ///
    /// Falls back to an in-memory database if the file cannot be opened.
    pub fn new(db_name: &str) -> Self {
        match rusqlite::Connection::open(homunculus_dir().join(format!("{db_name}.db"))) {
            Ok(c) => {
                if let Err(e) = create_tables(&c) {
                    Self::log_error(&format!(
                        "Failed to create tables; use in memory database as fallback: {e}"
                    ));
                    return Self::open_in_memory();
                }
                PrefsDatabase(c)
            }
            Err(e) => {
                Self::log_error(&format!(
                    "Failed to open database; use in memory database as fallback: {e}"
                ));
                Self::open_in_memory()
            }
        }
    }

    /// Opens a temporary in-memory SQLite database.
    ///
    /// Used as a fallback when the file-based database cannot be opened,
    /// and in tests.
    ///
    /// # Panics
    ///
    /// Panics if SQLite cannot allocate an in-memory database.
    pub fn open_in_memory() -> Self {
        let conn =
            rusqlite::Connection::open_in_memory().expect("Failed to open in-memory database");
        create_tables(&conn).expect("Failed to create tables");
        PrefsDatabase(conn)
    }

    /// Saves a native SQLite value with its type discriminator.
    pub fn save(
        &self,
        key: &str,
        value: SqlValue,
        value_type: &str,
    ) -> Result<(), rusqlite::Error> {
        validate_key(key)?;
        self.0.execute(
            "INSERT OR REPLACE INTO preferences (key, value, value_type) VALUES (?1, ?2, ?3)",
            rusqlite::params![key, value, value_type],
        )?;
        Ok(())
    }

    /// Loads a native SQLite value by key. Returns `(value, value_type)`.
    pub fn load(&self, key: &str) -> Result<Option<(SqlValue, String)>, rusqlite::Error> {
        validate_key(key)?;
        let mut stmt = self
            .0
            .prepare("SELECT value, value_type FROM preferences WHERE key = ?")?;
        let mut rows = stmt.query([key])?;
        match rows.next()? {
            Some(row) => {
                let value: SqlValue = row.get(0)?;
                let value_type: String = row.get(1)?;
                Ok(Some((value, value_type)))
            }
            None => Ok(None),
        }
    }

    /// Saves a `serde_json::Value`, converting to native SQLite types.
    pub fn save_json(&self, key: &str, value: &serde_json::Value) -> Result<(), rusqlite::Error> {
        let (sql_value, value_type) = json_to_sql(value);
        self.save(key, sql_value, value_type)
    }

    /// Loads a value and converts it to `serde_json::Value`.
    pub fn load_json(&self, key: &str) -> Result<Option<serde_json::Value>, rusqlite::Error> {
        let Some((value, value_type)) = self.load(key)? else {
            return Ok(None);
        };
        Ok(sql_to_json(value, &value_type))
    }

    /// Loads a value and deserializes it into type `T`.
    pub fn load_as<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, rusqlite::Error> {
        Ok(self
            .load_json(key)?
            .and_then(|value| serde_json::from_value(value).ok()))
    }

    /// Serializes any `Serialize` type to JSON, then saves as native SQLite types.
    pub fn save_as<S: Serialize + ?Sized>(
        &self,
        key: &str,
        value: &S,
    ) -> Result<(), rusqlite::Error> {
        let json_value = serde_json::to_value(value).map_err(|_| rusqlite::Error::InvalidQuery)?;
        self.save_json(key, &json_value)
    }

    /// Returns all preference keys stored in the database.
    pub fn list_keys(&self) -> Result<Vec<String>, rusqlite::Error> {
        let mut stmt = self.0.prepare("SELECT key FROM preferences")?;
        let keys = stmt
            .query_map([], |row| row.get(0))?
            .collect::<Result<Vec<String>, _>>()?;
        Ok(keys)
    }

    /// Returns all preference entries as `(key, value)` pairs.
    pub fn list_entries(&self) -> Result<Vec<(String, serde_json::Value)>, rusqlite::Error> {
        let mut stmt = self
            .0
            .prepare("SELECT key, value, value_type FROM preferences")?;
        let entries = stmt
            .query_map([], |row| {
                let key: String = row.get(0)?;
                let value: SqlValue = row.get(1)?;
                let value_type: String = row.get(2)?;
                Ok((key, value, value_type))
            })?
            .filter_map(|r| r.ok())
            .filter_map(|(key, value, value_type)| {
                sql_to_json(value, &value_type).map(|v| (key, v))
            })
            .collect();
        Ok(entries)
    }

    /// Deletes a preference entry by key.
    ///
    /// Returns `Ok(())` even if the key did not exist.
    pub fn delete(&self, key: &str) -> Result<(), rusqlite::Error> {
        self.0
            .execute("DELETE FROM preferences WHERE key = ?", [key])?;
        Ok(())
    }

    /// Inserts or updates an imported asset record.
    ///
    /// Uses `ON CONFLICT(id) DO UPDATE` to avoid triggering `ON DELETE CASCADE`
    /// (which `INSERT OR REPLACE` would cause). Only the mutable fields (`path`,
    /// `type`, `description`, `source_path`, `created_at`) are updated on conflict;
    /// `persona_id` is set only on initial insert.
    pub fn upsert_imported_asset(
        &self,
        id: &str,
        persona_id: Option<&str>,
        path: &str,
        asset_type: &str,
        description: Option<&str>,
        source_path: Option<&str>,
    ) -> Result<(), rusqlite::Error> {
        self.0.execute(
            "INSERT INTO imported_assets (id, persona_id, path, type, description, source_path, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, datetime('now'))
             ON CONFLICT(id) DO UPDATE SET
                path = excluded.path,
                type = excluded.type,
                description = excluded.description,
                source_path = excluded.source_path,
                created_at = excluded.created_at",
            rusqlite::params![id, persona_id, path, asset_type, description, source_path],
        )?;
        Ok(())
    }

    /// Lists all imported asset records.
    pub fn list_imported_assets(&self) -> Result<Vec<ImportedAsset>, rusqlite::Error> {
        let mut stmt = self.0.prepare(
            "SELECT id, persona_id, path, type, description, source_path, created_at
             FROM imported_assets",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(ImportedAsset {
                id: row.get(0)?,
                persona_id: row.get(1)?,
                path: row.get(2)?,
                asset_type: row.get(3)?,
                description: row.get(4)?,
                source_path: row.get(5)?,
                created_at: row.get(6)?,
            })
        })?;
        rows.collect()
    }

    /// Deletes an imported asset by ID.
    ///
    /// Returns `Ok(())` even if the ID did not exist.
    pub fn delete_imported_asset(&self, id: &str) -> Result<(), rusqlite::Error> {
        self.0
            .execute("DELETE FROM imported_assets WHERE id = ?", [id])?;
        Ok(())
    }

    fn log_error(msg: &str) {
        #[cfg(feature = "bevy")]
        error!("{msg}");
        #[cfg(not(feature = "bevy"))]
        eprintln!("{msg}");
    }
}

#[cfg(feature = "bevy")]
impl PrefsDatabase {
    /// Saves a persona to the `personas` table, replacing any existing row with the same ID.
    ///
    /// Also saves all metadata entries to `persona_metadata`.
    ///
    /// **Warning**: Uses `INSERT OR REPLACE`, which in SQLite is internally `DELETE + INSERT`.
    /// This triggers `ON DELETE CASCADE` on `persona_metadata`, silently deleting all metadata
    /// rows for an existing persona before re-inserting them. Prefer [`insert_persona`] for
    /// new personas and [`update_persona`] for existing ones.
    pub fn save_persona(&self, persona: &Persona) -> Result<(), rusqlite::Error> {
        self.0.execute(
            "INSERT OR REPLACE INTO personas (id, name, age, gender, first_person_pronoun, profile, personality, vrm_asset_id)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            rusqlite::params![
                persona.id.0,
                persona.name,
                persona.age.map(|v| v as i64),
                gender_to_str(&persona.gender),
                persona.first_person_pronoun,
                persona.profile,
                persona.personality,
                persona.vrm_asset_id,
            ],
        )?;
        self.save_all_metadata(&persona.id, &persona.metadata)
    }

    /// Inserts a new persona into the `personas` table.
    ///
    /// Returns a UNIQUE constraint error if a persona with the same ID already exists.
    /// Also saves all metadata entries to `persona_metadata`.
    pub fn insert_persona(&self, persona: &Persona) -> Result<(), rusqlite::Error> {
        insert_persona_to_conn(&self.0, persona)?;
        self.save_all_metadata(&persona.id, &persona.metadata)
    }

    /// Updates an existing persona in the `personas` table.
    ///
    /// Uses `UPDATE ... WHERE id = ?` so no `DELETE + INSERT` occurs on the `personas`
    /// row itself. Replaces all metadata entries with the supplied set via
    /// `save_all_metadata` (which performs `DELETE + INSERT` on `persona_metadata`).
    ///
    /// Returns an error if no persona with the given ID exists.
    pub fn update_persona(&self, persona: &Persona) -> Result<(), rusqlite::Error> {
        update_persona_to_conn(&self.0, persona)?;
        self.save_all_metadata(&persona.id, &persona.metadata)
    }

    /// Loads a persona by ID, returning `None` if not found.
    pub fn load_persona(&self, id: &str) -> Result<Option<Persona>, rusqlite::Error> {
        let mut stmt = self.0.prepare(
            "SELECT id, name, age, gender, first_person_pronoun, profile, personality, vrm_asset_id
             FROM personas WHERE id = ?",
        )?;
        let mut rows = stmt.query([id])?;
        let Some(row) = rows.next()? else {
            return Ok(None);
        };
        let persona = row_to_persona(row)?;
        let metadata = self.load_persona_metadata(&persona.id)?;
        Ok(Some(Persona {
            metadata,
            ..persona
        }))
    }

    /// Lists all personas with their metadata.
    pub fn list_personas(&self) -> Result<Vec<Persona>, rusqlite::Error> {
        let mut stmt = self.0.prepare(
            "SELECT id, name, age, gender, first_person_pronoun, profile, personality, vrm_asset_id
             FROM personas",
        )?;
        let rows = stmt.query_map([], row_to_persona)?;
        let mut personas = Vec::new();
        for row_result in rows {
            let persona = row_result?;
            let metadata = self.load_persona_metadata(&persona.id)?;
            personas.push(Persona {
                metadata,
                ..persona
            });
        }
        Ok(personas)
    }

    /// Deletes a persona by ID. Metadata is cascade-deleted by the foreign key constraint.
    ///
    /// Returns the number of rows deleted (0 if persona did not exist).
    pub fn delete_persona(&self, id: &str) -> Result<usize, rusqlite::Error> {
        let affected = self.0.execute("DELETE FROM personas WHERE id = ?", [id])?;
        Ok(affected)
    }

    /// Saves a single metadata key-value pair for a persona.
    pub fn save_persona_metadata(
        &self,
        persona_id: &PersonaId,
        key: &str,
        value: &serde_json::Value,
    ) -> Result<(), rusqlite::Error> {
        let json_text = serde_json::to_string(value).unwrap_or_default();
        self.0.execute(
            "INSERT OR REPLACE INTO persona_metadata (persona_id, key, value) VALUES (?1, ?2, ?3)",
            rusqlite::params![persona_id.0, key, json_text],
        )?;
        Ok(())
    }

    /// Loads all metadata key-value pairs for a persona.
    pub fn load_persona_metadata(
        &self,
        persona_id: &PersonaId,
    ) -> Result<HashMap<String, serde_json::Value>, rusqlite::Error> {
        let mut stmt = self
            .0
            .prepare("SELECT key, value FROM persona_metadata WHERE persona_id = ?")?;
        let rows = stmt.query_map([&persona_id.0], |row| {
            let key: String = row.get(0)?;
            let value_text: String = row.get(1)?;
            Ok((key, value_text))
        })?;
        let mut map = HashMap::new();
        for row_result in rows {
            let (key, value_text) = row_result?;
            if let Ok(value) = serde_json::from_str(&value_text) {
                map.insert(key, value);
            }
        }
        Ok(map)
    }

    /// Replaces all metadata for a persona (deletes existing, inserts new).
    fn save_all_metadata(
        &self,
        persona_id: &PersonaId,
        metadata: &HashMap<String, serde_json::Value>,
    ) -> Result<(), rusqlite::Error> {
        save_all_metadata_to_conn(&self.0, persona_id, metadata)
    }
}

impl Default for PrefsDatabase {
    fn default() -> Self {
        Self::new(Self::DB_NAME)
    }
}

fn create_tables(db: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
    db.execute_batch("PRAGMA foreign_keys = ON")?;
    db.execute(
        "CREATE TABLE IF NOT EXISTS preferences (
            key TEXT PRIMARY KEY,
            value,
            value_type TEXT NOT NULL DEFAULT 'json'
              CHECK (value_type IN ('null', 'bool', 'number', 'string', 'json'))
        ) WITHOUT ROWID",
        [],
    )?;
    db.execute(
        "CREATE TABLE IF NOT EXISTS personas (
            id TEXT PRIMARY KEY,
            name TEXT,
            age INTEGER,
            gender TEXT NOT NULL DEFAULT 'unknown',
            first_person_pronoun TEXT,
            profile TEXT NOT NULL DEFAULT '',
            personality TEXT,
            vrm_asset_id TEXT
        )",
        [],
    )?;
    db.execute(
        "CREATE TABLE IF NOT EXISTS persona_metadata (
            persona_id TEXT NOT NULL,
            key TEXT NOT NULL,
            value TEXT NOT NULL,
            PRIMARY KEY (persona_id, key),
            FOREIGN KEY (persona_id) REFERENCES personas(id) ON DELETE CASCADE
        )",
        [],
    )?;
    db.execute(
        "CREATE TABLE IF NOT EXISTS imported_assets (
            id TEXT PRIMARY KEY,
            persona_id TEXT REFERENCES personas(id) ON DELETE CASCADE,
            path TEXT NOT NULL,
            type TEXT NOT NULL,
            description TEXT,
            source_path TEXT,
            created_at TEXT
        )",
        [],
    )?;
    Ok(())
}

/// Maps `serde_json::Value` to `(rusqlite::types::Value, type discriminator)`.
fn json_to_sql(value: &serde_json::Value) -> (SqlValue, &'static str) {
    match value {
        serde_json::Value::Null => (SqlValue::Null, "null"),
        serde_json::Value::Bool(b) => (SqlValue::Integer(i64::from(*b)), "bool"),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                (SqlValue::Integer(i), "number")
            } else if let Some(f) = n.as_f64() {
                (SqlValue::Real(f), "number")
            } else {
                (SqlValue::Text(n.to_string()), "number")
            }
        }
        serde_json::Value::String(s) => (SqlValue::Text(s.clone()), "string"),
        val @ (serde_json::Value::Array(_) | serde_json::Value::Object(_)) => (
            SqlValue::Text(serde_json::to_string(val).unwrap_or_default()),
            "json",
        ),
    }
}

/// Validates a preference key. Rejects empty keys, keys exceeding 256 characters,
/// and keys containing control characters.
fn validate_key(key: &str) -> Result<(), rusqlite::Error> {
    if key.is_empty() {
        return Err(rusqlite::Error::InvalidQuery);
    }
    if key.len() > 256 {
        return Err(rusqlite::Error::InvalidQuery);
    }
    if key.bytes().any(|b| b < 0x20) {
        return Err(rusqlite::Error::InvalidQuery);
    }
    Ok(())
}

/// Maps `(rusqlite::types::Value, type discriminator)` back to `serde_json::Value`.
fn sql_to_json(value: SqlValue, value_type: &str) -> Option<serde_json::Value> {
    match value_type {
        "null" => Some(serde_json::Value::Null),
        "bool" => match value {
            SqlValue::Integer(i) => Some(serde_json::Value::Bool(i != 0)),
            _ => None,
        },
        "number" => match value {
            SqlValue::Integer(i) => Some(serde_json::Value::Number(i.into())),
            SqlValue::Real(f) => serde_json::Number::from_f64(f).map(serde_json::Value::Number),
            SqlValue::Text(s) => s
                .parse::<i64>()
                .ok()
                .map(|n| serde_json::Value::Number(n.into()))
                .or_else(|| {
                    s.parse::<u64>()
                        .ok()
                        .map(|n| serde_json::Value::Number(n.into()))
                })
                .or_else(|| {
                    s.parse::<f64>().ok().and_then(|f| {
                        serde_json::Number::from_f64(f).map(serde_json::Value::Number)
                    })
                }),
            _ => None,
        },
        "string" => match value {
            SqlValue::Text(s) => Some(serde_json::Value::String(s)),
            _ => None,
        },
        "json" => match value {
            SqlValue::Text(s) => serde_json::from_str(&s).ok(),
            _ => None,
        },
        _ => None,
    }
}

/// Converts a `Gender` enum to a lowercase string for DB storage.
#[cfg(feature = "bevy")]
fn gender_to_str(gender: &Gender) -> &'static str {
    match gender {
        Gender::Male => "male",
        Gender::Female => "female",
        Gender::Other => "other",
        Gender::Unknown => "unknown",
    }
}

/// Parses a gender string from the DB into a `Gender` enum.
#[cfg(feature = "bevy")]
fn str_to_gender(s: &str) -> Gender {
    match s {
        "male" => Gender::Male,
        "female" => Gender::Female,
        "other" => Gender::Other,
        _ => Gender::Unknown,
    }
}

/// Converts a SQLite row (from the `personas` table) into a `Persona` (without metadata).
#[cfg(feature = "bevy")]
fn row_to_persona(row: &rusqlite::Row<'_>) -> Result<Persona, rusqlite::Error> {
    let id: String = row.get(0)?;
    let name: Option<String> = row.get(1)?;
    let age: Option<i64> = row.get(2)?;
    let gender_str: String = row.get(3)?;
    let first_person_pronoun: Option<String> = row.get(4)?;
    let profile: String = row.get(5)?;
    let personality: Option<String> = row.get(6)?;
    let vrm_asset_id: Option<String> = row.get(7)?;
    Ok(Persona {
        id: PersonaId::new(id),
        name,
        age: age.map(|v| v as u32),
        gender: str_to_gender(&gender_str),
        first_person_pronoun,
        profile,
        personality,
        vrm_asset_id,
        metadata: HashMap::new(),
    })
}

/// Inserts a new persona row using a raw connection/transaction (`INSERT INTO`).
///
/// Fails with a UNIQUE constraint error if the persona ID already exists.
#[cfg(feature = "bevy")]
fn insert_persona_to_conn(
    conn: &rusqlite::Connection,
    persona: &Persona,
) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT INTO personas (id, name, age, gender, first_person_pronoun, profile, personality, vrm_asset_id)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        rusqlite::params![
            persona.id.0,
            persona.name,
            persona.age.map(|v| v as i64),
            gender_to_str(&persona.gender),
            persona.first_person_pronoun,
            persona.profile,
            persona.personality,
            persona.vrm_asset_id,
        ],
    )?;
    Ok(())
}

/// Updates an existing persona row using a raw connection/transaction (`UPDATE`).
///
/// Returns [`rusqlite::Error::QueryReturnedNoRows`] if no persona with the given
/// ID exists.
#[cfg(feature = "bevy")]
fn update_persona_to_conn(
    conn: &rusqlite::Connection,
    persona: &Persona,
) -> Result<(), rusqlite::Error> {
    let rows_affected = conn.execute(
        "UPDATE personas SET name = ?1, age = ?2, gender = ?3, first_person_pronoun = ?4,
         profile = ?5, personality = ?6, vrm_asset_id = ?7
         WHERE id = ?8",
        rusqlite::params![
            persona.name,
            persona.age.map(|v| v as i64),
            gender_to_str(&persona.gender),
            persona.first_person_pronoun,
            persona.profile,
            persona.personality,
            persona.vrm_asset_id,
            persona.id.0,
        ],
    )?;
    if rows_affected == 0 {
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }
    Ok(())
}

/// Replaces all metadata for a persona using a raw connection/transaction.
#[cfg(feature = "bevy")]
fn save_all_metadata_to_conn(
    conn: &rusqlite::Connection,
    persona_id: &PersonaId,
    metadata: &HashMap<String, serde_json::Value>,
) -> Result<(), rusqlite::Error> {
    conn.execute(
        "DELETE FROM persona_metadata WHERE persona_id = ?",
        [&persona_id.0],
    )?;
    for (key, value) in metadata {
        let json_text = serde_json::to_string(value).unwrap_or_default();
        conn.execute(
            "INSERT INTO persona_metadata (persona_id, key, value) VALUES (?1, ?2, ?3)",
            rusqlite::params![persona_id.0, key, json_text],
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::PrefsDatabase;

    #[test]
    fn test_save_value() {
        let db = PrefsDatabase::open_in_memory();
        db.save_json("key", &serde_json::json!({"n": 1})).unwrap();
        let v = db.load_json("key").unwrap().unwrap();
        assert_eq!(v, serde_json::json!({"n": 1}));
    }

    #[test]
    fn test_delete_value() {
        let db = PrefsDatabase::open_in_memory();
        db.save_json("key", &serde_json::json!(42)).unwrap();
        assert!(db.load_json("key").unwrap().is_some());
        db.delete("key").unwrap();
        assert!(db.load_json("key").unwrap().is_none());
    }

    #[test]
    fn test_delete_nonexistent_key_is_ok() {
        let db = PrefsDatabase::open_in_memory();
        assert!(db.delete("does_not_exist").is_ok());
    }

    #[test]
    fn test_list_keys() {
        let db = PrefsDatabase::open_in_memory();
        db.save_json("a", &serde_json::json!(1)).unwrap();
        db.save_json("b", &serde_json::json!(2)).unwrap();
        let mut keys = db.list_keys().unwrap();
        keys.sort();
        assert_eq!(keys, vec!["a", "b"]);
    }

    #[test]
    fn test_list_entries() {
        let db = PrefsDatabase::open_in_memory();
        db.save_json("name", &serde_json::json!("alice")).unwrap();
        db.save_json("age", &serde_json::json!(30)).unwrap();
        db.save_json("active", &serde_json::json!(true)).unwrap();
        let mut entries = db.list_entries().unwrap();
        entries.sort_by(|a, b| a.0.cmp(&b.0));
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0], ("active".to_string(), serde_json::json!(true)));
        assert_eq!(entries[1], ("age".to_string(), serde_json::json!(30)));
        assert_eq!(entries[2], ("name".to_string(), serde_json::json!("alice")));
    }

    #[test]
    fn test_list_entries_empty() {
        let db = PrefsDatabase::open_in_memory();
        let entries = db.list_entries().unwrap();
        assert!(entries.is_empty());
    }

    #[test]
    fn test_save_load_native_integer() {
        use rusqlite::types::Value;
        let db = PrefsDatabase::open_in_memory();
        db.save("n", Value::Integer(42), "number").unwrap();
        let (val, vt) = db.load("n").unwrap().unwrap();
        assert_eq!(val, Value::Integer(42));
        assert_eq!(vt, "number");
    }

    #[test]
    fn test_save_load_native_real() {
        use rusqlite::types::Value;
        let db = PrefsDatabase::open_in_memory();
        db.save("f", Value::Real(3.14), "number").unwrap();
        let (val, vt) = db.load("f").unwrap().unwrap();
        assert_eq!(val, Value::Real(3.14));
        assert_eq!(vt, "number");
    }

    #[test]
    fn test_save_load_native_text() {
        use rusqlite::types::Value;
        let db = PrefsDatabase::open_in_memory();
        db.save("s", Value::Text("hello".to_owned()), "string")
            .unwrap();
        let (val, vt) = db.load("s").unwrap().unwrap();
        assert_eq!(val, Value::Text("hello".to_owned()));
        assert_eq!(vt, "string");
    }

    #[test]
    fn test_save_load_native_bool() {
        use rusqlite::types::Value;
        let db = PrefsDatabase::open_in_memory();
        db.save("b", Value::Integer(1), "bool").unwrap();
        let (val, vt) = db.load("b").unwrap().unwrap();
        assert_eq!(val, Value::Integer(1));
        assert_eq!(vt, "bool");
    }

    #[test]
    fn test_save_load_native_null() {
        use rusqlite::types::Value;
        let db = PrefsDatabase::open_in_memory();
        db.save("nul", Value::Null, "null").unwrap();
        let (val, vt) = db.load("nul").unwrap().unwrap();
        assert_eq!(val, Value::Null);
        assert_eq!(vt, "null");
    }

    #[test]
    fn test_save_load_native_json() {
        use rusqlite::types::Value;
        let db = PrefsDatabase::open_in_memory();
        let json_text = r#"{"x":1,"y":2}"#.to_owned();
        db.save("j", Value::Text(json_text.clone()), "json")
            .unwrap();
        let (val, vt) = db.load("j").unwrap().unwrap();
        assert_eq!(val, Value::Text(json_text));
        assert_eq!(vt, "json");
    }

    #[test]
    fn test_json_roundtrip_bool_true() {
        let db = PrefsDatabase::open_in_memory();
        db.save_json("flag", &serde_json::json!(true)).unwrap();
        assert_eq!(
            db.load_json("flag").unwrap().unwrap(),
            serde_json::json!(true)
        );
    }

    #[test]
    fn test_json_roundtrip_bool_false() {
        let db = PrefsDatabase::open_in_memory();
        db.save_json("flag", &serde_json::json!(false)).unwrap();
        assert_eq!(
            db.load_json("flag").unwrap().unwrap(),
            serde_json::json!(false)
        );
    }

    #[test]
    fn test_json_roundtrip_integer() {
        let db = PrefsDatabase::open_in_memory();
        db.save_json("n", &serde_json::json!(42)).unwrap();
        assert_eq!(db.load_json("n").unwrap().unwrap(), serde_json::json!(42));
    }

    #[test]
    fn test_json_roundtrip_float() {
        let db = PrefsDatabase::open_in_memory();
        db.save_json("f", &serde_json::json!(3.14)).unwrap();
        assert_eq!(db.load_json("f").unwrap().unwrap(), serde_json::json!(3.14));
    }

    #[test]
    fn test_json_roundtrip_string() {
        let db = PrefsDatabase::open_in_memory();
        db.save_json("s", &serde_json::json!("hello")).unwrap();
        assert_eq!(
            db.load_json("s").unwrap().unwrap(),
            serde_json::json!("hello")
        );
    }

    #[test]
    fn test_json_roundtrip_null() {
        let db = PrefsDatabase::open_in_memory();
        db.save_json("nul", &serde_json::Value::Null).unwrap();
        assert_eq!(
            db.load_json("nul").unwrap().unwrap(),
            serde_json::Value::Null
        );
    }

    #[test]
    fn test_json_roundtrip_object() {
        let db = PrefsDatabase::open_in_memory();
        let obj = serde_json::json!({"profile": "cheerful", "personality": "friendly and helpful"});
        db.save_json("persona", &obj).unwrap();
        assert_eq!(db.load_json("persona").unwrap().unwrap(), obj);
    }

    #[test]
    fn test_json_roundtrip_array() {
        let db = PrefsDatabase::open_in_memory();
        let arr = serde_json::json!([1, "two", 3.0, true, null]);
        db.save_json("arr", &arr).unwrap();
        assert_eq!(db.load_json("arr").unwrap().unwrap(), arr);
    }

    #[test]
    fn test_save_as_and_load_as() {
        let db = PrefsDatabase::open_in_memory();
        db.save_as("alpha", &0.5_f64).unwrap();
        let v: f64 = db.load_as("alpha").unwrap().unwrap();
        assert!((v - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_validate_key_empty() {
        let db = PrefsDatabase::open_in_memory();
        assert!(db.save_json("", &serde_json::json!(1)).is_err());
        assert!(db.load_json("").is_err());
    }

    #[test]
    fn test_validate_key_too_long() {
        let db = PrefsDatabase::open_in_memory();
        let long_key = "a".repeat(257);
        assert!(db.save_json(&long_key, &serde_json::json!(1)).is_err());
        assert!(db.load_json(&long_key).is_err());
    }

    #[test]
    fn test_validate_key_control_characters() {
        let db = PrefsDatabase::open_in_memory();
        assert!(db.save_json("bad\x00key", &serde_json::json!(1)).is_err());
        assert!(db.load_json("bad\nkey").is_err());
    }

    #[test]
    fn test_validate_key_valid() {
        let db = PrefsDatabase::open_in_memory();
        db.save_json("valid-key_123", &serde_json::json!(42))
            .unwrap();
        assert_eq!(
            db.load_json("valid-key_123").unwrap().unwrap(),
            serde_json::json!(42)
        );
    }

    #[test]
    fn test_save_and_load_persona() {
        use homunculus_core::prelude::{Gender, Persona, PersonaId};
        use std::collections::HashMap;

        let db = PrefsDatabase::open_in_memory();
        let mut metadata = HashMap::new();
        metadata.insert("mood".to_string(), serde_json::json!("happy"));
        let persona = Persona {
            id: PersonaId::new("elmer"),
            name: Some("Elmer".to_string()),
            age: Some(10),
            gender: Gender::Female,
            first_person_pronoun: Some("I".to_string()),
            profile: "A cheerful mascot".to_string(),
            personality: Some("friendly".to_string()),
            vrm_asset_id: Some("vrm:elmer".to_string()),
            metadata,
        };
        db.save_persona(&persona).unwrap();

        let loaded = db.load_persona("elmer").unwrap().unwrap();
        assert_eq!(loaded.id, persona.id);
        assert_eq!(loaded.name, Some("Elmer".to_string()));
        assert_eq!(loaded.age, Some(10));
        assert_eq!(loaded.gender, Gender::Female);
        assert_eq!(loaded.first_person_pronoun, Some("I".to_string()));
        assert_eq!(loaded.profile, "A cheerful mascot");
        assert_eq!(loaded.personality, Some("friendly".to_string()));
        assert_eq!(loaded.vrm_asset_id, Some("vrm:elmer".to_string()));
        assert_eq!(
            loaded.metadata.get("mood"),
            Some(&serde_json::json!("happy"))
        );
    }

    #[test]
    fn test_save_and_load_persona_minimal() {
        use homunculus_core::prelude::{Gender, Persona, PersonaId};

        let db = PrefsDatabase::open_in_memory();
        let persona = Persona {
            id: PersonaId::new("minimal"),
            gender: Gender::Unknown,
            ..Default::default()
        };
        db.save_persona(&persona).unwrap();

        let loaded = db.load_persona("minimal").unwrap().unwrap();
        assert_eq!(loaded.id.0, "minimal");
        assert_eq!(loaded.name, None);
        assert_eq!(loaded.age, None);
        assert_eq!(loaded.gender, Gender::Unknown);
        assert_eq!(loaded.profile, "");
        assert!(loaded.metadata.is_empty());
    }

    #[test]
    fn test_load_persona_not_found() {
        let db = PrefsDatabase::open_in_memory();
        assert!(db.load_persona("nonexistent").unwrap().is_none());
    }

    #[test]
    fn test_list_personas() {
        use homunculus_core::prelude::{Gender, Persona, PersonaId};

        let db = PrefsDatabase::open_in_memory();
        let p1 = Persona {
            id: PersonaId::new("alice"),
            name: Some("Alice".to_string()),
            gender: Gender::Female,
            profile: "First persona".to_string(),
            ..Default::default()
        };
        let p2 = Persona {
            id: PersonaId::new("bob"),
            name: Some("Bob".to_string()),
            gender: Gender::Male,
            profile: "Second persona".to_string(),
            ..Default::default()
        };
        db.save_persona(&p1).unwrap();
        db.save_persona(&p2).unwrap();

        let mut personas = db.list_personas().unwrap();
        personas.sort_by(|a, b| a.id.0.cmp(&b.id.0));
        assert_eq!(personas.len(), 2);
        assert_eq!(personas[0].id.0, "alice");
        assert_eq!(personas[0].name, Some("Alice".to_string()));
        assert_eq!(personas[1].id.0, "bob");
        assert_eq!(personas[1].name, Some("Bob".to_string()));
    }

    #[test]
    fn test_delete_persona_cascades_metadata() {
        use homunculus_core::prelude::{Gender, Persona, PersonaId};
        use std::collections::HashMap;

        let db = PrefsDatabase::open_in_memory();
        let mut metadata = HashMap::new();
        metadata.insert("key1".to_string(), serde_json::json!("value1"));
        metadata.insert("key2".to_string(), serde_json::json!(42));
        let persona = Persona {
            id: PersonaId::new("to-delete"),
            gender: Gender::Unknown,
            profile: String::new(),
            metadata,
            ..Default::default()
        };
        db.save_persona(&persona).unwrap();

        // Verify metadata exists
        let md = db
            .load_persona_metadata(&PersonaId::new("to-delete"))
            .unwrap();
        assert_eq!(md.len(), 2);

        // Delete persona
        db.delete_persona("to-delete").unwrap();

        // Verify persona is gone
        assert!(db.load_persona("to-delete").unwrap().is_none());

        // Verify metadata is cascade-deleted
        let md = db
            .load_persona_metadata(&PersonaId::new("to-delete"))
            .unwrap();
        assert!(md.is_empty());
    }

    #[test]
    fn test_save_persona_metadata_individual() {
        use homunculus_core::prelude::PersonaId;

        let db = PrefsDatabase::open_in_memory();
        // First create the persona (needed for FK constraint)
        let persona = homunculus_core::prelude::Persona {
            id: PersonaId::new("meta-test"),
            profile: String::new(),
            ..Default::default()
        };
        db.save_persona(&persona).unwrap();

        db.save_persona_metadata(
            &PersonaId::new("meta-test"),
            "key1",
            &serde_json::json!("val1"),
        )
        .unwrap();
        db.save_persona_metadata(&PersonaId::new("meta-test"), "key2", &serde_json::json!(42))
            .unwrap();

        let md = db
            .load_persona_metadata(&PersonaId::new("meta-test"))
            .unwrap();
        assert_eq!(md.len(), 2);
        assert_eq!(md.get("key1"), Some(&serde_json::json!("val1")));
        assert_eq!(md.get("key2"), Some(&serde_json::json!(42)));
    }

    #[test]
    fn test_save_persona_overwrites() {
        use homunculus_core::prelude::{Gender, Persona, PersonaId};

        let db = PrefsDatabase::open_in_memory();
        let persona_v1 = Persona {
            id: PersonaId::new("overwrite-test"),
            name: Some("V1".to_string()),
            gender: Gender::Male,
            profile: "Version 1".to_string(),
            ..Default::default()
        };
        db.save_persona(&persona_v1).unwrap();

        let persona_v2 = Persona {
            id: PersonaId::new("overwrite-test"),
            name: Some("V2".to_string()),
            gender: Gender::Female,
            profile: "Version 2".to_string(),
            ..Default::default()
        };
        db.save_persona(&persona_v2).unwrap();

        let loaded = db.load_persona("overwrite-test").unwrap().unwrap();
        assert_eq!(loaded.name, Some("V2".to_string()));
        assert_eq!(loaded.gender, Gender::Female);
        assert_eq!(loaded.profile, "Version 2");
    }

    #[test]
    fn test_insert_persona_duplicate_fails() {
        use homunculus_core::prelude::{Persona, PersonaId};

        let db = PrefsDatabase::open_in_memory();
        let persona = Persona {
            id: PersonaId::new("dup-test"),
            name: Some("First".to_string()),
            profile: "Original".to_string(),
            ..Default::default()
        };
        db.insert_persona(&persona).unwrap();

        // Second insert with the same ID should fail
        let persona2 = Persona {
            id: PersonaId::new("dup-test"),
            name: Some("Second".to_string()),
            profile: "Duplicate".to_string(),
            ..Default::default()
        };
        let result = db.insert_persona(&persona2);
        assert!(result.is_err(), "Expected UNIQUE constraint violation");

        // Original persona should be unchanged
        let loaded = db.load_persona("dup-test").unwrap().unwrap();
        assert_eq!(loaded.name, Some("First".to_string()));
        assert_eq!(loaded.profile, "Original");
    }

    #[test]
    fn test_update_persona_preserves_metadata() {
        use homunculus_core::prelude::{Persona, PersonaId};
        use std::collections::HashMap;

        let db = PrefsDatabase::open_in_memory();
        let mut metadata = HashMap::new();
        metadata.insert("mood".to_string(), serde_json::json!("happy"));
        metadata.insert("theme".to_string(), serde_json::json!("dark"));

        let persona = Persona {
            id: PersonaId::new("update-meta-test"),
            name: Some("Original".to_string()),
            profile: "Before update".to_string(),
            metadata: metadata.clone(),
            ..Default::default()
        };
        db.insert_persona(&persona).unwrap();

        // Verify metadata was saved
        let md = db
            .load_persona_metadata(&PersonaId::new("update-meta-test"))
            .unwrap();
        assert_eq!(md.len(), 2);

        // Update persona name, keeping same metadata
        let updated = Persona {
            id: PersonaId::new("update-meta-test"),
            name: Some("Updated".to_string()),
            profile: "After update".to_string(),
            metadata,
            ..Default::default()
        };
        db.update_persona(&updated).unwrap();

        // Verify persona fields were updated
        let loaded = db.load_persona("update-meta-test").unwrap().unwrap();
        assert_eq!(loaded.name, Some("Updated".to_string()));
        assert_eq!(loaded.profile, "After update");

        // Verify metadata still exists
        assert_eq!(loaded.metadata.len(), 2);
        assert_eq!(
            loaded.metadata.get("mood"),
            Some(&serde_json::json!("happy"))
        );
        assert_eq!(
            loaded.metadata.get("theme"),
            Some(&serde_json::json!("dark"))
        );
    }

    #[test]
    fn test_insert_persona_does_not_cascade() {
        use homunculus_core::prelude::{Persona, PersonaId};
        use std::collections::HashMap;

        let db = PrefsDatabase::open_in_memory();

        // Create persona with metadata via insert
        let mut metadata = HashMap::new();
        metadata.insert("key1".to_string(), serde_json::json!("value1"));
        let persona = Persona {
            id: PersonaId::new("cascade-test"),
            name: Some("Test".to_string()),
            profile: String::new(),
            metadata,
            ..Default::default()
        };
        db.insert_persona(&persona).unwrap();

        // Add extra metadata directly (simulating external metadata writes)
        db.save_persona_metadata(
            &PersonaId::new("cascade-test"),
            "extra",
            &serde_json::json!("extra-value"),
        )
        .unwrap();

        // Verify both metadata entries exist
        let md = db
            .load_persona_metadata(&PersonaId::new("cascade-test"))
            .unwrap();
        assert_eq!(md.len(), 2);
        assert_eq!(md.get("key1"), Some(&serde_json::json!("value1")));
        assert_eq!(md.get("extra"), Some(&serde_json::json!("extra-value")));

        // A second insert should fail (not silently replace and cascade-delete)
        let persona2 = Persona {
            id: PersonaId::new("cascade-test"),
            name: Some("Replaced".to_string()),
            profile: String::new(),
            ..Default::default()
        };
        assert!(db.insert_persona(&persona2).is_err());

        // Metadata should still be intact
        let md = db
            .load_persona_metadata(&PersonaId::new("cascade-test"))
            .unwrap();
        assert_eq!(md.len(), 2);
        assert_eq!(md.get("extra"), Some(&serde_json::json!("extra-value")));
    }

    #[test]
    fn test_update_nonexistent_persona_fails() {
        use homunculus_core::prelude::Persona;

        let db = PrefsDatabase::open_in_memory();
        let persona = Persona {
            id: homunculus_core::prelude::PersonaId::new("ghost"),
            name: Some("Ghost".to_string()),
            ..Default::default()
        };
        assert!(db.update_persona(&persona).is_err());
    }

    #[test]
    fn test_imported_asset_upsert() {
        let db = PrefsDatabase::open_in_memory();

        // Initial insert
        db.upsert_imported_asset("asset-1", None, "/old/path.vrm", "vrm", None, None)
            .unwrap();
        let assets = db.list_imported_assets().unwrap();
        assert_eq!(assets.len(), 1);
        assert_eq!(assets[0].path, "/old/path.vrm");

        // Upsert same ID with different path
        db.upsert_imported_asset(
            "asset-1",
            None,
            "/new/path.vrm",
            "vrm",
            Some("updated"),
            None,
        )
        .unwrap();
        let assets = db.list_imported_assets().unwrap();
        assert_eq!(assets.len(), 1, "upsert should not create a second row");
        assert_eq!(assets[0].path, "/new/path.vrm");
        assert_eq!(assets[0].description.as_deref(), Some("updated"));
    }

    #[test]
    fn test_imported_asset_cascade_delete() {
        use homunculus_core::prelude::{Persona, PersonaId};

        let db = PrefsDatabase::open_in_memory();

        // Create persona
        let persona = Persona {
            id: PersonaId::new("p1"),
            profile: String::new(),
            ..Default::default()
        };
        db.insert_persona(&persona).unwrap();

        // Insert imported asset linked to persona
        db.upsert_imported_asset(
            "asset-linked",
            Some("p1"),
            "/some/model.vrm",
            "vrm",
            None,
            None,
        )
        .unwrap();
        assert_eq!(db.list_imported_assets().unwrap().len(), 1);

        // Delete persona — imported asset should be cascade-deleted
        db.delete_persona("p1").unwrap();
        assert!(
            db.list_imported_assets().unwrap().is_empty(),
            "imported asset should be cascade-deleted with persona"
        );
    }

    #[test]
    fn test_imported_asset_null_persona_id() {
        use homunculus_core::prelude::{Persona, PersonaId};

        let db = PrefsDatabase::open_in_memory();

        // Create persona and an asset NOT linked to it
        let persona = Persona {
            id: PersonaId::new("p2"),
            profile: String::new(),
            ..Default::default()
        };
        db.insert_persona(&persona).unwrap();

        db.upsert_imported_asset("orphan-asset", None, "/standalone.vrma", "vrma", None, None)
            .unwrap();

        // Delete persona — orphan asset should survive
        db.delete_persona("p2").unwrap();
        let assets = db.list_imported_assets().unwrap();
        assert_eq!(assets.len(), 1);
        assert_eq!(assets[0].id, "orphan-asset");
        assert!(assets[0].persona_id.is_none());
    }
}
