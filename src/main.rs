#![allow(clippy::type_complexity, clippy::too_many_arguments)]
#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]
mod application_windows;
mod error;
mod file_watcher;
mod global_window;
mod macros;
mod mascot;
mod menu;
mod power_state;
mod settings;
mod system_param;
mod util;

use crate::application_windows::ApplicationWindowsPlugin;
use crate::file_watcher::FileWatcherPlugin;
use crate::mascot::DesktopMascotPlugin;
use crate::menu::MenuPlugin;
use crate::power_state::PowerStatePlugin;
use crate::settings::AppSettingsPlugin;
use crate::util::app_data_dir;
use bevy::app::{App, PluginGroup};
use bevy::color::Color;
use bevy::log::tracing_subscriber::Layer;
use bevy::log::{BoxedLayer, LogPlugin};
use bevy::prelude::{default, AmbientLight, ClearColor, MeshPickingPlugin, Window};
use bevy::render::settings::{RenderCreation, WgpuSettings};
use bevy::render::RenderPlugin;
use bevy::window::{ExitCondition, WindowPlugin, WindowResolution};
use bevy::DefaultPlugins;
use bevy_vrma::vrm::VrmPlugin;
use bevy_vrma::vrma::VrmaPlugin;
use bevy_webview_wry::api::{AllLogPlugins, AppExitApiPlugin};
use bevy_webview_wry::prelude::AllDialogPlugins;
use bevy_webview_wry::WebviewWryPlugin;
use std::sync::OnceLock;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins
            .set(LogPlugin {
                #[cfg(debug_assertions)]
                level: bevy::log::Level::DEBUG,
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
                    // On Windows, neither on `VULKAN` and `DX12` don't work transparency.
                    // Ensure not certainty that this will work correctly on all devices.
                    #[cfg(target_os = "windows")]
                    backends: Some(bevy::render::settings::Backends::GL),
                    ..default()
                }),
                ..default()
            }),
        WebviewWryPlugin {
            local_root: std::path::PathBuf::from("ui"),
        },
        MeshPickingPlugin,
        #[cfg(feature = "develop")]
        bevy_inspector_egui::quick::WorldInspectorPlugin::default(),
    ))
    .add_plugins((AllDialogPlugins, AllLogPlugins, AppExitApiPlugin))
    .add_plugins((
        MenuPlugin,
        DesktopMascotPlugin,
        PowerStatePlugin,
        VrmPlugin,
        VrmaPlugin,
        ApplicationWindowsPlugin,
        AppSettingsPlugin,
        FileWatcherPlugin,
    ))
    .insert_resource(AmbientLight {
        brightness: 3000.0,
        ..default()
    })
    .insert_resource(ClearColor(Color::NONE))
    .run();
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

#[cfg(test)]
pub(crate) mod tests {
    use bevy::asset::AssetPlugin;
    use bevy::prelude::ImagePlugin;
    use bevy::render::camera::CameraPlugin;
    use bevy::window::WindowPlugin;
    use bevy::MinimalPlugins;
    use bevy_flurx::FlurxPlugin;

    pub type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

    #[macro_export]
    macro_rules! success {
        () => {
            std::result::Result::Ok(())
        };
    }

    pub fn test_app() -> bevy::app::App {
        let mut app = bevy::app::App::new();
        app.add_plugins((
            MinimalPlugins,
            AssetPlugin::default(),
            ImagePlugin::default(),
            WindowPlugin::default(),
            CameraPlugin,
            FlurxPlugin,
        ));
        app
    }
}
