use crate::error::AppResult;
use crate::settings::preferences::action::ActionPreferences;
use crate::settings::preferences::MascotLocationPreferences;
use crate::settings::{actions_json_path, mascot_locations_json_path};
use bevy::app::{App, Startup};
use bevy::prelude::{Commands, Plugin};

pub struct AppSettingsLoadPlugin;

impl Plugin for AppSettingsLoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (load_mascot_locations, load_actions));
    }
}

fn load_mascot_locations(mut commands: Commands) {
    commands.insert_resource(read_mascot_locations().unwrap_or_default());
}

fn load_actions(mut commands: Commands) {
    commands.insert_resource(read_actions().unwrap_or_default());
}

fn read_mascot_locations() -> AppResult<MascotLocationPreferences> {
    let path = mascot_locations_json_path();
    if !path.exists() {
        return Ok(MascotLocationPreferences::default());
    }
    let json = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}

fn read_actions() -> AppResult<ActionPreferences> {
    let path = actions_json_path();
    if !path.exists() {
        return Ok(ActionPreferences::default());
    }
    let json = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}
