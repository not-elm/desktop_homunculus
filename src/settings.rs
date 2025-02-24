mod load;
mod save;
pub mod preferences;
pub mod state;

use crate::settings::load::AppSettingsLoadPlugin;
use crate::settings::preferences::action::{ActionPreferences, ActionProperties};
use crate::settings::preferences::{MascotPreferences, MascotPreferencesResource};
use crate::settings::save::AppSettingsSavePlugin;
use crate::settings::state::MascotAction;
use crate::vrma::Vrma;
use bevy::app::{App, Update};
use bevy::prelude::{any_component_removed, Added, IntoSystemConfigs, Plugin, Query, ResMut, With};
use std::path::PathBuf;

pub struct AppSettingsPlugin;

impl Plugin for AppSettingsPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<MascotAction>()
            .register_type::<ActionProperties>()
            .register_type::<MascotPreferencesResource>()
            .add_plugins((
                AppSettingsLoadPlugin,
                AppSettingsSavePlugin,
            ))
            .init_resource::<ActionPreferences>()
            .add_systems(Update, (
                add_action,
                remove_action.run_if(any_component_removed::<MascotAction>)
            ));
    }
}

fn add_action(
    mut actions: ResMut<ActionPreferences>,
    vrma: Query<&MascotAction, Added<Vrma>>,
) {
    for action in vrma.iter() {
        actions.register_if_not_exists(action.clone());
    }
}

fn remove_action(
    mut actions: ResMut<ActionPreferences>,
    vrma: Query<&MascotAction, With<Vrma>>,
) {
    let exists_actions = vrma.iter().cloned().collect::<Vec<_>>();
    actions.cleanup(&exists_actions);
}

fn save_file_path() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_default()
        .join("bevy_baby")
        .join("mascots.json")
}