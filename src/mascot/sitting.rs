use crate::global_mouse::button::GlobalMouseButton;
use crate::global_window::GlobalWindow;
use crate::mascot::MascotEntity;
use crate::settings::state::{ActionName, MascotAction};
use crate::system_param::mascot_tracker::MascotTracker;
use crate::system_param::mouse_position::MousePosition;
use crate::system_param::window_layers::{window_local_pos, WindowLayers};
use crate::system_param::GlobalScreenPos;
use bevy::app::{App, PostUpdate, Update};
use bevy::input::common_conditions::{input_just_pressed, input_just_released};
use bevy::math::Vec2;
use bevy::prelude::{debug, on_event, Changed, Commands, Component, Entity, Event, EventReader, EventWriter, IntoSystemConfigs, Local, ParallelCommands, Plugin, Query, Transform, With};
use bevy::utils::HashMap;
use bevy::window::RequestRedraw;
use itertools::Itertools;

#[derive(Event)]
struct MoveSittingPos {
    mascot: MascotEntity,
}

pub struct MascotSittingPlugin;

impl Plugin for MascotSittingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<MoveSittingPos>()
            .add_systems(Update, (
                start_tracking.run_if(input_just_pressed(GlobalMouseButton::Left)),
                track_to_sitting_window,
                end_tracking.run_if(input_just_released(GlobalMouseButton::Left)),
                remove_sitting_window,
                adjust_sitting_pos_on_scaling,
                adjust_sitting_pos_on_sit_down,
            ).run_if(any_mascots_sitting))
            .add_systems(PostUpdate, move_sitting_pos.run_if(on_event::<MoveSittingPos>));
    }
}

fn any_mascots_sitting(mascots: Query<&MascotAction>) -> bool {
    mascots.iter().any(|status| status.group.is_sit_down())
}

#[derive(Debug, Default, Clone, Component)]
pub struct SittingWindow {
    pub window: GlobalWindow,
    pub mascot_viewport_offset: Vec2,
    pub dragging: bool,
}

impl SittingWindow {
    pub fn new(
        global_window: GlobalWindow,
        sitting_pos: GlobalScreenPos,
    ) -> Self {
        Self {
            mascot_viewport_offset: *sitting_pos - global_window.frame.min,
            window: global_window,
            dragging: false,
        }
    }

    #[inline]
    pub fn update(&self) -> Option<Self> {
        let new_window = self.window.update()?;
        Some(Self {
            window: new_window,
            ..*self
        })
    }

    #[inline]
    pub fn sitting_pos(&self) -> GlobalScreenPos {
        GlobalScreenPos(self.window.frame.min + self.mascot_viewport_offset)
    }
}

fn adjust_sitting_pos_on_sit_down(
    mut ew: EventWriter<MoveSittingPos>,
    mascots: Query<(Entity, &MascotAction)>,
) {
    for (mascot_entity, state) in mascots.iter() {
        if state.name == ActionName::index() {
            ew.send(MoveSittingPos { mascot: MascotEntity(mascot_entity) });
        }
    }
}

fn adjust_sitting_pos_on_scaling(
    mut ew: EventWriter<MoveSittingPos>,
    mut scales: Local<HashMap<Entity, f32>>,
    mascots: Query<(Entity, &Transform), (Changed<Transform>, With<SittingWindow>)>,
) {
    for (entity, tf) in mascots.iter() {
        if let Some(prev_scale) = scales.get(&entity) {
            if f32::EPSILON < (prev_scale - tf.scale.x).abs() {
                ew.send(MoveSittingPos { mascot: MascotEntity(entity) });
            }
        }
        scales.insert(entity, tf.scale.x);
    }
}

fn move_sitting_pos(
    mut commands: Commands,
    mut redraw: EventWriter<RequestRedraw>,
    mut er: EventReader<MoveSittingPos>,
    tracker: MascotTracker,
    mascots: Query<&SittingWindow>,
) {
    for mascot in er
        .read()
        .map(|e| e.mascot)
        .unique()
    {
        if let Ok(sitting_window) = mascots.get(mascot.0) {
            let global = sitting_window.sitting_pos();
            if let Some(transform) = tracker.tracking_on_sitting(mascot, global) {
                commands.entity(mascot.0).insert(transform);
            }
        }
    }
    redraw.send(RequestRedraw);
}

fn start_tracking(
    mut redraw: EventWriter<RequestRedraw>,
    mut sitting_windows: Query<&mut SittingWindow>,
    mouse_position: MousePosition,
) {
    for mut sitting_window in sitting_windows
        .iter_mut()
        .filter(|s| !s.dragging)
    {
        if sitting_window.window.frame.contains(*mouse_position.global()) {
            debug!("Start tracking sitting application_windows: {:?}", sitting_window.window.title);
            sitting_window.dragging = true;
            redraw.send(RequestRedraw);
        }
    }
}

fn track_to_sitting_window(
    mut redraw: EventWriter<RequestRedraw>,
    par_commands: ParallelCommands,
    sitting_windows: Query<(Entity, &SittingWindow)>,
    tracker: MascotTracker,
) {
    sitting_windows.par_iter().for_each(|(mascot_entity, sitting_window)| {
        if !sitting_window.dragging {
            return;
        }
        let Some(new_sitting_window) = sitting_window.update() else {
            return;
        };
        let sitting_pos = new_sitting_window.sitting_pos();
        let Some(transform) = tracker.tracking_on_sitting(MascotEntity(mascot_entity), sitting_pos) else {
            return;
        };
        par_commands.command_scope(|mut commands| {
            commands.entity(mascot_entity).insert((
                new_sitting_window,
                transform,
            ));
        });
    });
    redraw.send(RequestRedraw);
}

fn end_tracking(
    par_commands: ParallelCommands,
    sitting_windows: Query<(Entity, &SittingWindow)>,
) {
    sitting_windows.par_iter().for_each(|(entity, sitting_window)| {
        par_commands.command_scope(|mut commands| {
            commands.entity(entity).insert(SittingWindow {
                dragging: false,
                mascot_viewport_offset: sitting_window.mascot_viewport_offset,
                window: sitting_window.window.clone(),
            });
        });
    });
}

fn remove_sitting_window(
    par_commands: ParallelCommands,
    mascots: Query<(Entity, &MascotAction), (Changed<MascotAction>, With<SittingWindow>)>,
) {
    mascots.par_iter().for_each(|(entity, status)| {
        if !status.group.is_sit_down() {
            par_commands.command_scope(|mut commands| {
                commands.entity(entity).remove::<SittingWindow>();
            });
        }
    });
}
