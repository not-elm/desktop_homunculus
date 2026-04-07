use axum::extract::State;
use homunculus_api::persona::{PersonaApi, PersonaFullSnapshot};
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};

/// Get a full snapshot of all personas including transform and VRM state.
#[utoipa::path(
    get,
    path = "/snapshot",
    tag = "personas",
    responses(
        (status = 200, description = "Full state of all personas", body = Vec<PersonaFullSnapshot>),
    ),
)]
pub async fn snapshot(State(api): State<PersonaApi>) -> HttpResult<Vec<PersonaFullSnapshot>> {
    api.full_snapshot().await.into_http_result()
}
