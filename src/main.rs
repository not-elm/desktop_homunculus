#![allow(clippy::type_complexity, clippy::too_many_arguments)]
#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

use bevy::DefaultPlugins;
use bevy::app::{App, PluginGroup};
use bevy::asset::io::file::FileAssetReader;
use bevy::log::tracing_subscriber::Layer;
use bevy::log::{BoxedLayer, LogPlugin};
use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::render::settings::{PowerPreference, RenderCreation, WgpuSettings};
use bevy::window::{ExitCondition, WindowPlugin, WindowResolution};
use bevy_flurx::FlurxPlugin;
use bevy_vrm1::vrm::VrmPlugin;
use bevy_vrm1::vrma::VrmaPlugin;
use bevy_webview_wry::WebviewWryPlugin;
use bevy_webview_wry::api::{
    AllAppPlugins, AllClipboardPlugins, AllDialogPlugins, AllFsPlugins, AllHttpPlugins,
    AllMonitorPlugins, AllPathPlugins, AllWebWindowPlugins,
};
use bevy_webview_wry::prelude::AllLogPlugins;
use homunculus_api::HomunculusApiPlugin;
use homunculus_core::HomunculusCorePlugin;
use homunculus_core::prelude::app_data_dir;
use homunculus_drag::HomunculusDragPlugin;
use homunculus_effects::HomunculusEffectsPlugin;
use homunculus_hit_test::HomunculusHitTestPlugin;
use homunculus_http_server::HomunculusHttpServerPlugin;
use homunculus_mod::HomunculusModPlugin;
use homunculus_power_saver::HomunculusPowerSaverPlugin;
use homunculus_prefs::HomunculusPrefsPlugin;
use homunculus_screen::HomunculusScreenPlugin;
use homunculus_shadow_panel::HomunculusShadowPanelPlugin;
use homunculus_sitting::HomunculusSittingPlugin;
use homunculus_speech::HomunculusSpeechPlugin;
use homunculus_windows::HomunculusWindowsPlugin;
use std::path::PathBuf;
use std::sync::OnceLock;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling;

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::NONE))
        .add_plugins((
            DefaultPlugins
                .set(LogPlugin {
                    #[cfg(debug_assertions)]
                    level: bevy::log::Level::INFO,
                    #[cfg(not(debug_assertions))]
                    level: bevy::log::Level::ERROR,
                    custom_layer,
                    #[cfg(target_os = "windows")]
                    filter: "wgpu_hal=off".to_string(),
                    #[cfg(not(target_os = "windows"))]
                    filter: LogPlugin::default().filter,
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(0., 0.),
                        decorations: false,
                        transparent: true,
                        ..default()
                    }),
                    exit_condition: ExitCondition::DontExit,
                    ..default()
                })
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        power_preference: PowerPreference::LowPower,
                        // On Windows, neither on `VULKAN` and `DX12` don't work transparency.
                        // Ensure not certainty that this will work correctly on all devices.
                        #[cfg(target_os = "windows")]
                        backends: Some(bevy::render::settings::Backends::GL),
                        ..default()
                    }),
                    ..default()
                }),
            MeshPickingPlugin,
            #[cfg(feature = "develop")]
            bevy_egui::EguiPlugin::default(),
            #[cfg(feature = "develop")]
            bevy_inspector_egui::quick::WorldInspectorPlugin::default(),
        ))
        .add_plugins((
            FlurxPlugin,
            WebviewWryPlugin {
                local_root: PathBuf::from(""),
            },
            VrmPlugin,
            VrmaPlugin,
        ))
        .add_plugins((
            AllWebWindowPlugins,
            AllAppPlugins,
            AllClipboardPlugins,
            AllFsPlugins,
            AllDialogPlugins,
            AllLogPlugins,
            AllPathPlugins,
            AllMonitorPlugins,
            AllHttpPlugins,
        ))
        .add_plugins((
            HomunculusCorePlugin,
            HomunculusEffectsPlugin,
            HomunculusDragPlugin,
            HomunculusWindowsPlugin,
            HomunculusPowerSaverPlugin,
            HomunculusScreenPlugin,
            HomunculusSittingPlugin,
            HomunculusShadowPanelPlugin,
            HomunculusSpeechPlugin,
            HomunculusApiPlugin,
            HomunculusModPlugin,
            HomunculusHitTestPlugin,
        ));
    #[cfg(not(target_arch = "wasm32"))]
    {
        app.add_plugins((
            homunculus_deno::HomunculusDenoPlugin,
            HomunculusPrefsPlugin,
            HomunculusHttpServerPlugin,
        ));
    }
    app.add_systems(PreStartup, load_env);
    app.run();
}

fn load_env() {
    let base = FileAssetReader::get_base_path();
    match dotenv::from_path(base.join("assets").join(".env")) {
        Ok(_) => info!("[ENV]: Loaded environment variables from .env file"),
        Err(e) => warn!("[ENV]: Failed to load .env file: {e}"),
    }
}

fn custom_layer(_app: &mut App) -> Option<BoxedLayer> {
    let file_appender = rolling::daily(app_data_dir().join("Logs"), "log.txt");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    static LOG_GUARD: OnceLock<WorkerGuard> = OnceLock::new();
    let _ = LOG_GUARD.set(guard);
    Some(
        bevy::log::tracing_subscriber::fmt::layer()
            .with_writer(non_blocking)
            .with_file(true)
            .with_line_number(true)
            .boxed(),
    )
}
