use crate::error::ApiResult;
use crate::prelude::WebviewApi;
use bevy::prelude::*;
use bevy_flurx::action::once;
use bevy_webview_wry::core::Webview;

impl WebviewApi {
    pub async fn is_closed(&self, webview: Entity) -> ApiResult<bool> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(is_closed).with(webview)).await
            })
            .await
    }
}

fn is_closed(In(webview): In<Entity>, webviews: Query<&Webview>) -> bool {
    !webviews.contains(webview)
}
