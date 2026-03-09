#![allow(clippy::type_complexity, clippy::too_many_arguments)]
#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

mod cef_fetch;

use crate::cef_fetch::CefFetchPlugin;
use bevy::DefaultPlugins;
use bevy::app::{App, PluginGroup};
use bevy::asset::UnapprovedPathMode;
use bevy::input::common_conditions::{input_just_pressed, input_pressed};
use bevy::log::tracing_subscriber::Layer;
use bevy::log::{BoxedLayer, LogPlugin};
use bevy::prelude::*;
use bevy::window::{ExitCondition, WindowLevel, WindowPlugin, WindowResolution};
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
use homunculus_tray::HomunculusTrayPlugin;
use homunculus_utils::config::HomunculusConfig;
use homunculus_windows::HomunculusWindowsPlugin;
use std::sync::OnceLock;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling;

fn main() {
    // CEF subprocesses (renderer, GPU, utility) re-execute this binary.
    // Detect and exit before any Bevy/window initialization.
    #[cfg(not(target_os = "macos"))]
    bevy_cef::prelude::early_exit_if_subprocess();

    setup_panic_hook();

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
                        skip_taskbar: true,
                        window_level: WindowLevel::AlwaysOnTop,
                        ..default()
                    }),
                    exit_condition: ExitCondition::OnAllClosed,
                    ..default()
                })
                .set(AssetPlugin {
                    file_path: resolve_asset_path(),
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
        .add_plugins(HomunculusTrayPlugin)
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
                root_cache_path: Some(
                    homunculus_dir()
                        .join("cef_data")
                        .to_string_lossy()
                        .into_owned(),
                ),
            },
            CefFetchPlugin,
        ))
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

/// Resolves the asset directory path based on the current platform and build mode.
///
/// - **Dev builds** (CARGO_MANIFEST_DIR set): Uses `"assets"` relative to CWD (Cargo convention).
/// - **macOS release**: Uses `"../Resources/assets"` (inside `.app` bundle).
/// - **Windows release**: Uses exe-relative `"assets"` to avoid CWD dependency.
/// - **Fallback**: `"assets"` relative to CWD.
fn resolve_asset_path() -> String {
    // Dev builds: Cargo sets CARGO_MANIFEST_DIR, use default relative path
    if std::env::var("CARGO_MANIFEST_DIR").is_ok() {
        return "assets".to_string();
    }

    if cfg!(target_os = "macos") {
        return "../Resources/assets".to_string();
    }

    if cfg!(target_os = "windows") {
        // Use exe-relative path so MSI installs work regardless of CWD
        if let Ok(exe_path) = std::env::current_exe()
            && let Some(exe_dir) = exe_path.parent()
        {
            let assets_dir = exe_dir.join("assets");
            if assets_dir.exists() {
                return assets_dir.to_string_lossy().into_owned();
            }
        }
    }

    "assets".to_string()
}

/// Installs a custom panic hook that writes to a log file and shows a message box on Windows.
///
/// In release builds with `windows_subsystem = "windows"`, panics are invisible because
/// there is no console. This hook ensures panics are always visible to the user.
fn setup_panic_hook() {
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let message = format!("Desktop Homunculus crashed:\n\n{info}");

        // Write panic to log file
        let log_dir = homunculus_dir().join("Logs");
        let _ = std::fs::create_dir_all(&log_dir);
        let log_path = log_dir.join("panic.log");
        if let Ok(mut file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)
        {
            use std::io::Write;
            let _ = writeln!(file, "{message}");
        }

        // Show message box on Windows release builds (no console available)
        #[cfg(all(target_os = "windows", not(debug_assertions)))]
        show_error_message_box(&message);

        default_hook(info);
    }));
}

/// Shows a native Windows error dialog. Used only in release builds where
/// `windows_subsystem = "windows"` suppresses the console.
#[cfg(all(target_os = "windows", not(debug_assertions)))]
fn show_error_message_box(message: &str) {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    const MB_OK: u32 = 0x0000_0000;
    const MB_ICONERROR: u32 = 0x0000_0010;

    unsafe extern "system" {
        fn MessageBoxW(hwnd: *mut u8, text: *const u16, caption: *const u16, typ: u32) -> i32;
    }

    let wide_message: Vec<u16> = OsStr::new(message).encode_wide().chain(Some(0)).collect();
    let wide_title: Vec<u16> = OsStr::new("Desktop Homunculus - Error")
        .encode_wide()
        .chain(Some(0))
        .collect();
    // SAFETY: MessageBoxW is a standard Win32 API call with null parent window.
    unsafe {
        MessageBoxW(
            std::ptr::null_mut(),
            wide_message.as_ptr(),
            wide_title.as_ptr(),
            MB_OK | MB_ICONERROR,
        );
    }
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
