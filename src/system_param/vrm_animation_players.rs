use crate::vrma::animation::{AnimationPlayerEntities, VrmAnimationGraph};
use crate::vrma::VrmaEntity;
use bevy::animation::AnimationPlayer;
use bevy::ecs::system::SystemParam;
use bevy::prelude::Query;

#[derive(SystemParam)]
pub struct VrmAnimationPlayers<'w, 's> {
    vrma: Query<'w, 's, (&'static AnimationPlayerEntities, &'static VrmAnimationGraph)>,
    animation_players: Query<'w, 's, &'static mut AnimationPlayer>,
}

impl VrmAnimationPlayers<'_, '_> {
    pub fn play_all(&mut self, vrma: VrmaEntity, is_repeat: bool) {
        let Ok((vrma_players, graph)) = self.vrma.get(vrma.0) else {
            return;
        };

        for player_entity in vrma_players.iter() {
            let Ok(mut player) = self.animation_players.get_mut(*player_entity) else {
                continue;
            };
            player.stop_all();
            for node in &graph.nodes {
                let controller = player.play(*node);
                if is_repeat {
                    controller.repeat();
                }
            }
        }
    }

    pub fn all_finished(&self, vrma: VrmaEntity) -> bool {
        let Ok((vrma_players, _)) = self.vrma.get(vrma.0) else {
            return true;
        };
        vrma_players
            .iter()
            .filter_map(|entity| self.animation_players.get(*entity).ok())
            .all(AnimationPlayer::all_finished)
    }

    pub fn stop_all(&mut self, vrma: VrmaEntity) {
        let Ok((vrma_players, _)) = self.vrma.get(vrma.0) else {
            return;
        };

        for player_entity in vrma_players.iter() {
            let Ok(mut player) = self.animation_players.get_mut(*player_entity) else {
                continue;
            };
            player.stop_all();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::success;
    use crate::system_param::vrm_animation_players::VrmAnimationPlayers;
    use crate::tests::{test_app, TestResult};
    use crate::vrma::animation::{AnimationPlayerEntities, VrmAnimationGraph};
    use crate::vrma::VrmaEntity;
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::{AnimationNodeIndex, AnimationPlayer, Commands, Component, Entity, Query, With};
    use bevy::utils::default;

    #[derive(Component)]
    struct Target;

    #[test]
    fn run_players() -> TestResult {
        let mut app = test_app();
        app.world_mut().run_system_once(|mut commands: Commands| {
            let p1 = commands.spawn((
                Target,
                AnimationPlayer::default(),
            )).id();
            commands.spawn(AnimationPlayer::default());

            commands.spawn((
                AnimationPlayerEntities(vec![p1]),
                VrmAnimationGraph {
                    nodes: vec![AnimationNodeIndex::new(1)],
                    ..default()
                },
            ));
        })?;
        app.update();

        app.world_mut().run_system_once(|mut players: VrmAnimationPlayers, entity: Query<Entity, With<AnimationPlayerEntities>>| {
            players.play_all(VrmaEntity(entity.single()), false);
        })?;
        app.update();

        app.world_mut().run_system_once(|target: Query<&AnimationPlayer, With<Target>>| {
            assert!(!target.single().all_finished());
        })?;
        success!()
    }

    #[test]
    fn stop_all() -> TestResult {
        let mut app = test_app();
        app.world_mut().run_system_once(|mut commands: Commands| {
            let p1 = commands.spawn((
                Target,
                AnimationPlayer::default(),
            )).id();

            commands.spawn((
                AnimationPlayerEntities(vec![p1]),
                VrmAnimationGraph {
                    nodes: vec![AnimationNodeIndex::new(1)],
                    ..default()
                },
            ));
        })?;
        app.update();

        app.world_mut().run_system_once(|mut players: VrmAnimationPlayers, entity: Query<Entity, With<AnimationPlayerEntities>>| {
            players.play_all(VrmaEntity(entity.single()), false);
        })?;
        app.update();

        app.world_mut().run_system_once(|mut players: VrmAnimationPlayers, entity: Query<Entity, With<AnimationPlayerEntities>>| {
            players.stop_all(VrmaEntity(entity.single()));
        })?;
        app.update();

        app.world_mut().run_system_once(|target: Query<&AnimationPlayer, With<Target>>| {
            assert!(target.single().all_finished());
        })?;
        success!()
    }
}