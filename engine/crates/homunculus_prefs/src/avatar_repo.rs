//! Data access layer for the `avatars` and `avatar_extensions` tables.
//!
//! [`AvatarRepo`] provides CRUD operations on avatar rows and their
//! per-mod extension data.  It borrows a [`PrefsDatabase`] reference so
//! callers keep full control of the connection lifetime.

use crate::PrefsDatabase;
use serde::{Deserialize, Serialize};

/// A single row from the `avatars` table.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarRow {
    pub id: String,
    pub asset_id: String,
    pub name: String,
    /// JSON-encoded persona object.
    pub persona: String,
    /// JSON-encoded transform object.
    pub transform: String,
    pub state: String,
    pub created_at: String,
}

/// A single row from the `avatar_extensions` table.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionRow {
    pub avatar_id: String,
    pub mod_name: String,
    /// JSON-encoded extension data.
    pub data: String,
}

/// Data-access object for avatar and avatar-extension tables.
///
/// Wraps a shared reference to [`PrefsDatabase`] and exposes typed
/// query/mutation helpers.
pub struct AvatarRepo<'a>(pub(crate) &'a PrefsDatabase);

impl<'a> AvatarRepo<'a> {
    /// Inserts a new avatar row.
    pub fn create(
        &self,
        id: &str,
        asset_id: &str,
        name: &str,
        persona_json: &str,
        transform_json: &str,
    ) -> Result<(), rusqlite::Error> {
        self.0.0.execute(
            "INSERT INTO avatars (id, asset_id, name, persona, transform) \
             VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![id, asset_id, name, persona_json, transform_json],
        )?;
        Ok(())
    }

    /// Finds an avatar by its primary key.
    pub fn find_by_id(&self, id: &str) -> Result<Option<AvatarRow>, rusqlite::Error> {
        let mut stmt = self.0.0.prepare(
            "SELECT id, asset_id, name, persona, transform, state, created_at \
             FROM avatars WHERE id = ?1",
        )?;
        row_to_avatar_opt(&mut stmt, rusqlite::params![id])
    }

    /// Finds the first avatar with a given `asset_id`.
    pub fn find_by_asset_id(&self, asset_id: &str) -> Result<Option<AvatarRow>, rusqlite::Error> {
        let mut stmt = self.0.0.prepare(
            "SELECT id, asset_id, name, persona, transform, state, created_at \
             FROM avatars WHERE asset_id = ?1",
        )?;
        row_to_avatar_opt(&mut stmt, rusqlite::params![asset_id])
    }

    /// Returns every avatar row.
    pub fn list_all(&self) -> Result<Vec<AvatarRow>, rusqlite::Error> {
        let mut stmt = self.0.0.prepare(
            "SELECT id, asset_id, name, persona, transform, state, created_at \
             FROM avatars ORDER BY created_at ASC",
        )?;
        rows_to_avatars(&mut stmt, [])
    }

    /// Returns every avatar together with its extension rows.
    pub fn list_all_with_extensions(
        &self,
    ) -> Result<Vec<(AvatarRow, Vec<ExtensionRow>)>, rusqlite::Error> {
        let avatars = self.list_all()?;
        let mut result = Vec::with_capacity(avatars.len());
        for avatar in avatars {
            let extensions = self.list_extensions(&avatar.id)?;
            result.push((avatar, extensions));
        }
        Ok(result)
    }

    /// Updates the persona JSON for an avatar.
    pub fn update_persona(&self, id: &str, persona_json: &str) -> Result<(), rusqlite::Error> {
        self.0.0.execute(
            "UPDATE avatars SET persona = ?1 WHERE id = ?2",
            rusqlite::params![persona_json, id],
        )?;
        Ok(())
    }

    /// Updates the display name for an avatar.
    pub fn update_name(&self, id: &str, name: &str) -> Result<(), rusqlite::Error> {
        self.0.0.execute(
            "UPDATE avatars SET name = ?1 WHERE id = ?2",
            rusqlite::params![name, id],
        )?;
        Ok(())
    }

    /// Updates the transform JSON for an avatar.
    pub fn update_transform(&self, id: &str, transform_json: &str) -> Result<(), rusqlite::Error> {
        self.0.0.execute(
            "UPDATE avatars SET transform = ?1 WHERE id = ?2",
            rusqlite::params![transform_json, id],
        )?;
        Ok(())
    }

    /// Updates the state string (e.g. `"idle"`, `"sitting"`) for an avatar.
    pub fn update_state(&self, id: &str, state: &str) -> Result<(), rusqlite::Error> {
        self.0.0.execute(
            "UPDATE avatars SET state = ?1 WHERE id = ?2",
            rusqlite::params![state, id],
        )?;
        Ok(())
    }

    /// Returns the extension data JSON for a specific avatar and mod.
    pub fn get_extension(
        &self,
        avatar_id: &str,
        mod_name: &str,
    ) -> Result<Option<String>, rusqlite::Error> {
        let mut stmt = self
            .0
            .0
            .prepare("SELECT data FROM avatar_extensions WHERE avatar_id = ?1 AND mod_name = ?2")?;
        let mut rows = stmt.query(rusqlite::params![avatar_id, mod_name])?;
        match rows.next()? {
            Some(row) => Ok(Some(row.get(0)?)),
            None => Ok(None),
        }
    }

    /// Upserts extension data for a specific avatar and mod.
    pub fn set_extension(
        &self,
        avatar_id: &str,
        mod_name: &str,
        data_json: &str,
    ) -> Result<(), rusqlite::Error> {
        self.0.0.execute(
            "INSERT OR REPLACE INTO avatar_extensions (avatar_id, mod_name, data) \
             VALUES (?1, ?2, ?3)",
            rusqlite::params![avatar_id, mod_name, data_json],
        )?;
        Ok(())
    }

    /// Deletes extension data for a specific avatar and mod.
    pub fn delete_extension(&self, avatar_id: &str, mod_name: &str) -> Result<(), rusqlite::Error> {
        self.0.0.execute(
            "DELETE FROM avatar_extensions WHERE avatar_id = ?1 AND mod_name = ?2",
            rusqlite::params![avatar_id, mod_name],
        )?;
        Ok(())
    }

    /// Returns all extension rows for a given avatar.
    pub fn list_extensions(&self, avatar_id: &str) -> Result<Vec<ExtensionRow>, rusqlite::Error> {
        let mut stmt = self.0.0.prepare(
            "SELECT avatar_id, mod_name, data FROM avatar_extensions \
             WHERE avatar_id = ?1 ORDER BY mod_name ASC",
        )?;
        let rows = stmt
            .query_map(rusqlite::params![avatar_id], read_extension_row)?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    /// Deletes an avatar row. Extensions are cascade-deleted by the FK constraint.
    pub fn delete(&self, id: &str) -> Result<(), rusqlite::Error> {
        self.0
            .0
            .execute("DELETE FROM avatars WHERE id = ?1", rusqlite::params![id])?;
        Ok(())
    }
}

/// Maps the first result row to an [`AvatarRow`], returning `None` for empty result sets.
fn row_to_avatar_opt(
    stmt: &mut rusqlite::Statement<'_>,
    params: impl rusqlite::Params,
) -> Result<Option<AvatarRow>, rusqlite::Error> {
    let mut rows = stmt.query(params)?;
    match rows.next()? {
        Some(row) => Ok(Some(read_avatar_row(row)?)),
        None => Ok(None),
    }
}

/// Collects all result rows into a `Vec<AvatarRow>`.
fn rows_to_avatars(
    stmt: &mut rusqlite::Statement<'_>,
    params: impl rusqlite::Params,
) -> Result<Vec<AvatarRow>, rusqlite::Error> {
    let rows = stmt
        .query_map(params, read_avatar_row)?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(rows)
}

/// Reads a single `AvatarRow` from the current result row.
fn read_avatar_row(row: &rusqlite::Row<'_>) -> Result<AvatarRow, rusqlite::Error> {
    Ok(AvatarRow {
        id: row.get(0)?,
        asset_id: row.get(1)?,
        name: row.get(2)?,
        persona: row.get(3)?,
        transform: row.get(4)?,
        state: row.get(5)?,
        created_at: row.get(6)?,
    })
}

/// Reads a single `ExtensionRow` from the current result row.
fn read_extension_row(row: &rusqlite::Row<'_>) -> Result<ExtensionRow, rusqlite::Error> {
    Ok(ExtensionRow {
        avatar_id: row.get(0)?,
        mod_name: row.get(1)?,
        data: row.get(2)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_db() -> PrefsDatabase {
        PrefsDatabase::open_in_memory()
    }

    fn repo(db: &PrefsDatabase) -> AvatarRepo<'_> {
        AvatarRepo(db)
    }

    #[test]
    fn create_and_find_by_id() {
        let db = test_db();
        let r = repo(&db);
        r.create("elmer", "vrm:elmer", "Elmer", "{}", "{}").unwrap();

        let row = r.find_by_id("elmer").unwrap().unwrap();
        assert_eq!(row.id, "elmer");
        assert_eq!(row.asset_id, "vrm:elmer");
        assert_eq!(row.name, "Elmer");
        assert_eq!(row.persona, "{}");
        assert_eq!(row.transform, "{}");
        assert_eq!(row.state, "idle");
    }

    #[test]
    fn find_by_id_missing_returns_none() {
        let db = test_db();
        let r = repo(&db);
        assert!(r.find_by_id("nonexistent").unwrap().is_none());
    }

    #[test]
    fn find_by_asset_id() {
        let db = test_db();
        let r = repo(&db);
        r.create("elmer", "vrm:elmer", "Elmer", "{}", "{}").unwrap();

        let row = r.find_by_asset_id("vrm:elmer").unwrap().unwrap();
        assert_eq!(row.id, "elmer");
    }

    #[test]
    fn find_by_asset_id_missing_returns_none() {
        let db = test_db();
        let r = repo(&db);
        assert!(r.find_by_asset_id("vrm:nope").unwrap().is_none());
    }

    #[test]
    fn list_all_returns_all_in_creation_order() {
        let db = test_db();
        let r = repo(&db);
        r.create("a", "vrm:a", "A", "{}", "{}").unwrap();
        r.create("b", "vrm:b", "B", "{}", "{}").unwrap();

        let all = r.list_all().unwrap();
        assert_eq!(all.len(), 2);
        assert_eq!(all[0].id, "a");
        assert_eq!(all[1].id, "b");
    }

    #[test]
    fn list_all_empty() {
        let db = test_db();
        let r = repo(&db);
        assert!(r.list_all().unwrap().is_empty());
    }

    #[test]
    fn update_persona() {
        let db = test_db();
        let r = repo(&db);
        r.create("e", "vrm:e", "E", "{}", "{}").unwrap();

        let new_persona = r#"{"profile":"cheerful"}"#;
        r.update_persona("e", new_persona).unwrap();

        let row = r.find_by_id("e").unwrap().unwrap();
        assert_eq!(row.persona, new_persona);
    }

    #[test]
    fn update_name() {
        let db = test_db();
        let r = repo(&db);
        r.create("e", "vrm:e", "", "{}", "{}").unwrap();

        r.update_name("e", "New Name").unwrap();

        let row = r.find_by_id("e").unwrap().unwrap();
        assert_eq!(row.name, "New Name");
    }

    #[test]
    fn update_transform() {
        let db = test_db();
        let r = repo(&db);
        r.create("e", "vrm:e", "E", "{}", "{}").unwrap();

        let transform = r#"{"x":1.0,"y":2.0}"#;
        r.update_transform("e", transform).unwrap();

        let row = r.find_by_id("e").unwrap().unwrap();
        assert_eq!(row.transform, transform);
    }

    #[test]
    fn update_state() {
        let db = test_db();
        let r = repo(&db);
        r.create("e", "vrm:e", "E", "{}", "{}").unwrap();

        r.update_state("e", "sitting").unwrap();

        let row = r.find_by_id("e").unwrap().unwrap();
        assert_eq!(row.state, "sitting");
    }

    #[test]
    fn extension_set_get_list_delete() {
        let db = test_db();
        let r = repo(&db);
        r.create("e", "vrm:e", "E", "{}", "{}").unwrap();

        // Initially empty
        assert!(r.get_extension("e", "voicevox").unwrap().is_none());
        assert!(r.list_extensions("e").unwrap().is_empty());

        // Set
        r.set_extension("e", "voicevox", r#"{"speakerId":1}"#)
            .unwrap();
        let data = r.get_extension("e", "voicevox").unwrap().unwrap();
        assert_eq!(data, r#"{"speakerId":1}"#);

        // List
        r.set_extension("e", "tts", r#"{"engine":"coeiroink"}"#)
            .unwrap();
        let exts = r.list_extensions("e").unwrap();
        assert_eq!(exts.len(), 2);
        assert_eq!(exts[0].mod_name, "tts");
        assert_eq!(exts[1].mod_name, "voicevox");

        // Delete one
        r.delete_extension("e", "voicevox").unwrap();
        assert!(r.get_extension("e", "voicevox").unwrap().is_none());
        assert_eq!(r.list_extensions("e").unwrap().len(), 1);
    }

    #[test]
    fn set_extension_upserts() {
        let db = test_db();
        let r = repo(&db);
        r.create("e", "vrm:e", "E", "{}", "{}").unwrap();

        r.set_extension("e", "voicevox", r#"{"v":1}"#).unwrap();
        r.set_extension("e", "voicevox", r#"{"v":2}"#).unwrap();

        let data = r.get_extension("e", "voicevox").unwrap().unwrap();
        assert_eq!(data, r#"{"v":2}"#);
        assert_eq!(r.list_extensions("e").unwrap().len(), 1);
    }

    #[test]
    fn delete_avatar_cascades_extensions() {
        let db = test_db();
        let r = repo(&db);
        r.create("e", "vrm:e", "E", "{}", "{}").unwrap();
        r.set_extension("e", "mod_a", "{}").unwrap();
        r.set_extension("e", "mod_b", "{}").unwrap();

        r.delete("e").unwrap();

        assert!(r.find_by_id("e").unwrap().is_none());
        // Extensions should be cascade-deleted
        assert!(r.list_extensions("e").unwrap().is_empty());
    }

    #[test]
    fn delete_nonexistent_avatar_is_ok() {
        let db = test_db();
        let r = repo(&db);
        r.delete("nonexistent").unwrap();
    }

    #[test]
    fn delete_extension_nonexistent_is_ok() {
        let db = test_db();
        let r = repo(&db);
        r.delete_extension("nonexistent", "mod").unwrap();
    }

    #[test]
    fn list_all_with_extensions() {
        let db = test_db();
        let r = repo(&db);
        r.create("a", "vrm:a", "A", "{}", "{}").unwrap();
        r.create("b", "vrm:b", "B", "{}", "{}").unwrap();
        r.set_extension("a", "mod1", r#"{"x":1}"#).unwrap();
        r.set_extension("a", "mod2", r#"{"x":2}"#).unwrap();

        let result = r.list_all_with_extensions().unwrap();
        assert_eq!(result.len(), 2);

        let (avatar_a, exts_a) = &result[0];
        assert_eq!(avatar_a.id, "a");
        assert_eq!(exts_a.len(), 2);

        let (avatar_b, exts_b) = &result[1];
        assert_eq!(avatar_b.id, "b");
        assert!(exts_b.is_empty());
    }

    #[test]
    fn avatar_row_serde_roundtrip() {
        let row = AvatarRow {
            id: "elmer".to_string(),
            asset_id: "vrm:elmer".to_string(),
            name: "Elmer".to_string(),
            persona: "{}".to_string(),
            transform: "{}".to_string(),
            state: "idle".to_string(),
            created_at: "2026-01-01 00:00:00".to_string(),
        };
        let json = serde_json::to_string(&row).unwrap();
        assert!(json.contains("assetId"));
        assert!(json.contains("createdAt"));
        let deserialized: AvatarRow = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, row.id);
        assert_eq!(deserialized.asset_id, row.asset_id);
    }

    #[test]
    fn extension_row_serde_roundtrip() {
        let row = ExtensionRow {
            avatar_id: "elmer".to_string(),
            mod_name: "voicevox".to_string(),
            data: r#"{"speakerId":1}"#.to_string(),
        };
        let json = serde_json::to_string(&row).unwrap();
        assert!(json.contains("avatarId"));
        assert!(json.contains("modName"));
        let deserialized: ExtensionRow = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.avatar_id, row.avatar_id);
        assert_eq!(deserialized.mod_name, row.mod_name);
    }

    #[test]
    fn update_nonexistent_avatar_is_ok() {
        let db = test_db();
        let r = repo(&db);
        // These succeed (no rows affected) without error
        r.update_persona("ghost", "{}").unwrap();
        r.update_name("ghost", "name").unwrap();
        r.update_transform("ghost", "{}").unwrap();
        r.update_state("ghost", "idle").unwrap();
    }

    #[test]
    fn create_duplicate_id_fails() {
        let db = test_db();
        let r = repo(&db);
        r.create("dup", "vrm:dup", "Dup", "{}", "{}").unwrap();
        let result = r.create("dup", "vrm:other", "Other", "{}", "{}");
        assert!(result.is_err());
    }
}
