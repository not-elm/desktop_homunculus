pub mod animation;
mod auto_transition;
mod mascot_action;
mod range_timer;
pub mod transition;
pub mod wait_animation;

use crate::mascot::action::animation::AnimationActionPlugin;
use crate::mascot::action::auto_transition::AutoTransitionPlugin;
pub use crate::mascot::action::mascot_action::MascotAction;
use crate::mascot::action::range_timer::RangeTimerActionPlugin;
use crate::mascot::action::transition::TransitionActionPlugin;
use crate::mascot::action::wait_animation::WaitAnimationPlugin;
use crate::mascot::{Mascot, MascotEntity};
use crate::settings::preferences::action::{ActionName, ActionPreferences};
use bevy::app::{App, Update};
use bevy::prelude::{
    Changed, Commands, Entity, Event, EventReader, In, ParallelCommands, Plugin, Query, Res,
    Trigger, With,
};
use bevy_flurx::action::{once, wait};
use bevy_flurx::prelude::*;
use serde::de::DeserializeOwned;
use serde::Serialize;

#[derive(Event)]
pub struct RequestAction {
    pub mascot: MascotEntity,
    pub params: MascotAction,
}

#[derive(Event)]
struct ActionDone {
    mascot: MascotEntity,
}

pub struct MascotActionPlugin;

impl Plugin for MascotActionPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_event::<ActionDone>()
            .add_plugins((
                AnimationActionPlugin,
                TransitionActionPlugin,
                WaitAnimationPlugin,
                AutoTransitionPlugin,
                RangeTimerActionPlugin,
            ))
            .add_systems(Update, transition_actions);
    }
}

fn transition_actions(
    par_commands: ParallelCommands,
    preference: Res<ActionPreferences>,
    mascots: Query<(Entity, &ActionName), (Changed<ActionName>, With<Mascot>)>,
) {
    mascots.par_iter().for_each(|(entity, action_name)| {
        let Some(properties) = preference.get(action_name).cloned() else {
            return;
        };
        let mascot = MascotEntity(entity);
        let action_name = action_name.clone();
        par_commands.command_scope(move |mut commands| {
            commands.entity(mascot.0).insert(properties.tags);
            commands.spawn(Reactor::schedule(move |task| async move {
                for action in properties.actions {
                    task.will(Update, delay::frames().with(1)).await;
                    let canceled = task
                        .will(
                            Update,
                            wait::either(
                                wait::until(detect_change_action_name)
                                    .with((mascot, action_name.clone())),
                                once::run(emit_action)
                                    .with((mascot, action))
                                    .then(wait::until(action_done).with(mascot)),
                            ),
                        )
                        .await
                        .is_left();
                    if canceled {
                        break;
                    }
                }
            }));
        });
    });
}

fn detect_change_action_name(
    In((mascot, current_name)): In<(MascotEntity, ActionName)>,
    mascots: Query<&ActionName>,
) -> bool {
    mascots.get(mascot.0).is_ok_and(|n| n != &current_name)
}

fn emit_action(
    In((mascot, params)): In<(MascotEntity, MascotAction)>,
    mut commands: Commands,
) {
    commands.trigger(RequestAction { mascot, params });
}

fn action_done(
    In(mascot): In<MascotEntity>,
    mut er: EventReader<ActionDone>,
) -> bool {
    er.read().any(|e| e.mascot == mascot)
}

pub trait MascotActionExt {
    fn add_mascot_action<Params>(
        &mut self,
        action_id: &'static str,
        action: fn(MascotEntity, Params) -> ActionSeed,
    ) where
        Params: Serialize + DeserializeOwned + Send + Sync + 'static;
}

impl MascotActionExt for App {
    fn add_mascot_action<Params>(
        &mut self,
        action_id: &'static str,
        action: fn(MascotEntity, Params) -> ActionSeed,
    ) where
        Params: Serialize + DeserializeOwned + Send + Sync + 'static,
    {
        self.add_observer(
            move |trigger: Trigger<RequestAction>, mut commands: Commands| {
                if trigger.params.id != action_id {
                    return;
                }
                let mascot = trigger.mascot;
                let event = serde_json::from_str::<Params>(&trigger.params.params).unwrap();
                commands.spawn(Reactor::schedule(move |task| async move {
                    task.will(Update, action(mascot, event)).await;
                    task.will(
                        Update,
                        once::run(move |mut commands: Commands| {
                            commands.send_event(ActionDone { mascot });
                        }),
                    )
                    .await;
                }));
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::mascot::action::transition_actions;
    use crate::mascot::Mascot;
    use crate::settings::preferences::action::{
        ActionName, ActionPreferences, ActionProperties, ActionTags,
    };
    use crate::tests::{test_app, TestResult};
    use bevy::app::Update;
    use bevy::prelude::{Commands, IntoSystemConfigs};
    use bevy::utils::default;

    #[test]
    fn test_transition_actions() -> TestResult {
        let mut app = test_app();
        let mut preference = ActionPreferences::default();
        preference.register_if_not_exists(
            ActionName::drop(),
            ActionProperties {
                tags: vec!["drag"].into(),
                ..default()
            },
        );
        app.add_systems(
            Update,
            (
                |mut commands: Commands| {
                    commands.spawn((Mascot, ActionName::drop()));
                },
                transition_actions,
            )
                .chain(),
        );
        app.insert_resource(preference);
        app.update();

        let tags = app.world_mut().query::<&ActionTags>().single(app.world());
        assert_eq!(tags, &ActionTags::from(vec!["drag"]));
        Ok(())
    }
}
