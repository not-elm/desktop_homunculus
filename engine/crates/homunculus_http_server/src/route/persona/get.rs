use axum::extract::State;
use homunculus_api::persona::PersonaApi;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_core::prelude::Persona;

use super::PersonaPath;

/// List all personas.
#[utoipa::path(
    get,
    path = "/",
    tag = "personas",
    responses(
        (status = 200, description = "List of personas", body = Vec<Persona>),
    ),
)]
pub async fn list(State(api): State<PersonaApi>) -> HttpResult<Vec<Persona>> {
    api.list().await.into_http_result()
}

/// Get a single persona by ID.
#[utoipa::path(
    get,
    path = "/",
    tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    responses(
        (status = 200, description = "Persona details", body = Persona),
        (status = 404, description = "Persona not found"),
    ),
)]
pub async fn get(State(api): State<PersonaApi>, path: PersonaPath) -> HttpResult<Persona> {
    api.get(path.persona_id).await.into_http_result()
}
