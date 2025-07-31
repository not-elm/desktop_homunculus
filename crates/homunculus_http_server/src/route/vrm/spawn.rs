use axum::Json;
use axum::extract::State;
use bevy::prelude::Entity;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_api::vrm::{VrmApi, VrmSpawnArgs};

/// Spawn a VRM model.
///
/// ### Path
///
/// `POST /vrm/spawn`
pub async fn spawn(
    State(api): State<VrmApi>,
    Json(body): Json<VrmSpawnArgs>,
) -> HttpResult<Entity> {
    api.spawn(body).await.into_http_result()
}
