use crate::global_window::{obtain_global_windows, GlobalWindows};
use crate::mascot::sitting::SittingWindow;
use crate::mascot::{Mascot, MascotEntity};
use crate::settings::preferences::action::ActionName;
use crate::system_param::mascot_tracker::MascotTracker;
use crate::system_param::windows::Windows;
use crate::system_param::GlobalScreenPos;
use bevy::app::{App, Plugin, Update};
use bevy::ecs::component::HookContext;
use bevy::log::debug;
use bevy::prelude::*;
use bevy::render::camera::NormalizedRenderTarget;
use bevy_vrma::system_param::cameras::Cameras;
use bevy_vrma::vrma::retarget::RetargetBindingSystemSet;
use std::fmt::Debug;

pub struct MascotDragPlugin;

impl Plugin for MascotDragPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_systems(Update, on_drag_index.after(RetargetBindingSystemSet));

        app.world_mut().register_component_hooks::<Mascot>().on_add(
            |mut world, context: HookContext| {
                world
                    .commands()
                    .entity(context.entity)
                    .observe(on_drag_start)
                    .observe(on_drag_move)
                    .observe(on_drag_drop);
            },
        );
    }
}

fn on_drag_start(
    trigger: Trigger<Pointer<DragStart>>,
    mut commands: Commands,
    tracker: MascotTracker,
    windows: Windows,
    actions: Query<&ActionName>,
) {
    if !matches!(trigger.event.button, PointerButton::Primary) {
        return;
    }
    let mascot = MascotEntity(trigger.observer());
    if not_playing_sit_down(&actions, mascot.0) {
        let Some(global) = global_cursor_pos(&trigger, &windows) else {
            return;
        };
        let Some(transform) = tracker.tracking(mascot, global, 1.) else {
            return;
        };
        commands
            .entity(mascot.0)
            .insert((ActionName::drag_start(), transform));
    }
}

fn not_playing_sit_down(
    actions: &Query<&ActionName>,
    mascot_entity: Entity,
) -> bool {
    actions
        .get(mascot_entity)
        .is_ok_and(|action| !action.is_sit_down())
}

/// This system is executed while the character is floating up after starting the drag.
/// Adjust the position of the character's Hips to match the mouse cursor position.
fn on_drag_index(
    par_commands: ParallelCommands,
    windows: Windows,
    tracker: MascotTracker,
    mascots: Query<(Entity, &ActionName)>,
) {
    mascots.par_iter().for_each(|(entity, mascot_action)| {
        if !mascot_action.is_drag_start() {
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
    let mascot = MascotEntity(trigger.observer());
    let NormalizedRenderTarget::Window(window_ref) = trigger.pointer_location.target else {
        return;
    };
    let drag_pos = trigger.pointer_location.position;
    let Some(origin) =
        cameras.to_world_pos_from_viewport(window_ref.entity(), drag_pos - trigger.delta)
    else {
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
    let mascot = MascotEntity(trigger.observer());
    let global_windows: GlobalWindows = obtain_global_windows().unwrap_or_default();
    match global_windows.find_sitting_window(global_cursor_pos) {
        Some(global_window) => {
            let sitting_pos = global_window.sitting_pos(global_cursor_pos);
            let sitting_window = SittingWindow::new(global_window, sitting_pos);
            let Some(transform) = tracker.tracking_on_sitting(mascot, sitting_window.sitting_pos())
            else {
                return;
            };
            debug!(
                "Sitting application_windows: {:?}",
                sitting_window.window.title
            );
            commands
                .entity(mascot.0)
                .insert((sitting_window, transform, ActionName::sit_down()));
        }
        None => {
            commands.entity(mascot.0).insert(ActionName::drop());
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
