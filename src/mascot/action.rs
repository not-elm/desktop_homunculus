use crate::mascot::{Mascot, MascotEntity};
use crate::settings::preferences::action::{ActionName, ActionPreferences, ExecuteMascotAction};
use bevy::app::{App, Update};
use bevy::prelude::{Changed, Entity, ParallelCommands, Plugin, Query, Res, With};
use bevy_flurx::prelude::Reactor;


pub struct MascotActionPlugin;

impl Plugin for MascotActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, transition_actions);
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
            let action = property.action;
            commands.spawn(Reactor::schedule(move |task| async move {
                action.execute(mascot, &task).await;
            }));
        });
    });
}


// fn transition_actions(
//     mut request_play: EventWriter<RequestPlayVrma>,
//     mut request_stop: EventWriter<RequestStopVrma>,
//     mascots: Query<(Entity, &ActionProperties, &Children), (Changed<ActionProperties>, With<Mascot>)>,
//     vrma: Query<(Entity, &ActionProperties), Without<Mascot>>,
// ) {
//     for (mascot_entity, nex_action, children) in mascots.iter() {
//         for (vrma_entity, action) in children
//             .iter()
//             .filter_map(|child| vrma.get(*child).ok())
//         {
//             if nex_action == action {
//                 request_play.send(RequestPlayVrma {
//                     vrma: VrmaEntity(vrma_entity),
//                     mascot: MascotEntity(mascot_entity),
//                     action: nex_action.clone(),
//                 });
//             } else {
//                 request_stop.send(RequestStopVrma { vrma: VrmaEntity(vrma_entity) });
//             }
//         }
//     }
// }


#[cfg(test)]
mod tests {
    use crate::mascot::action::transition_actions;
    use crate::mascot::Mascot;
    use crate::settings::preferences::action::scale::ScaleAction;
    use crate::settings::preferences::action::{ActionName, ActionPreferences, ActionProperties, ActionTags, MascotAction};
    use crate::tests::{test_app, TestResult};
    use bevy::app::Update;
    use bevy::prelude::{Commands, IntoSystemConfigs};

    #[test]
    fn test_transition_actions() -> TestResult {
        let mut app = test_app();
        let mut preference = ActionPreferences::default();
        preference.register_if_not_exists(ActionName::drop(), ActionProperties {
            tags: vec!["drag"].into(),
            action: MascotAction::Scale(ScaleAction::default()),
        });
        app.add_systems(Update, (
            |mut commands: Commands| {
                commands.spawn((
                    Mascot,
                    ActionName::drop(),
                ));
            },
            transition_actions,
        ).chain());
        app.insert_resource(preference);
        app.update();

        let tags = app.world_mut().query::<&ActionTags>().single(app.world());
        assert_eq!(tags, &ActionTags::from(vec!["drag"]));
        Ok(())
    }
}
