use bevy::prelude::*;
use homunculus_utils::prelude::*;

/// Registry of all loaded mods, built at startup.
#[derive(Resource, Debug, Default)]
pub struct ModRegistry {
    entries: Vec<ModInfo>,
}

impl ModRegistry {
    pub fn register(&mut self, info: ModInfo) {
        self.entries.push(info);
    }

    pub fn all(&self) -> &[ModInfo] {
        &self.entries
    }

    pub fn find_by_name(&self, name: &str) -> Option<&ModInfo> {
        self.entries.iter().find(|e| e.name == name)
    }
}
