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

pub mod characters;
mod migration;

#[cfg(feature = "bevy")]
mod vrm_transform;

#[cfg(feature = "bevy")]
use crate::vrm_transform::PrefsVrmTransformPlugin;
#[cfg(feature = "bevy")]
use bevy::prelude::*;

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
    /// Preferences key for a VRM's persona, keyed by asset ID.
    ///
    /// # Example
    ///
    /// ```
    /// use homunculus_prefs::PrefsKeys;
    /// assert_eq!(PrefsKeys::persona("vrm:elmer"), "persona::vrm:elmer");
    /// ```
    pub fn persona(asset_id: &str) -> String {
        format!("persona::{asset_id}")
    }

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
                let db = PrefsDatabase(c);
                if let Err(e) = migration::run_if_needed(&db) {
                    Self::log_error(&format!("Migration failed: {e}"));
                }
                db
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

    fn log_error(msg: &str) {
        #[cfg(feature = "bevy")]
        error!("{msg}");
        #[cfg(not(feature = "bevy"))]
        eprintln!("{msg}");
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
        "CREATE TABLE IF NOT EXISTS schema_version (version INTEGER NOT NULL)",
        [],
    )?;
    db.execute(
        "CREATE TABLE IF NOT EXISTS characters (
            id TEXT PRIMARY KEY NOT NULL
                CHECK(length(id) BETWEEN 1 AND 63),
            name TEXT NOT NULL DEFAULT '',
            persona TEXT NOT NULL DEFAULT '{}'
                CHECK(json_valid(persona)),
            transform TEXT NOT NULL DEFAULT '{}'
                CHECK(json_valid(transform)),
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        ) WITHOUT ROWID",
        [],
    )?;
    db.execute(
        "CREATE TABLE IF NOT EXISTS character_extensions (
            character_id TEXT NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
            mod_name TEXT NOT NULL,
            data TEXT NOT NULL DEFAULT '{}'
                CHECK(json_valid(data)),
            PRIMARY KEY (character_id, mod_name)
        ) WITHOUT ROWID",
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
        let obj = serde_json::json!({"profile": "cheerful", "ocean": {"openness": 0.8}});
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
}
