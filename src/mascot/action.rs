use crate::mascot::{Mascot, MascotEntity};
use crate::settings::preferences::action::ActionProperties;
use bevy::app::{App, Update};
use bevy::prelude::{Changed, Entity, ParallelCommands, Plugin, Query, With};
use bevy_flurx::prelude::Reactor;


pub struct MascotActionPlugin;

impl Plugin for MascotActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, transition_actions);
    }
}

fn transition_actions(
    par_commands: ParallelCommands,
    mascots: Query<(Entity, &ActionProperties), (Changed<ActionProperties>, With<Mascot>)>,
) {
    mascots.par_iter().for_each(|(entity, property)| {
        let property = property.clone();
        let mascot = MascotEntity(entity);
        par_commands.command_scope(move |mut commands| {
            commands.spawn(Reactor::schedule(move |task| async move {
                property.execute(mascot, &task).await;
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
