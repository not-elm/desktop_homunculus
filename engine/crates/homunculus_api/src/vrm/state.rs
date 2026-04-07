use crate::error::ApiResult;
use crate::vrm::VrmApi;
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use homunculus_core::prelude::PersonaState;

impl VrmApi {
    /// Retrieves the state of a VRM entity.
    pub async fn state(&self, vrm_entity: Entity) -> ApiResult<PersonaState> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(get_vrm_state).with(vrm_entity))
                    .await
                    .expect("Failed to get VRM state")
            })
            .await
    }

    /// Sets the state of a VRM entity.
    pub async fn set_state(&self, vrm_entity: Entity, state: PersonaState) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(put_vrm_state).with((vrm_entity, state)))
                    .await;
            })
            .await
    }
}

fn get_vrm_state(In(entity): In<Entity>, vrm_states: Query<&PersonaState>) -> Option<PersonaState> {
    vrm_states.get(entity).ok().cloned()
}

fn put_vrm_state(In((entity, state)): In<(Entity, PersonaState)>, mut commands: Commands) {
    commands.entity(entity).try_insert(state);
}
