use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use homunculus_api::persona::PersonaApi;

use super::PersonaPath;

/// Delete a persona.
#[utoipa::path(
    delete,
    path = "/",
    tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    responses(
        (status = 204, description = "Persona deleted"),
        (status = 404, description = "Persona not found"),
    ),
)]
pub async fn delete(
    axum::extract::State(api): axum::extract::State<PersonaApi>,
    path: PersonaPath,
) -> Response {
    match api.delete(path.persona_id).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => e.into_response(),
    }
}
