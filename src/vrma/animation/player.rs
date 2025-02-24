use crate::mascot::MascotEntity;
use crate::settings::preferences::action::{ActionPreferences, TransitionMode};
use crate::settings::state::MascotAction;
use crate::system_param::vrm_animation_players::VrmAnimationPlayers;
use crate::vrma::animation::auto_transition::ScheduleAutoTransition;
use crate::vrma::animation::schedule_transition::ScheduleTransition;
use crate::vrma::retarget::CurrentRetargeting;
use crate::vrma::{RetargetSource, VrmaEntity};
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Children, Commands, Entity, Event, EventReader, EventWriter, Query, Res};

#[derive(Event, Debug)]
pub struct RequestPlayVrma {
    pub mascot: MascotEntity,
    pub vrma: VrmaEntity,
    pub action: MascotAction,
}

#[derive(Event, Debug)]
pub struct RequestStopVrma {
    pub vrma: VrmaEntity,
}

pub struct VrmaAnimationPlayPlugin;

impl Plugin for VrmaAnimationPlayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<RequestPlayVrma>()
            .add_event::<RequestStopVrma>()
            .add_systems(Update, (
                request_play,
                request_stop,
            ));
    }
}

fn request_play(
    mut commands: Commands,
    mut schedule_transition: EventWriter<ScheduleTransition>,
    mut auto_transition: EventWriter<ScheduleAutoTransition>,
    mut er: EventReader<RequestPlayVrma>,
    mut players: VrmAnimationPlayers,
    preferences: Res<ActionPreferences>,
    entities: Query<(Option<&Children>, Option<&RetargetSource>)>,
    states: Query<&MascotAction>,
) {
    for event in er.read() {
        let property = preferences.properties(&event.action);
        players.play_all(event.vrma, property.is_repeat_animation);

        match property.transition {
            TransitionMode::Manual { next } => {
                if let Ok(current) = states.get(event.vrma.0) {
                    schedule_transition.send(ScheduleTransition {
                        mascot: event.mascot,
                        vrma: event.vrma,
                        current: current.clone(),
                        next,
                    });
                }
            }
            TransitionMode::Auto { min_secs, max_secs } => {
                auto_transition.send(ScheduleAutoTransition {
                    mascot: event.mascot,
                    min_secs,
                    max_secs,
                });
            }
            TransitionMode::None => {}
        }

        foreach_children(
            &mut commands,
            event.vrma.0,
            &entities,
            &|commands, entity, retargeting_marker| {
                if retargeting_marker.is_some() {
                    commands.entity(entity).insert(CurrentRetargeting);
                }
            },
        );
    }
}

fn request_stop(
    mut commands: Commands,
    mut er: EventReader<RequestStopVrma>,
    mut players: VrmAnimationPlayers,
    entities: Query<(Option<&Children>, Option<&RetargetSource>)>,
) {
    for event in er.read() {
        players.stop_all(event.vrma);
        foreach_children(
            &mut commands,
            event.vrma.0,
            &entities,
            &|commands, entity, retargeting_marker| {
                if retargeting_marker.is_some() {
                    commands.entity(entity).remove::<CurrentRetargeting>();
                }
            },
        )
    }
}

fn foreach_children(
    commands: &mut Commands,
    entity: Entity,
    entities: &Query<(Option<&Children>, Option<&RetargetSource>)>,
    f: &impl Fn(&mut Commands, Entity, Option<&RetargetSource>),
) {
    let Ok((children, bone_to)) = entities.get(entity) else {
        return;
    };
    f(commands, entity, bone_to);
    if let Some(children) = children {
        for child in children.iter() {
            foreach_children(commands, *child, entities, f);
        }
    }
}

