use crate::error::{ApiError, ApiResult};
use crate::prelude::WebviewApi;
use bevy::prelude::*;
use bevy_cef::prelude::RequestReload;
use bevy_flurx::action::once;
use homunculus_effects::{Entity, Update};

impl WebviewApi {
    /// Reloads the current page in the specified webview.
    pub async fn reload(&self, webview: Entity) -> ApiResult<()> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(reload).with(webview)).await
            })
            .await?
    }
}

fn reload(
    In(webview): In<Entity>,
    mut commands: Commands,
    webviews: Query<Entity, With<bevy_cef::prelude::WebviewSource>>,
) -> ApiResult<()> {
    if webviews.contains(webview) {
        commands.trigger(RequestReload { webview });
        Ok(())
    } else {
        Err(ApiError::WebviewNotFound(webview))
    }
}
