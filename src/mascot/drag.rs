// mod index;

use crate::global_mouse::cursor::GlobalMouseCursor;
use crate::global_window::{obtain_global_windows, GlobalWindows};
use crate::mascot::sitting::SittingWindow;
use crate::mascot::{Mascot, MascotEntity};
use crate::settings::state::{ActionGroup, ActionName, MascotAction};
use crate::system_param::mascot_tracker::MascotTracker;
use crate::system_param::window_layers::WindowLayers;
use bevy::app::{App, Plugin};
use bevy::hierarchy::{HierarchyQueryExt, Parent};
use bevy::log::debug;
use bevy::prelude::{Commands, Drag, DragEnd, DragStart, Entity, Pointer, PointerButton, Query, Res, Trigger};
use bevy::render::camera::NormalizedRenderTarget;

pub struct MascotDragPlugin;

impl Plugin for MascotDragPlugin {
    fn build(&self, app: &mut App) {
        app
            .world_mut()
            .register_component_hooks::<Mascot>()
            .on_add(|mut world, entity, _| {
                world
                    .commands()
                    .entity(entity)
                    .observe(on_drag_start)
                    .observe(on_drag_move)
                    .observe(on_drag_drop);
            });
    }
}

fn on_drag_start(
    trigger: Trigger<Pointer<DragStart>>,
    mut commands: Commands,
    states: Query<&MascotAction>,
    parents: Query<&Parent>,
) {
    if !matches!(trigger.event.button, PointerButton::Primary) {
        return;
    }
    let mascot_entity = parents.root_ancestor(trigger.target);
    if not_playing_sit_down(&states, mascot_entity) {
        debug!("on_drag_start {:?}", trigger.pointer_location.position);
        commands.entity(mascot_entity).insert(MascotAction::from_group(ActionGroup::drag()));
    }
}

fn not_playing_sit_down(
    states: &Query<&MascotAction>,
    mascot_entity: Entity,
) -> bool {
    states.get(mascot_entity).is_ok_and(|state| {
        state != &MascotAction {
            group: ActionGroup::sit_down(),
            name: ActionName::index(),
        }
    })
}

fn on_drag_move(
    trigger: Trigger<Pointer<Drag>>,
    mut commands: Commands,
    tracker: MascotTracker,
    windows: WindowLayers,
) {
    let mascot = MascotEntity(trigger.entity());
    let NormalizedRenderTarget::Window(window_ref) = trigger.pointer_location.target else {
        return;
    };
    let Some(global) = windows.to_global_pos(window_ref.entity(), trigger.pointer_location.position) else {
        return;
    };
    let Some(transform) = tracker.tracking_on_drag(mascot, global) else {
        return;
    };
    commands.entity(mascot.0).insert(transform);
}

fn on_drag_drop(
    trigger: Trigger<Pointer<DragEnd>>,
    mut commands: Commands,
    tracker: MascotTracker,
    global_cursor: Res<GlobalMouseCursor>,
    #[cfg(target_os = "windows")]
    // To run on main thread
    _: bevy::prelude::NonSend<bevy::winit::WinitWindows>,
) {
    let global_cursor_pos = global_cursor.global();
    let mascot = MascotEntity(trigger.entity());
    let global_windows: GlobalWindows = obtain_global_windows().unwrap_or_default();
    match global_windows.find_sitting_window(global_cursor_pos) {
        Some(global_window) => {
            let sitting_pos = global_window.sitting_pos(global_cursor_pos);
            let sitting_window = SittingWindow::new(global_window, sitting_pos);
            let Some(transform) = tracker.tracking_on_sitting(mascot, sitting_window.sitting_pos()) else {
                return;
            };
            debug!("Sitting application_windows: {:?}", sitting_window.window.title);
            commands.entity(mascot.0).insert((
                sitting_window,
                transform,
                MascotAction::from_group(ActionGroup::sit_down()),
            ));
        }
        None => {
            commands.entity(mascot.0).insert(MascotAction {
                group: ActionGroup::drag(),
                name: ActionName::drop(),
            });
        }
    }
}

