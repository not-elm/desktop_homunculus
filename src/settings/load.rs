use crate::error::AppResult;
use crate::settings::preferences::{AppPreferences, MascotPreferencesResource};
use crate::settings::save_file_path;
use bevy::app::{App, Startup};
use bevy::log::error;
use bevy::prelude::{Commands, Plugin};

pub struct AppSettingsLoadPlugin;

impl Plugin for AppSettingsLoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_settings);
    }
}

fn load_settings(
    mut commands: Commands,
) {
    let preferences = read_settings().unwrap_or_else(|e| {
        error!("Failed to load settings: {:?}", e);
        AppPreferences::default()
    });
    commands.insert_resource(MascotPreferencesResource(preferences.mascots));
    commands.insert_resource(preferences.actions);
}

fn read_settings() -> AppResult<AppPreferences> {
    let path = save_file_path();
    if !path.exists() {
        return Ok(AppPreferences::default());
    }
    let json = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}