use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use homunculus_api::persona::PersonaApi;
use homunculus_core::prelude::Persona;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::PersonaPath;

/// Attach a VRM model to a persona.
#[utoipa::path(
    post,
    path = "/vrm",
    tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    request_body = AttachVrmBody,
    responses(
        (status = 200, description = "VRM attached, returns updated persona", body = Persona),
        (status = 404, description = "Persona or asset not found"),
        (status = 409, description = "VRM already attached"),
    ),
)]
pub async fn attach(
    State(api): State<PersonaApi>,
    path: PersonaPath,
    Json(body): Json<AttachVrmBody>,
) -> Response {
    match api.attach_vrm(path.persona_id, body.asset_id).await {
        Ok(persona) => (StatusCode::OK, Json(persona)).into_response(),
        Err(e) => e.into_response(),
    }
}

/// Detach the VRM model from a persona.
#[utoipa::path(
    delete,
    path = "/vrm",
    tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    responses(
        (status = 204, description = "VRM detached"),
        (status = 404, description = "Persona not found"),
        (status = 409, description = "No VRM attached"),
    ),
)]
pub async fn detach(State(api): State<PersonaApi>, path: PersonaPath) -> Response {
    match api.detach_vrm(path.persona_id).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => e.into_response(),
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AttachVrmBody {
    /// The asset ID of the VRM model to attach.
    pub asset_id: String,
}
