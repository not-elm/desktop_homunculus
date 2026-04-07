use axum::extract::State;
use homunculus_api::persona::PersonaApi;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_core::prelude::Persona;

/// Get a snapshot of all personas.
#[utoipa::path(
    get,
    path = "/snapshot",
    tag = "personas",
    responses(
        (status = 200, description = "All persona data", body = Vec<Persona>),
    ),
)]
pub async fn snapshot(State(api): State<PersonaApi>) -> HttpResult<Vec<Persona>> {
    api.list().await.into_http_result()
}
