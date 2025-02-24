use crate::mascot::MascotEntity;
use crate::settings::state::MascotAction;
use crate::vrma::animation::{all_animation_finished, changed_other_state};
use crate::vrma::VrmaEntity;
use bevy::app::{App, Plugin, Update};
use bevy::log::debug;
use bevy::prelude::{Commands, Event, EventReader, In};
use bevy_flurx::prelude::*;

#[derive(Debug, Event)]
pub struct ScheduleTransition {
    pub mascot: MascotEntity,
    pub vrma: VrmaEntity,
    pub current: MascotAction,
    pub next: MascotAction,
}

pub struct VrmaScheduleTransitionPlugin;

impl Plugin for VrmaScheduleTransitionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ScheduleTransition>()
            .add_systems(Update, request_schedule_transition);
    }
}

fn request_schedule_transition(
    mut commands: Commands,
    mut er: EventReader<ScheduleTransition>,
) {
    for event in er.read() {
        let mascot = event.mascot;
        let vrma = event.vrma;
        let current = event.current.clone();
        let next = event.next.clone();
        debug!("Schedule transition: {:?}", next);

        commands.spawn(Reactor::schedule(move |task| async move {
            let can_transition = task.will(Update, wait::either(
                wait::until(all_animation_finished).with(vrma),
                wait::until(changed_other_state).with((mascot, current)),
            ))
                .await
                .is_left();
            if can_transition {
                task.will(Update, once::run(transition).with((mascot, next))).await;
            }
        }));
    }
}

fn transition(
    In((mascot, next)): In<(MascotEntity, MascotAction)>,
    mut commands: Commands,
) {
    commands.entity(mascot.0).insert(next);
}