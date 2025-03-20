use crate::error::OutputLog;
use crate::mascot::Mascot;
use crate::settings::preferences::action::ActionPreferences;
use crate::settings::{actions_json_path, mascot_locations_json_path, MascotLocation};
use crate::system_param::monitors::Monitors;
use crate::util::create_parent_dir_all_if_need;
use bevy::app::{AppExit, PostUpdate};
use bevy::prelude::{
    debug, on_event, In, IntoSystem, IntoSystemConfigs, Plugin, Query, Res, Transform, With,
};
use bevy::render::view::RenderLayers;
use bevy::utils::HashMap;
use bevy_vrma::system_param::cameras::Cameras;
use bevy_vrma::vrm::VrmPath;
use std::path::PathBuf;

pub struct AppSettingsSavePlugin;

impl Plugin for AppSettingsSavePlugin {
    fn build(
        &self,
        app: &mut bevy::app::App,
    ) {
        app.add_systems(
            PostUpdate,
            (
                deserialize_mascot_locations.pipe(save_mascot_locations),
                save_actions,
            )
                .run_if(on_event::<AppExit>),
        );
    }
}

fn deserialize_mascot_locations(
    mascots: Query<(&Transform, &VrmPath), With<Mascot>>,
    monitors: Monitors,
    cameras: Cameras,
) -> HashMap<PathBuf, MascotLocation> {
    mascots
        .iter()
        .flat_map(|(tf, path)| {
            let (camera, gtf, layers) = cameras.find_camera_from_world_pos(tf.translation)?;
            let (_, monitor) = monitors.find_monitor(layers)?;
            Some((
                path.0.clone(),
                MascotLocation {
                    monitor_name: monitor.name.clone(),
                    scale: tf.scale,
                    rotation: tf.rotation,
                    viewport_pos: camera
                        .world_to_viewport(gtf, tf.translation)
                        .ok()?
                        .extend(tf.translation.z),
                },
            ))
        })
        .collect()
}

fn save_mascot_locations(
    In(locations): In<HashMap<PathBuf, MascotLocation>>,
    mascots: Query<(&Transform, &VrmPath, &RenderLayers), With<Mascot>>,
) {
    debug!("save mascot locations: {:?}", mascots);
    create_parent_dir_all_if_need(&mascot_locations_json_path());

    std::fs::write(
        mascot_locations_json_path(),
        serde_json::to_string(&locations).unwrap(),
    )
    .output_log_if_error("Save");
}

fn save_actions(actions: Res<ActionPreferences>) {
    let actions_json = serde_json::to_string(&*actions).unwrap();
    std::fs::write(actions_json_path(), actions_json).output_log_if_error("Save");
}
