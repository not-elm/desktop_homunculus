use crate::error::ApiResult;
use crate::prelude::WebviewApi;
use bevy_flurx::prelude::*;
use homunculus_effects::{Commands, Entity, In, Update};

impl WebviewApi {
    /// Closes a webview.
    pub async fn close(&self, webview: Entity) -> ApiResult<()> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(close).with(webview)).await;
            })
            .await
    }
}

fn close(In(entity): In<Entity>, mut commands: Commands) {
    commands.entity(entity).try_despawn();
}
