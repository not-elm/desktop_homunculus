pub mod action;

use crate::settings::preferences::action::ActionPreferences;
use bevy::prelude::{Reflect, Resource, Transform};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct AppPreferences {
    pub mascots: Vec<MascotPreferences>,
    pub actions: ActionPreferences,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default, Resource, Clone, Reflect)]
pub struct MascotPreferencesResource(pub Vec<MascotPreferences>);

impl MascotPreferencesResource {
    pub fn transform(&self, path: &Path) -> Transform {
        self.0
            .iter()
            .find_map(|p| {
                (p.path == path).then_some(p.transform)
            })
            .unwrap_or_default()
    }
}

#[derive(PartialEq, Debug, Default, Serialize, Deserialize, Clone, Reflect)]
pub struct MascotPreferences {
    pub transform: Transform,
    pub path: PathBuf,
}

