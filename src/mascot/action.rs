mod animation;

use crate::mascot::{Mascot, MascotEntity};
use crate::settings::preferences::action::{ActionName, ActionPreferences};
use bevy::app::{App, Update};
use bevy::prelude::{
    Changed, Commands, Entity, Event, ParallelCommands, Plugin, Query, Res, Trigger, With,
};
use bevy_flurx::action::once;
use bevy_flurx::prelude::{ActionSeed, Reactor};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Event)]
pub struct RequestAction {
    pub mascot: MascotEntity,
    pub id: String,
    pub params: String,
}

#[derive(Event, Default)]
struct ActionDone;

pub struct MascotActionPlugin;

impl Plugin for MascotActionPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_event::<ActionDone>()
            .add_systems(Update, transition_actions);
    }
}

fn transition_actions(
    par_commands: ParallelCommands,
    preference: Res<ActionPreferences>,
    mascots: Query<(Entity, &ActionName), (Changed<ActionName>, With<Mascot>)>,
) {
    mascots.par_iter().for_each(|(entity, action_name)| {
        let Some(property) = preference.get(action_name).cloned() else {
            return;
        };
        let mascot = MascotEntity(entity);
        par_commands.command_scope(move |mut commands| {
            commands.entity(mascot.0).insert(property.tags);
            commands.trigger(RequestAction {
                mascot: MascotEntity(entity),
                id: property.action_id,
                params: property.params,
            });
        });
    });
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
                if trigger.id != action_id {
                    return;
                }
                let mascot = trigger.mascot;
                let event = serde_json::from_str::<Params>(&trigger.params).unwrap();
                commands.spawn(Reactor::schedule(move |task| async move {
                    task.will(Update, action(mascot, event)).await;
                    task.will(
                        Update,
                        once::run(move |mut commands: Commands| {
                            commands.entity(mascot.0).trigger(ActionDone);
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
    use crate::actions;
    use crate::mascot::action::{
        transition_actions, ActionDone, MascotActionExt, MascotActionPlugin,
    };
    use crate::mascot::Mascot;
    use crate::settings::preferences::action::{
        ActionName, ActionPreferences, ActionProperties, ActionTags,
    };
    use crate::tests::{test_app, TestResult};
    use bevy::app::Update;
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::{Commands, Component, IntoSystemConfigs, Trigger};
    use bevy::utils::default;
    use bevy_flurx::action::once;

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

    #[test]
    fn await_for_emit_action_done() -> TestResult {
        #[derive(Component)]
        struct Success;

        let mut app = test_app();
        app.add_plugins(MascotActionPlugin);
        app.add_mascot_action::<()>("test", |_, _| once::no_op());
        app.insert_resource(ActionPreferences(actions! {
            "test": ActionProperties{
                action_id: "test".to_string(),
                params: serde_json::to_string(&()).unwrap(),
                ..default()
            },
        }));
        app.world_mut().run_system_once(|mut commands: Commands| {
            commands.spawn((Mascot, ActionName::from("test"))).observe(
                |_: Trigger<ActionDone>, mut commands: Commands| {
                    commands.spawn(Success);
                },
            );
        })?;
        app.update();
        app.update();
        app.update();
        assert!(app
            .world_mut()
            .query::<&Success>()
            .get_single(app.world())
            .is_ok());

        Ok(())
    }
}
