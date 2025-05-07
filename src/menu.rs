mod actions;
mod load_mascot;
mod reset_position;
mod scale;

use crate::mascot::Mascot;
use crate::menu::actions::{request_send_actions, MenuActionsPlugin};
use crate::menu::load_mascot::load_mascot;
use crate::menu::reset_position::MenuResetPositionPlugin;
use crate::menu::scale::MenuScalePlugin;
use crate::system_param::mesh_aabb::MascotAabb;
use crate::system_param::monitors::{monitor_rect, Monitors};
use crate::system_param::windows::Windows;
use bevy::app::{App, Plugin, PostUpdate, Update};
use bevy::math::{Rect, Vec2};
use bevy::picking::events::Click;
use bevy::prelude::*;
use bevy::render::camera::NormalizedRenderTarget;
use bevy::render::view::RenderLayers;
use bevy::utils::default;
use bevy::window::{Window, WindowPosition, WindowResolution};
use bevy::winit::WinitWindows;
use bevy_flurx::action::once;
use bevy_flurx::prelude::ReactorTask;
use bevy_vrma::system_param::cameras::Cameras;
use bevy_webview_wry::ipc::IpcHandlers;
use bevy_webview_wry::prelude::*;
use winit::dpi::PhysicalPosition;

#[derive(Component)]
pub struct TargetMascot(pub Entity);

#[derive(Component)]
pub struct Menu;

#[derive(Component)]
pub struct MenuUnInitialized;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_plugins((MenuScalePlugin, MenuActionsPlugin, MenuResetPositionPlugin))
            .add_systems(
                Update,
                (
                    mark_initialized_menu.run_if(any_with_component::<MenuUnInitialized>),
                    request_close_event.run_if(any_with_component::<Menu>),
                ),
            )
            .add_systems(PostUpdate, register_observer);
    }
}

fn register_observer(
    mut commands: Commands,
    mascots: Query<Entity, Added<Mascot>>,
) {
    for mascot in mascots.iter() {
        let mut observer = Observer::new(open_menu);
        observer.watch_entity(mascot);
        commands.spawn(observer);
    }
}

const MENU_SIZE: Vec2 = Vec2::new(500., 600.);

fn open_menu(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    monitors: Monitors,
    windows: Windows,
    parents: Query<&ChildOf>,
    menus: Query<&Menu>,
    mascots: Query<&Name>,
) {
    if !(matches!(trigger.event.button, PointerButton::Secondary) && menus.is_empty()) {
        return;
    }
    let NormalizedRenderTarget::Window(window_ref) = trigger.pointer_location.target else {
        return;
    };
    let Some(global_cursor_pos) =
        windows.to_global_pos(window_ref.entity(), trigger.pointer_location.position)
    else {
        return;
    };
    let Some((_, monitor, _)) = monitors.find_monitor_from_global_screen_pos(global_cursor_pos)
    else {
        return;
    };
    let (position, resolution) = fit_position(*global_cursor_pos, &monitor_rect(monitor));
    let mascot_entity = parents.root_ancestor(trigger.target);
    commands.spawn((
        Menu,
        MenuUnInitialized,
        Name::new("Menu"),
        TargetMascot(mascot_entity),
        Window {
            title: mascots
                .get(mascot_entity)
                .cloned()
                .unwrap_or_default()
                .to_string(),
            resizable: false,
            resolution,
            position: WindowPosition::At(position.as_ivec2()),
            window_level: bevy::window::WindowLevel::AlwaysOnTop,
            ..default()
        },
        WebViewBundle {
            ipc_handlers: IpcHandlers::new([
                load_mascot,
                get_scale,
                get_mascot_name,
                request_send_actions,
            ]),
            use_devtools: UseDevtools(true),
            background: Background::Transparent,
            ..default()
        },
    ));
}

fn fit_position(
    cursor_pos: Vec2,
    monitor_frame: &Rect,
) -> (Vec2, WindowResolution) {
    let menu_window_frame = Rect::new(
        cursor_pos.x,
        cursor_pos.y,
        cursor_pos.x + MENU_SIZE.x,
        cursor_pos.y + MENU_SIZE.y,
    );
    let dx = (menu_window_frame.max.x - monitor_frame.max.x).max(0.);
    let dy = (menu_window_frame.max.y - monitor_frame.max.y).max(0.);
    (
        cursor_pos - Vec2::new(dx, dy),
        WindowResolution::new(MENU_SIZE.x, MENU_SIZE.y),
    )
}

fn mark_initialized_menu(
    mut commands: Commands,
    cameras: Cameras,
    monitors: Monitors,
    winit_windows: NonSend<WinitWindows>,
    menus: Query<(Entity, &RenderLayers, &TargetMascot), With<MenuUnInitialized>>,
    mascot_aabb: MascotAabb,
) {
    for (menu_entity, layers, TargetMascot(mascot_entity)) in menus.iter() {
        if let Some(winit_window) = winit_windows.get_window(menu_entity) {
            let monitor_pos = monitors.monitor_pos(layers).unwrap_or_default();
            let scale_factor = monitors.scale_factor(layers).unwrap_or(1.0);
            let (_, max) = mascot_aabb.calculate(*mascot_entity);
            let max = cameras.to_viewport_pos(layers, max).unwrap_or_default();
            let max = monitor_pos + max;
            let pos = max * scale_factor;
            winit_window.set_outer_position(PhysicalPosition::new(pos.x, pos.y));
            commands.entity(menu_entity).remove::<MenuUnInitialized>();
        }
    }
}

fn request_close_event(
    mut commands: Commands,
    winit_windows: NonSend<WinitWindows>,
    menus: Query<Entity, (With<Menu>, Without<MenuUnInitialized>)>,
) {
    for entity in menus.iter() {
        let is_visible = winit_windows
            .get_window(entity)
            .and_then(|window| window.is_visible())
            .unwrap_or(false);
        if !is_visible {
            commands.entity(entity).despawn();
        }
    }
}

#[command]
async fn get_mascot_name(
    entity: WebviewEntity,
    task: ReactorTask,
) -> Option<String> {
    task.will(Update, once::run(mascot_name).with(entity)).await
}

fn mascot_name(
    In(entity): In<WebviewEntity>,
    target: Query<&TargetMascot>,
    mascot: Query<&Name>,
) -> Option<String> {
    target
        .get(entity.0)
        .ok()
        .and_then(|TargetMascot(mascot_entity)| {
            mascot.get(*mascot_entity).map(|name| name.to_string()).ok()
        })
}

#[command]
async fn get_scale(
    entity: WebviewEntity,
    task: ReactorTask,
) -> Option<f32> {
    task.will(Update, once::run(scale).with(entity)).await
}

fn scale(
    In(entity): In<WebviewEntity>,
    target: Query<&TargetMascot>,
    mascot: Query<&Transform>,
) -> Option<f32> {
    target
        .get(entity.0)
        .ok()
        .and_then(|TargetMascot(mascot_entity)| {
            mascot.get(*mascot_entity).map(|tf| tf.scale.x).ok()
        })
}
