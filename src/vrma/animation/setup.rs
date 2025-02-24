use crate::settings::state::MascotAction;
use crate::vrma::animation::{AnimationPlayerEntities, VrmAnimationGraph};
use bevy::app::{App, Update};
use bevy::hierarchy::{HierarchyQueryExt, Parent};
use bevy::prelude::{Added, AnimationGraphHandle, AnimationPlayer, Commands, Entity, Plugin, Query};

/// At the timing when the spawn of the Vrma's AnimationPlayer is completed,
/// register the AnimationGraph and associate the Player's entity with the root entity.
pub struct VrmaAnimationSetupPlugin;

impl Plugin for VrmaAnimationSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            setup,
        ));
    }
}

fn setup(
    mut commands: Commands,
    mut vrma: Query<(&mut AnimationPlayerEntities, &VrmAnimationGraph, &MascotAction)>,
    players: Query<Entity, Added<AnimationPlayer>>,
    parents: Query<&Parent>,
) {
    for player_entity in players.iter() {
        let mut entity = player_entity;
        loop {
            if let Ok((mut players, animation_graph, state)) = vrma.get_mut(entity) {
                players.push(player_entity);
                commands.entity(player_entity).insert(AnimationGraphHandle(animation_graph.handle.clone()));

                // If default state player loaded, insert the state to mascot entity.
                if state == &MascotAction::default() {
                    let mascot_entity = parents.root_ancestor(entity);
                    commands.entity(mascot_entity).insert(state.clone());
                }
                break;
            }

            if let Ok(parent) = parents.get(entity) {
                entity = parent.get();
            } else {
                break;
            }
        }
    }
}



