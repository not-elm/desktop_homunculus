// mod index;

use crate::global_window::{obtain_global_windows, GlobalWindows};
use crate::mascot::sitting::SittingWindow;
use crate::mascot::{Mascot, MascotEntity};
use crate::settings::state::{ActionGroup, ActionName, MascotAction};
use crate::system_param::bone_offsets::BoneOffsets;
use crate::system_param::cameras::Cameras;
use crate::system_param::mascot_tracker::MascotTracker;
use crate::system_param::windows::Windows;
use crate::system_param::GlobalScreenPos;
use crate::vrma::retarget::RetargetBindingSystemSet;
use bevy::app::{App, Plugin, PostUpdate, Update};
use bevy::hierarchy::{HierarchyQueryExt, Parent};
use bevy::log::debug;
use bevy::prelude::{Cancel, Commands, Down, Drag, DragEnd, DragOver, DragStart, Entity, IntoSystemConfigs, Over, ParallelCommands, Pointer, PointerButton, Query, Reflect, Transform, Trigger};
use bevy::render::camera::NormalizedRenderTarget;
use bevy_flurx::action::once::switch::off;
use std::fmt::Debug;

pub struct MascotDragPlugin;

impl Plugin for MascotDragPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_drag_index.after(RetargetBindingSystemSet));

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
    tracker: MascotTracker,
    windows: Windows,
    states: Query<&MascotAction>,
) {
    if !matches!(trigger.event.button, PointerButton::Primary) {
        return;
    }
    let mascot = MascotEntity(trigger.entity());
    if not_playing_sit_down(&states, mascot.0) {
        let Some(global) = global_cursor_pos(&trigger, &windows) else {
            return;
        };
        let Some(transform) = tracker.tracking(mascot, global, 1.) else {
            return;
        };
        commands.entity(mascot.0).insert((
            MascotAction::from_group(ActionGroup::drag()),
            transform,
        ));
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

/// This system is executed while the character is floating up after starting the drag.
/// Adjust the position of the character's Hips to match the mouse cursor position.
fn on_drag_index(
    par_commands: ParallelCommands,
    windows: Windows,
    tracker: MascotTracker,
    mascots: Query<(Entity, &MascotAction)>,
) {
    mascots.par_iter().for_each(|(entity, mascot_action)| {
        if !(mascot_action.group.is_drag() && mascot_action.name.is_index()) {
            return;
        }
        let Some(global_cursor_pos) = windows.global_cursor_pos() else {
            return;
        };
        let Some(new_tf) = tracker.tracking_on_drag(MascotEntity(entity), global_cursor_pos) else {
            return;
        };
        par_commands.command_scope(|mut commands| {
            commands.entity(entity).insert(new_tf);
        });
    });
}

fn on_drag_move(
    trigger: Trigger<Pointer<Drag>>,
    mut commands: Commands,
    cameras: Cameras,
    transforms: Query<&Transform>,
) {
    if !matches!(trigger.event.button, PointerButton::Primary) {
        return;
    }
    let mascot = MascotEntity(trigger.entity());
    let NormalizedRenderTarget::Window(window_ref) = trigger.pointer_location.target else {
        return;
    };
    let drag_pos = trigger.pointer_location.position;
    let Some(origin) = cameras.to_world_pos_from_viewport(window_ref.entity(), drag_pos - trigger.delta) else {
        return;
    };
    let Some(current) = cameras.to_world_pos_from_viewport(window_ref.entity(), drag_pos) else {
        return;
    };
    let Ok(transform) = transforms.get(mascot.0) else {
        return;
    };
    let delta = current - origin;
    commands.entity(mascot.0).insert(Transform {
        translation: transform.translation + delta,
        ..*transform
    });
}

fn on_drag_drop(
    trigger: Trigger<Pointer<DragEnd>>,
    mut commands: Commands,
    tracker: MascotTracker,
    windows: Windows,
    #[cfg(target_os = "windows")]
    // To run on main thread
    _: bevy::prelude::NonSend<bevy::winit::WinitWindows>,
) {
    if !matches!(trigger.event.button, PointerButton::Primary) {
        return;
    }
    let Some(global_cursor_pos) = global_cursor_pos(&trigger, &windows) else {
        return;
    };
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

fn global_cursor_pos<E: Debug + Clone + Reflect>(
    trigger: &Trigger<Pointer<E>>,
    windows: &Windows,
) -> Option<GlobalScreenPos> {
    let NormalizedRenderTarget::Window(window_ref) = trigger.pointer_location.target else {
        return None;
    };
    windows.to_global_pos(window_ref.entity(), trigger.pointer_location.position)
}