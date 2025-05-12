mod close;
mod is_closed;
mod open;

use crate::api;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy_vrm1::prelude::VrmBone;
use bevy_webview_wry::prelude::*;
use homunculus_core::prelude::{BoneOffsets, Coordinate, ModModuleSource};

api!(WebviewApi);

pub(super) struct WebviewApiPlugin;

impl Plugin for WebviewApiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ClosingWebviewSounds>()
            .add_systems(
                PreUpdate,
                (
                    tracking_to_vrm,
                    #[cfg(target_os = "macos")]
                    macos::bring_to_front_webview,
                )
                    .run_if(any_with_component::<Webview>),
            )
            .add_observer(request_move_window);
    }
}

#[derive(Resource, Debug, Default)]
struct ClosingWebviewSounds(HashMap<Entity, ModModuleSource>);

#[derive(Component, Debug, Clone)]
struct WebviewTracking {
    vrm: Entity,
    bone: Option<VrmBone>,
    offset: Option<IVec2>,
}

fn tracking_to_vrm(
    pat_commands: ParallelCommands,
    webviews: Query<(Entity, &WebviewTracking)>,
    coordinate: Coordinate,
    vrms: Query<&Transform>,
    bone_offsets: BoneOffsets,
) {
    webviews.par_iter().for_each(|(window, tracking)| {
        if let Ok(tf) = vrms.get(tracking.vrm)
            && let Some(offset) = Some(
                tracking
                    .bone
                    .as_ref()
                    .and_then(|n| bone_offsets.offset(tracking.vrm, n))
                    .unwrap_or_default(),
            )
            && let Some(viewport) = coordinate.to_global_by_world(tf.translation + offset)
        {
            let position =
                WindowPosition::At(viewport.as_ivec2() + tracking.offset.unwrap_or(IVec2::ZERO));
            pat_commands.command_scope(move |mut commands| {
                commands
                    .entity(window)
                    .trigger(RequestMoveWindow { position });
            });
        }
    });
}

#[derive(Event)]
struct RequestMoveWindow {
    position: WindowPosition,
}

fn request_move_window(trigger: Trigger<RequestMoveWindow>, mut windows: Query<&mut Window>) {
    if let Ok(mut window) = windows.get_mut(trigger.target()) {
        window.position = trigger.position;
    }
}

#[cfg(target_os = "macos")]
mod macos {
    use bevy::winit::WinitWindows;
    use bevy_webview_wry::core::Webview;
    use homunculus_core::prelude::AppWindow;
    use homunculus_effects::{Entity, NonSend, Query, With};
    use objc2::__framework_prelude::Retained;
    use objc2_app_kit::{NSView, NSWindow, NSWindowOrderingMode};
    #[allow(deprecated)]
    use winit::raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

    /// Forces the webview to be brought to the front when the app window is focused.
    pub fn bring_to_front_webview(
        winit_windows: NonSend<WinitWindows>,
        app_windows: Query<Entity, With<AppWindow>>,
        webviews: Query<Entity, With<Webview>>,
    ) {
        for app_window_entity in app_windows.iter() {
            let Some(app_window) = winit_windows.get_window(app_window_entity) else {
                continue;
            };
            if !app_window.has_focus() {
                continue;
            }
            for webview in webviews.iter() {
                if let Some(webview_window) = winit_windows.get_window(webview)
                    && let Some(webview_ns_window) = obtain_ns_window(webview_window)
                    && let Some(app_ns_window) = obtain_ns_window(app_window)
                {
                    unsafe {
                        webview_ns_window.orderWindow_relativeTo(
                            NSWindowOrderingMode::Above,
                            app_ns_window.windowNumber(),
                        );
                    }
                };
            }
        }
    }

    fn obtain_ns_window(window: &winit::window::Window) -> Option<Retained<NSWindow>> {
        #[allow(deprecated)]
        let ns_window = window.raw_window_handle().ok()?;
        if let RawWindowHandle::AppKit(handle) = ns_window {
            let ns_ptr = handle.ns_view.as_ptr();
            let ns_view: Retained<NSView> = unsafe { Retained::retain(ns_ptr.cast())? };
            ns_view.window()
        } else {
            None
        }
    }
}
