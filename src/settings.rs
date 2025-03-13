mod load;
mod save;
pub mod preferences;
pub mod state;

use crate::settings::load::AppSettingsLoadPlugin;
use crate::settings::preferences::action::{ActionPreferences, ActionProperties};
use crate::settings::preferences::{MascotLocation, MascotLocationPreferences};
use crate::settings::save::AppSettingsSavePlugin;
use crate::settings::state::MascotAction;
use crate::util::app_data_dir;
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
            .register_type::<MascotLocationPreferences>()
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

fn mascot_locations_json_path() -> PathBuf {
    app_data_dir().join("mascot_locations.json")
}

fn actions_json_path() -> PathBuf {
    app_data_dir().join("actions.json")
}