mod delete;
mod get;
mod list;
mod set;

use clap::{Args, Subcommand};
use homunculus_prefs::PrefsDatabase;
use homunculus_utils::error::UtilResult;

use crate::prefs::{delete::cmd_delete, get::cmd_get, list::cmd_list, set::cmd_set};

/// CLI arguments for the `hmcs prefs` subcommand.
#[derive(Args)]
pub struct PrefsArgs {
    #[command(subcommand)]
    pub command: PrefsSubcommand,
}

/// Available operations on the preferences database.
#[derive(Subcommand)]
pub enum PrefsSubcommand {
    /// List all preference keys
    List,
    /// Get a preference value by key (JSON pretty-printed)
    Get {
        /// Preference key
        key: String,
    },
    /// Set a preference value (auto-infers type: null, bool, number, string, or JSON)
    Set {
        /// Preference key
        key: String,
        /// Value (e.g. dark, 42, 0.5, true, null, '{"x":1}')
        value: String,
    },
    /// Delete a preference by key
    Delete {
        /// Preference key
        key: String,
    },
}

impl PrefsArgs {
    pub fn execute(self) -> UtilResult {
        let db = PrefsDatabase::default();
        match self.command {
            PrefsSubcommand::List => cmd_list(&db),
            PrefsSubcommand::Get { key } => cmd_get(&db, &key),
            PrefsSubcommand::Set { key, value } => cmd_set(&db, &key, &value),
            PrefsSubcommand::Delete { key } => cmd_delete(&db, &key),
        }
    }
}

#[cfg(test)]
mod tests {
    use homunculus_prefs::PrefsDatabase;

    fn make_db() -> PrefsDatabase {
        PrefsDatabase::open_in_memory()
    }

    #[test]
    fn test_list_empty() {
        let db = make_db();
        assert_eq!(db.list_keys().unwrap(), Vec::<String>::new());
    }

    #[test]
    fn test_list_with_keys() {
        let db = make_db();
        db.save_json("alpha", &serde_json::json!(1)).unwrap();
        db.save_json("beta", &serde_json::json!(2)).unwrap();
        let mut keys = db.list_keys().unwrap();
        keys.sort();
        assert_eq!(keys, vec!["alpha", "beta"]);
    }

    #[test]
    fn test_get_existing_key() {
        let db = make_db();
        db.save_json("theme", &serde_json::json!("dark")).unwrap();
        let value = db.load_json("theme").unwrap().unwrap();
        assert_eq!(value, serde_json::json!("dark"));
    }

    #[test]
    fn test_get_missing_key_returns_none() {
        let db = make_db();
        assert!(db.load_json("nonexistent").unwrap().is_none());
    }

    #[test]
    fn test_set_and_overwrite() {
        let db = make_db();
        db.save_json("vol", &serde_json::json!(0.5)).unwrap();
        db.save_json("vol", &serde_json::json!(0.8)).unwrap();
        assert_eq!(
            db.load_json("vol").unwrap().unwrap(),
            serde_json::json!(0.8)
        );
    }

    #[test]
    fn test_set_complex_json() {
        let db = make_db();
        let val = serde_json::json!({"x": 1.0, "y": 2.0});
        db.save_json("transform", &val).unwrap();
        assert_eq!(db.load_json("transform").unwrap().unwrap(), val);
    }

    #[test]
    fn test_delete_removes_key() {
        let db = make_db();
        db.save_json("key", &serde_json::json!(42)).unwrap();
        db.delete("key").unwrap();
        assert!(db.load_json("key").unwrap().is_none());
    }

    #[test]
    fn test_delete_nonexistent_is_ok() {
        let db = make_db();
        assert!(db.delete("ghost").is_ok());
    }
}
