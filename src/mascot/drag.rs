// mod index;

use crate::global_mouse::button::GlobalMouseButton;
use crate::global_mouse::cursor::GlobalMouseCursor;
use crate::global_window::{obtain_global_windows, GlobalWindows};
use crate::mascot::sitting::SittingWindow;
use crate::mascot::{Mascot, MascotEntity};
use crate::settings::state::{ActionGroup, ActionName, MascotAction};
use crate::system_param::mascot_controller::MascotTracker;
use crate::system_param::mascot_root_searcher::MascotRootSearcher;
use crate::system_param::monitors::Monitors;
use crate::system_param::mouse_position::MousePosition;
use bevy::app::{App, Plugin, Update};
use bevy::ecs::system::NonSend;
use bevy::input::common_conditions::{input_just_released, input_pressed};
use bevy::log::debug;
use bevy::prelude::{Commands, DragStart, Entity, IntoSystemConfigs, ParallelCommands, Pointer, PointerButton, Query, Res, Trigger};
use bevy::render::view::RenderLayers;
use bevy::winit::WinitWindows;

pub struct MascotDragPlugin;

impl Plugin for MascotDragPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                on_drag_move.run_if(input_pressed(GlobalMouseButton::Left)),
                on_drag_drop.run_if(input_just_released(GlobalMouseButton::Left)),
            ));

        app
            .world_mut()
            .register_component_hooks::<Mascot>()
            .on_add(|mut world, entity, _| {
                world
                    .commands()
                    .entity(entity)
                    .observe(on_drag_start);
            });
    }
}

fn on_drag_start(
    trigger: Trigger<Pointer<DragStart>>,
    mut commands: Commands,
    roots: MascotRootSearcher,
    states: Query<&MascotAction>,
) {
    if !matches!(trigger.event.button, PointerButton::Primary) {
        return;
    }
    if let Some(mascot_entity) = roots.find_root(trigger.target) {
        if not_playing_sit_down(&states, mascot_entity) {
            debug!("on_drag_start {:?}", trigger.pointer_location.position);
            commands.entity(mascot_entity).insert(MascotAction::from_main(ActionGroup::drag()));
        }
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
    par_commands: ParallelCommands,
    tracker: MascotTracker,
    mouse_position: MousePosition,
    move_targets: Query<(Entity, &RenderLayers, &MascotAction)>,
) {
    move_targets.par_iter().for_each(|(entity, layers, state)| {
        if !state.group.is_drag() {
            return;
        }
        let Some(cursor_pos) = mouse_position.local(layers) else {
            return;
        };
        let Some(transform) = tracker.tracking_on_drag(MascotEntity(entity), cursor_pos) else {
            return;
        };
        par_commands.command_scope(|mut commands| {
            commands.entity(entity).insert(transform);
        });
    });
}

fn on_drag_drop(
    par_commands: ParallelCommands,
    tracker: MascotTracker,
    global_cursor: Res<GlobalMouseCursor>,
    monitors: Monitors,
    move_targets: Query<(Entity, &RenderLayers, &MascotAction)>,
    // To run on main thread
    _: NonSend<WinitWindows>,
) {
    let global_cursor_pos = global_cursor.global_cursor_pos();
    for (entity, layers, state) in move_targets.iter(){
        if !state.group.is_drag() {
            return;
        }
        let mascot = MascotEntity(entity);
        let global_windows: GlobalWindows = obtain_global_windows().unwrap_or_default();
        println!("{global_windows:#?}");
        match global_windows.find_sitting_window(global_cursor_pos) {
            Some(global_window) => {
                let global_sitting_pos = global_window.sitting_pos(global_cursor_pos);
                let sitting_window = SittingWindow::new(global_window, global_sitting_pos);
                let Some(transform) = monitors
                    .monitor_pos(layers)
                    .and_then(|monitor_pos| {
                        tracker.tracking_on_sitting(mascot, sitting_window.global_sitting_pos() - monitor_pos)
                    })
                else {
                    return;
                };
                debug!("Sitting application_windows: {:?}", sitting_window.window.title);
                par_commands.command_scope(|mut commands| {
                    commands.entity(entity).insert((
                        sitting_window,
                        transform,
                        MascotAction::from_main(ActionGroup::sit_down()),
                    ));
                });
            }
            None => {
                par_commands.command_scope(|mut commands| {
                    commands.entity(entity).insert(MascotAction {
                        group: ActionGroup::drag(),
                        name: ActionName::drop(),
                    });
                });
            }
        }
    }
}

