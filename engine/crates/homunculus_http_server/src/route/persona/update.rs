use axum::Json;
use axum::extract::State;
use homunculus_api::persona::{PatchPersona, PersonaApi, PersonaSnapshot};
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};

use super::PersonaPath;

/// Partially update a persona.
#[utoipa::path(
    patch,
    path = "/",
    tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    request_body = PatchPersona,
    responses(
        (status = 200, description = "Updated persona", body = PersonaSnapshot),
        (status = 404, description = "Persona not found"),
    ),
)]
pub async fn patch(
    State(api): State<PersonaApi>,
    path: PersonaPath,
    Json(body): Json<PatchPersona>,
) -> HttpResult<PersonaSnapshot> {
    api.patch(path.persona_id, body).await.into_http_result()
}
