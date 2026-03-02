use bevy::prelude::*;
use std::collections::HashMap;

// Re-export types that moved to homunculus_utils for backward compatibility.
#[allow(unused_imports)]
pub use homunculus_utils::schema::asset::{AssetDeclaration, AssetEntry, AssetId, AssetType};

/// Component to track the asset ID loaded on an entity.
#[derive(Component, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssetIdComponent(pub AssetId);

/// Registry of all declared assets, built at startup from MOD package.json files.
///
/// Maps asset ID to AssetEntry. Asset IDs are globally unique (MOD developers' responsibility).
#[derive(Resource, Debug, Default)]
pub struct AssetRegistry {
    entries: HashMap<AssetId, AssetEntry>,
}

impl AssetRegistry {
    /// Register an asset entry. Logs a warning and skips if the ID already exists.
    pub fn register(&mut self, entry: AssetEntry) {
        if let Some(existing) = self.entries.get(&entry.id) {
            warn!(
                "Duplicate asset ID '{}': already registered by mod '{}', skipping from mod '{}'",
                entry.id, existing.mod_name, entry.mod_name
            );
            return;
        }
        self.entries.insert(entry.id.clone(), entry);
    }

    /// Look up an asset entry by ID.
    pub fn get(&self, id: &str) -> Option<&AssetEntry> {
        self.entries.get(id)
    }

    /// Returns all registered assets.
    pub fn all(&self) -> impl Iterator<Item = &AssetEntry> {
        self.entries.values()
    }

    /// Returns all registered assets filtered by type.
    pub fn by_type(&self, asset_type: &AssetType) -> impl Iterator<Item = &AssetEntry> {
        self.entries
            .values()
            .filter(move |e| &e.asset_type == asset_type)
    }

    /// Returns all registered assets filtered by mod name.
    pub fn by_mod(&self, mod_name: &str) -> impl Iterator<Item = &AssetEntry> {
        self.entries
            .values()
            .filter(move |e| e.mod_name == mod_name)
    }
}
