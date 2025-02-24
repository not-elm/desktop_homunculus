use crate::mascot::{Mascot, MascotEntity};
use crate::settings::state::MascotAction;
use crate::vrma::animation::changed_other_state;
use bevy::app::{App, Update};
use bevy::core::Name;
use bevy::hierarchy::Children;
use bevy::log::debug;
use bevy::prelude::{Commands, Event, EventReader, In, Plugin, Query, With, Without};
use bevy_flurx::prelude::*;
use rand::prelude::IteratorRandom;
use rand::Rng;
use std::time::Duration;

#[derive(Debug, Event, Copy, Clone)]
pub struct ScheduleAutoTransition {
    pub mascot: MascotEntity,
    pub min_secs: u64,
    pub max_secs: u64,
}

pub struct VrmaAutoTransitionPlugin;

impl Plugin for VrmaAutoTransitionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ScheduleAutoTransition>()
            .add_systems(Update, (
                schedule_auto_transition,
            ));
    }
}

fn schedule_auto_transition(
    mut commands: Commands,
    mut er: EventReader<ScheduleAutoTransition>,
    mascots: Query<(&Name, &MascotAction, &Children), With<Mascot>>,
    vrma: Query<&MascotAction, Without<Mascot>>,
) {
    for ScheduleAutoTransition { mascot, min_secs, max_secs, } in er.read().copied() {
        let Ok((name, current_state, children)) = mascots.get(mascot.0) else {
            continue;
        };
        let candidates = all_candidates(children, &vrma);
        let Some(next_state) = random_state(current_state, &candidates) else {
            continue;
        };
        let current_state = current_state.clone();
        let name = name.clone();
        let delay_time = Duration::from_secs(rand::rng().random_range(min_secs..=max_secs));

        commands.spawn(Reactor::schedule(move |task| async move {
            let can_transition = task.will(Update, wait_timer(
                mascot,
                name,
                current_state.clone(),
                next_state.clone(),
                delay_time,
            )).await;

            if can_transition {
                task.will(Update, once::run(transition_state).with((
                    mascot,
                    next_state,
                ))).await;
            }
        }));
    }
}

fn all_candidates(
    children: &Children,
    vrma: &Query<&MascotAction, Without<Mascot>>,
) -> Vec<MascotAction> {
    children
        .iter()
        .filter_map(|child| vrma.get(*child).ok())
        .cloned()
        .collect()
}

fn random_state(
    current: &MascotAction,
    candidates: &[MascotAction],
) -> Option<MascotAction> {
    candidates
        .iter()
        .filter(|s| *s != current)
        .filter(|s| s.group == current.group)
        .choose(&mut rand::rng())
        .cloned()
}

fn wait_timer(
    mascot: MascotEntity,
    name: Name,
    current: MascotAction,
    next: MascotAction,
    delay_time: Duration,
) -> ActionSeed<(), bool> {
    ActionSeed::define(move |_| {
        debug!("{name:?} will translate from {current} to {next} after {delay_time:?}");

        delay::frames().with(1)
            .then(wait::either(
                delay::time().with(delay_time),
                wait::until(changed_other_state).with((mascot, current)),
            ))
            .map(|either| either.is_left())
    })
}

fn transition_state(
    In((mascot, next)): In<(MascotEntity, MascotAction)>,
    mut commands: Commands,
) {
    commands.entity(mascot.0).insert(next);
}
