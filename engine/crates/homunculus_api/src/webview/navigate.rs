use crate::error::{ApiError, ApiResult};
use crate::prelude::WebviewApi;
use crate::webview::open::{OriginalWebviewSource, source_to_webview_source};
use bevy::prelude::*;
use bevy_cef::prelude::{RequestGoBack, RequestGoForward};
use bevy_cef_core::prelude::Browsers;
use bevy_flurx::action::once;
use homunculus_core::prelude::{AssetResolver, NavigationState, WebviewSource};
use homunculus_effects::{Entity, Update};

impl WebviewApi {
    /// Navigates a webview back in history.
    pub async fn go_back(&self, webview: Entity) -> ApiResult<()> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(go_back).with(webview)).await
            })
            .await?
    }

    /// Navigates a webview forward in history.
    pub async fn go_forward(&self, webview: Entity) -> ApiResult<()> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(go_forward).with(webview)).await
            })
            .await?
    }

    /// Returns the navigation history state of a webview.
    pub async fn navigation_state(&self, webview: Entity) -> ApiResult<NavigationState> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(get_navigation_state).with(webview))
                    .await
            })
            .await?
    }

    /// Navigates a webview to a new source (URL or inline HTML).
    pub async fn navigate(&self, webview: Entity, source: WebviewSource) -> ApiResult<()> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(navigate).with((webview, source)))
                    .await
            })
            .await?
    }
}

fn go_back(
    In(webview): In<Entity>,
    mut commands: Commands,
    webviews: Query<Entity, With<bevy_cef::prelude::WebviewSource>>,
) -> ApiResult<()> {
    if webviews.contains(webview) {
        commands.trigger(RequestGoBack { webview });
        Ok(())
    } else {
        Err(ApiError::WebviewNotFound(webview))
    }
}

fn go_forward(
    In(webview): In<Entity>,
    mut commands: Commands,
    webviews: Query<Entity, With<bevy_cef::prelude::WebviewSource>>,
) -> ApiResult<()> {
    if webviews.contains(webview) {
        commands.trigger(RequestGoForward { webview });
        Ok(())
    } else {
        Err(ApiError::WebviewNotFound(webview))
    }
}

fn navigate(
    In((webview, source)): In<(Entity, WebviewSource)>,
    mut commands: Commands,
    webviews: Query<Entity, With<bevy_cef::prelude::WebviewSource>>,
    asset_resolver: AssetResolver,
) -> ApiResult<()> {
    if !webviews.contains(webview) {
        return Err(ApiError::WebviewNotFound(webview));
    }
    let cef_source = source_to_webview_source(&source, &asset_resolver)?;
    commands
        .entity(webview)
        .try_insert((cef_source, OriginalWebviewSource(source)));
    Ok(())
}

fn get_navigation_state(
    In(webview): In<Entity>,
    browsers: NonSend<Browsers>,
    webviews: Query<Entity, With<bevy_cef::prelude::WebviewSource>>,
) -> ApiResult<NavigationState> {
    if !webviews.contains(webview) {
        return Err(ApiError::WebviewNotFound(webview));
    }
    Ok(NavigationState {
        can_go_back: browsers.can_go_back(&webview),
        can_go_forward: browsers.can_go_forward(&webview),
    })
}
