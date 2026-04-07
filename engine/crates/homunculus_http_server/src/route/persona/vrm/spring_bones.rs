use axum::Json;
use axum::extract::{Path, State};
use bevy::prelude::Entity;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_api::vrm::{SpringBoneChainsResponse, SpringBonePropsUpdate, VrmApi};

use crate::route::persona::PersonaPath;

/// List all spring bone chains for a persona's VRM.
#[utoipa::path(
    get,
    path = "/vrm/spring-bones",
    tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    responses(
        (status = 200, description = "Spring bone chains", body = SpringBoneChainsResponse),
        (status = 404, description = "Persona or VRM not found"),
    ),
)]
pub async fn list_spring_bones(
    State(api): State<VrmApi>,
    path: PersonaPath,
) -> HttpResult<SpringBoneChainsResponse> {
    api.list_spring_bones(path.entity).await.into_http_result()
}

/// Update properties of a spring bone chain.
#[utoipa::path(
    patch,
    path = "/vrm/spring-bones/{chain_id}",
    tag = "personas",
    params(
        ("id" = String, Path, description = "Persona ID"),
        ("chain_id" = String, Path, description = "Spring bone chain entity ID"),
    ),
    request_body = SpringBonePropsUpdate,
    responses(
        (status = 200, description = "Spring bone properties updated"),
        (status = 404, description = "Persona, VRM, or chain not found"),
    ),
)]
pub async fn patch_spring_bones(
    State(api): State<VrmApi>,
    path: PersonaPath,
    Path((_, chain_id)): Path<(String, Entity)>,
    Json(body): Json<SpringBonePropsUpdate>,
) -> HttpResult {
    api.set_spring_bone_props(path.entity, chain_id, body)
        .await
        .into_http_result()
}
