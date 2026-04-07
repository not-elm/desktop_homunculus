use axum::Json;
use axum::extract::State;
use homunculus_api::persona::{PatchPersona, PersonaApi};
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_core::prelude::Persona;

use super::PersonaPath;

/// Partially update a persona.
#[utoipa::path(
    patch,
    path = "/",
    tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    request_body = PatchPersona,
    responses(
        (status = 200, description = "Updated persona", body = Persona),
        (status = 404, description = "Persona not found"),
    ),
)]
pub async fn patch(
    State(api): State<PersonaApi>,
    path: PersonaPath,
    Json(body): Json<PatchPersona>,
) -> HttpResult<Persona> {
    api.patch(path.persona_id, body).await.into_http_result()
}
