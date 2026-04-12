use axum::Json;
use axum::extract::State;
use homunculus_api::persona::PersonaApi;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_core::prelude::PersonaState;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::PersonaPath;

/// Get the state of a persona.
#[utoipa::path(
    get,
    path = "/state",
    tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    responses(
        (status = 200, description = "Persona state", body = StateBody),
        (status = 404, description = "Persona not found"),
    ),
)]
pub async fn get_persona_state(State(api): State<PersonaApi>, path: PersonaPath) -> HttpResult<StateBody> {
    let state = api.state(path.persona_id).await?;
    Ok(Json(StateBody { state }))
}

/// Set the state of a persona.
#[utoipa::path(
    put,
    path = "/state",
    tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    request_body = StateBody,
    responses(
        (status = 200, description = "Persona state updated"),
        (status = 404, description = "Persona not found"),
    ),
)]
pub async fn set_persona_state(
    State(api): State<PersonaApi>,
    path: PersonaPath,
    Json(body): Json<StateBody>,
) -> HttpResult {
    api.set_state(path.persona_id, body.state)
        .await
        .into_http_result()
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct StateBody {
    #[schema(value_type = Object)]
    pub state: PersonaState,
}
