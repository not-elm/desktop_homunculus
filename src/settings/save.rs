use crate::mascot::Mascot;
use crate::settings::preferences::action::ActionPreferences;
use crate::settings::preferences::AppPreferences;
use crate::settings::{save_file_path, MascotPreferences};
use crate::util::create_parent_dir_all_if_need;
use crate::vrm::VrmPath;
use bevy::app::{AppExit, PostUpdate};
use bevy::log::error;
use bevy::prelude::{debug, on_event, IntoSystemConfigs, Plugin, Query, Res, Transform, With};

pub struct AppSettingsSavePlugin;

impl Plugin for AppSettingsSavePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(PostUpdate, save.run_if(on_event::<AppExit>));
    }
}

fn save(
    actions: Res<ActionPreferences>,
    mascots: Query<(&Transform, &VrmPath), With<Mascot>>,
) {
    let mascots = mascots
        .iter()
        .map(|(tf, vrm_path)| {
            MascotPreferences {
                transform: *tf,
                path: vrm_path.0.clone(),
            }
        })
        .collect::<Vec<_>>();
    debug!("save preference: {:?}", mascots);

    create_parent_dir_all_if_need(&save_file_path());

    if let Err(e) = std::fs::write(save_file_path(), serde_json::to_string(&AppPreferences {
        mascots,
        actions: actions.clone(),
    }).unwrap()) {
        error!("Failed to save settings: {:?}", e);
    }
}