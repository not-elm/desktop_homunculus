mod load;
pub mod preferences;
mod save;

use crate::settings::load::AppSettingsLoadPlugin;
use crate::settings::preferences::action::ActionPreferences;
use crate::settings::preferences::{MascotLocation, MascotLocationPreferences};
use crate::settings::save::AppSettingsSavePlugin;
use crate::util::app_data_dir;
use bevy::app::App;
use bevy::prelude::Plugin;
use std::path::PathBuf;

pub struct AppSettingsPlugin;

impl Plugin for AppSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MascotLocationPreferences>()
            .add_plugins((AppSettingsLoadPlugin, AppSettingsSavePlugin))
            .init_resource::<ActionPreferences>();
    }
}

fn mascot_locations_json_path() -> PathBuf {
    app_data_dir().join("mascot_locations.json")
}

fn actions_json_path() -> PathBuf {
    app_data_dir().join("actions.json")
}
