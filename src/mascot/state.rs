use crate::mascot::{Mascot, MascotEntity};
use crate::settings::state::MascotAction;
use crate::vrma::animation::player::{RequestPlayVrma, RequestStopVrma};
use crate::vrma::VrmaEntity;
use bevy::app::{App, Update};
use bevy::prelude::{Changed, Children, Entity, EventWriter, Plugin, Query, With, Without};

pub struct MascotStatePlugin;

impl Plugin for MascotStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, transition_actions);
    }
}

fn transition_actions(
    mut request_play: EventWriter<RequestPlayVrma>,
    mut request_stop: EventWriter<RequestStopVrma>,
    mascots: Query<(Entity, &MascotAction, &Children), (Changed<MascotAction>, With<Mascot>)>,
    vrma: Query<(Entity, &MascotAction), Without<Mascot>>,
) {
    for (mascot_entity, nex_action, children) in mascots.iter() {
        for (vrma_entity, action) in children
            .iter()
            .filter_map(|child| vrma.get(*child).ok())
        {
            if nex_action == action {
                request_play.send(RequestPlayVrma {
                    vrma: VrmaEntity(vrma_entity),
                    mascot: MascotEntity(mascot_entity),
                    action: nex_action.clone(),
                });
            } else {
                request_stop.send(RequestStopVrma { vrma: VrmaEntity(vrma_entity) });
            }
        }
    }
}
