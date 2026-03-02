#![allow(clippy::type_complexity, clippy::too_many_arguments)]
#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

use bevy::DefaultPlugins;
use bevy::app::{App, PluginGroup};
use bevy::asset::UnapprovedPathMode;
use bevy::input::common_conditions::{input_just_pressed, input_pressed};
use bevy::log::tracing_subscriber::Layer;
use bevy::log::{BoxedLayer, LogPlugin};
use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::render::settings::{PowerPreference, RenderCreation, WgpuSettings};
use bevy::window::{ExitCondition, WindowPlugin, WindowResolution};
use bevy_cef::CefPlugin;
use bevy_cef::prelude::*;
use bevy_flurx::FlurxPlugin;
use bevy_vrm1::vrm::VrmPlugin;
use bevy_vrm1::vrma::VrmaPlugin;
use homunculus_api::HomunculusApiPlugin;
use homunculus_audio::HomunculusAudioPlugin;
use homunculus_core::HomunculusCorePlugin;
use homunculus_core::prelude::homunculus_dir;
use homunculus_drag::HomunculusDragPlugin;
use homunculus_hit_test::HomunculusHitTestPlugin;
use homunculus_http_server::HomunculusHttpServerPlugin;
use homunculus_mod::HomunculusModPlugin;
use homunculus_power_saver::HomunculusPowerSaverPlugin;
use homunculus_prefs::HomunculusPrefsPlugin;
use homunculus_screen::HomunculusScreenPlugin;
use homunculus_shadow_panel::HomunculusShadowPanelPlugin;
use homunculus_sitting::HomunculusSittingPlugin;
use homunculus_speech::HomunculusSpeechPlugin;
use homunculus_utils::config::HomunculusConfig;
use homunculus_windows::HomunculusWindowsPlugin;
use std::sync::OnceLock;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling;

mod cef_fetch;
mod graceful_shutdown;

use crate::cef_fetch::CefFetchPlugin;
use crate::graceful_shutdown::GracefulShutdownPlugin;

fn main() {
    let config = HomunculusConfig::load().unwrap_or_else(|e| {
        eprintln!("Failed to load config: {e}");
        HomunculusConfig::default()
    });

    let mut app = App::new();
    app.insert_resource(config)
        .insert_resource(ClearColor(Color::NONE))
        .add_plugins((
            HomunculusModPlugin,
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
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(0, 0),
                        decorations: false,
                        transparent: true,
                        ..default()
                    }),
                    exit_condition: ExitCondition::OnAllClosed,
                    ..default()
                })
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        power_preference: PowerPreference::LowPower,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    file_path: if std::env::var("CARGO_MANIFEST_DIR").is_ok() {
                        "assets".to_string()
                    } else {
                        "../Resources/assets".to_string()
                    },
                    unapproved_path_mode: UnapprovedPathMode::Allow,
                    ..default()
                }),
            MeshPickingPlugin,
            #[cfg(feature = "develop")]
            bevy_egui::EguiPlugin::default(),
            #[cfg(feature = "develop")]
            bevy_inspector_egui::quick::WorldInspectorPlugin::default(),
        ))
        .add_plugins((FlurxPlugin, VrmPlugin, VrmaPlugin))
        .add_plugins((
            HomunculusCorePlugin,
            HomunculusAudioPlugin,
            HomunculusDragPlugin,
            HomunculusWindowsPlugin,
            HomunculusPowerSaverPlugin,
            HomunculusScreenPlugin,
            HomunculusSittingPlugin,
            HomunculusShadowPanelPlugin,
            HomunculusSpeechPlugin,
            HomunculusApiPlugin,
            HomunculusHitTestPlugin,
            HomunculusPrefsPlugin,
            HomunculusHttpServerPlugin,
            CefPlugin {
                command_line_config: CommandLineConfig::default()
                    .with_switch("disable-web-security"),
                extensions: CefExtensions::new().add("cef-fetch", include_str!("./cef_fetch.js")),
            },
            CefFetchPlugin,
        ))
        .add_plugins(GracefulShutdownPlugin)
        .add_systems(
            Update,
            (
                webview_navigate_back.run_if(input_just_pressed(MouseButton::Back).or(
                    input_pressed(KeyCode::SuperLeft).and(input_just_pressed(KeyCode::BracketLeft)),
                )),
                webview_navigate_forward.run_if(
                    input_just_pressed(MouseButton::Forward).or(input_pressed(KeyCode::SuperLeft)
                        .and(input_just_pressed(KeyCode::BracketRight))),
                ),
                show_devtool.run_if(input_just_pressed(KeyCode::F1)),
                close_devtool.run_if(input_just_pressed(KeyCode::F2)),
            ),
        )
        .run();
}

fn custom_layer(_app: &mut App) -> Option<BoxedLayer> {
    let file_appender = rolling::daily(homunculus_dir().join("Logs"), "log.txt");
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

fn webview_navigate_back(mut commands: Commands, webviews: Query<Entity, With<WebviewSource>>) {
    for webview in webviews.iter() {
        commands.trigger(RequestGoBack { webview });
    }
}

fn webview_navigate_forward(mut commands: Commands, webviews: Query<Entity, With<WebviewSource>>) {
    for webview in webviews.iter() {
        commands.trigger(RequestGoForward { webview });
    }
}

fn show_devtool(mut commands: Commands, webviews: Query<Entity, With<WebviewSource>>) {
    for webview in webviews.iter() {
        commands.trigger(RequestShowDevTool { webview });
    }
}

fn close_devtool(mut commands: Commands, webviews: Query<Entity, With<WebviewSource>>) {
    for webview in webviews.iter() {
        commands.trigger(RequestCloseDevtool { webview });
    }
}
