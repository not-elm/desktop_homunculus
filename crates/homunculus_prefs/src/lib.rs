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
//! - **VRM Transform Tracking**: Specialized support for persisting VRM model positions
//!
//! ## Database Location
//!
//! The preference database is stored in the application's data directory:
//! - **macOS**: `~/Library/Application Support/[app_name]/prefs.db`
//! - **Windows**: `%APPDATA%\[app_name]\prefs.db`
//! - **Linux**: `~/.local/share/[app_name]/prefs.db`
//!
//! ## Error Handling
//!
//! If the file-based database cannot be opened, the system automatically falls
//! back to an in-memory database to ensure the application continues functioning.

mod vrm_transform;

use crate::vrm_transform::PrefsVrmTransformPlugin;
use bevy::prelude::*;
use bevy::reflect::erased_serde::__private::serde::de::DeserializeOwned;
use homunculus_core::prelude::app_data_dir;
use serde::Serialize;

pub mod prelude {
    pub use crate::{HomunculusPrefsPlugin, PrefsDatabase, PrefsKeys};
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
pub struct HomunculusPrefsPlugin;

impl Plugin for HomunculusPrefsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PrefsVrmTransformPlugin)
            .insert_non_send_resource(PrefsDatabase::new("prefs"));
    }
}

pub struct PrefsKeys;

impl PrefsKeys {
    #[inline]
    pub fn vrm_transform(vrm_name: &str) -> String {
        format!("vrm::{vrm_name}::transform")
    }

    pub const SHADOW_PANEL_ALPHA: &'static str = "shadow_panel::alpha";
}

pub struct PrefsDatabase(pub rusqlite::Connection);

impl PrefsDatabase {
    pub fn new(db_name: &str) -> Self {
        rusqlite::Connection::open(app_data_dir().join(format!("{db_name}.db")))
            .and_then(|c| {
                create_tables(&c)?;
                Ok(PrefsDatabase(c))
            })
            .unwrap_or_else(|e| {
                error!("Failed to open database; use in memory database as fallback: {e}");
                PrefsDatabase::open_in_memory()
            })
    }

    pub fn open_in_memory() -> Self {
        let conn =
            rusqlite::Connection::open_in_memory().expect("Failed to open in-memory database");
        create_tables(&conn).expect("Failed to create tables");
        PrefsDatabase(conn)
    }

    pub fn load(&self, key: &str) -> Option<serde_json::Value> {
        let mut stmt = self
            .0
            .prepare("SELECT value FROM preferences WHERE key = ?")
            .ok()?;
        let mut rows = stmt.query([key]).ok()?;
        if let Some(row) = rows.next().ok()? {
            let value: String = row.get(0).ok()?;
            serde_json::from_str(&value).ok()
        } else {
            None
        }
    }

    pub fn load_as<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        self.load(key)
            .and_then(|value| serde_json::from_value(value).ok())
    }

    pub fn save<S: Serialize + ?Sized>(&self, key: &str, value: &S) -> Result<(), rusqlite::Error> {
        let value_str = serde_json::to_string(value).map_err(|_| rusqlite::Error::InvalidQuery)?;
        self.0.execute(
            "INSERT OR REPLACE INTO preferences (key, value) VALUES (?, ?)",
            [key, &value_str],
        )?;
        Ok(())
    }
}

fn create_tables(db: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
    db.execute(
        "CREATE TABLE IF NOT EXISTS preferences (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::PrefsDatabase;

    #[test]
    fn test_save_value() {
        let db = PrefsDatabase::open_in_memory();
        db.save("key", &serde_json::json!({"n": 1})).unwrap();
        let v = db.load("key").unwrap();
        assert_eq!(v, serde_json::json!({"n": 1}));
    }
}
