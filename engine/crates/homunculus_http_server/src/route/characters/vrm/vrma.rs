use crate::extract::character::VrmGuard;
use crate::route::AssetRequest;
use axum::Json;
use axum::extract::{Query, State};
use bevy::animation::RepeatAnimation;
use bevy_vrm1::prelude::PlayVrma;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_api::prelude::{VrmAnimationApi, VrmaInfo, VrmaState};
use homunculus_api::vrm::VrmApi;
use homunculus_core::prelude::AssetId;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use utoipa::ToSchema;

/// List all VRMA animations under a character's VRM.
#[utoipa::path(
    get,
    path = "/vrma",
    tag = "vrm",
    params(("id" = String, Path, description = "Character ID")),
    responses(
        (status = 200, description = "List of VRMA animations", body = Vec<VrmaInfo>),
        (status = 404, description = "Character not found"),
        (status = 422, description = "No VRM attached"),
    ),
)]
pub async fn get(
    State(api): State<VrmAnimationApi>,
    VrmGuard { entity, .. }: VrmGuard,
) -> HttpResult<Vec<VrmaInfo>> {
    api.list_all(entity).await.into_http_result()
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PlayBody {
    /// Asset ID for the VRMA animation to play.
    pub asset: AssetId,
    /// Duration in seconds for the transition of the animation.
    #[serde(rename = "transitionSecs")]
    pub transition_secs: Option<f64>,
    /// Repetition behavior of an animation.
    pub repeat: Option<Repeat>,
    /// If true, the API will wait until the animation finishes.
    #[serde(rename = "waitForCompletion")]
    pub wait_for_completion: Option<bool>,
    /// If true, resets SpringBone velocities to prevent bouncing during animation transitions.
    #[serde(rename = "resetSpringBones")]
    pub reset_spring_bones: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Repeat {
    Forever,
    Never,
    Count { count: u32 },
}

/// Play a VRM animation.
#[utoipa::path(
    post,
    path = "/vrma/play",
    tag = "vrm",
    params(("id" = String, Path, description = "Character ID")),
    request_body = PlayBody,
    responses(
        (status = 200, description = "Animation started"),
        (status = 404, description = "Character or animation not found"),
        (status = 422, description = "No VRM attached"),
    ),
)]
pub async fn play(
    State(vrm_api): State<VrmApi>,
    State(vrma_api): State<VrmAnimationApi>,
    VrmGuard { entity, .. }: VrmGuard,
    Json(body): Json<PlayBody>,
) -> HttpResult {
    let vrma = vrm_api.vrma(entity, body.asset).await?;

    let mut args = PlayVrma {
        vrma,
        transition_duration: Duration::from_secs(0),
        repeat: RepeatAnimation::Never,
        reset_spring_bones: true,
    };
    if let Some(transition_secs) = body.transition_secs {
        args.transition_duration = Duration::from_secs_f64(transition_secs);
    }
    if let Some(repeat) = body.repeat {
        args.repeat = match repeat {
            Repeat::Forever => RepeatAnimation::Forever,
            Repeat::Never => RepeatAnimation::Never,
            Repeat::Count { count } => RepeatAnimation::Count(count),
        };
    }
    vrma_api
        .play(args, body.wait_for_completion.unwrap_or_default())
        .await
        .into_http_result()
}

/// Stop a VRM animation.
#[utoipa::path(
    post,
    path = "/vrma/stop",
    tag = "vrm",
    params(("id" = String, Path, description = "Character ID")),
    request_body = AssetRequest,
    responses(
        (status = 200, description = "Animation stopped"),
        (status = 404, description = "Character or animation not found"),
        (status = 422, description = "No VRM attached"),
    ),
)]
pub async fn stop(
    State(vrm_api): State<VrmApi>,
    State(vrma_api): State<VrmAnimationApi>,
    VrmGuard { entity, .. }: VrmGuard,
    Json(body): Json<AssetRequest>,
) -> HttpResult {
    let vrma = vrm_api.vrma(entity, body.asset).await?;
    vrma_api.stop(vrma).await.into_http_result()
}

/// Get the state of a VRM animation.
#[utoipa::path(
    get,
    path = "/vrma/state",
    tag = "vrm",
    params(
        ("id" = String, Path, description = "Character ID"),
        ("asset" = String, Query, description = "Asset ID of the VRMA animation"),
    ),
    responses(
        (status = 200, description = "Animation state", body = VrmaState),
        (status = 404, description = "Character or animation not found"),
        (status = 422, description = "No VRM attached"),
    ),
)]
pub async fn state(
    State(vrm_api): State<VrmApi>,
    State(vrma_api): State<VrmAnimationApi>,
    VrmGuard { entity, .. }: VrmGuard,
    Query(query): Query<AssetRequest>,
) -> HttpResult<VrmaState> {
    let vrma = vrm_api.vrma(entity, query.asset).await?;
    vrma_api.state(vrma).await.into_http_result()
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SpeedBody {
    /// Asset ID for the VRMA animation.
    pub asset: AssetId,
    /// Playback speed multiplier.
    pub speed: f32,
}

/// Set the playback speed for a VRM animation.
#[utoipa::path(
    put,
    path = "/vrma/speed",
    tag = "vrm",
    params(("id" = String, Path, description = "Character ID")),
    request_body = SpeedBody,
    responses(
        (status = 200, description = "Playback speed updated"),
        (status = 404, description = "Character or animation not found"),
        (status = 422, description = "No VRM attached"),
    ),
)]
pub async fn speed(
    State(vrm_api): State<VrmApi>,
    State(vrma_api): State<VrmAnimationApi>,
    VrmGuard { entity, .. }: VrmGuard,
    Json(body): Json<SpeedBody>,
) -> HttpResult {
    let vrma = vrm_api.vrma(entity, body.asset).await?;
    vrma_api
        .set_speed(vrma, body.speed)
        .await
        .into_http_result()
}

#[cfg(test)]
mod tests {
    use crate::tests::{call, spawn_character_with_vrm, test_app};
    use bevy::prelude::Name;
    use bevy_vrm1::prelude::{Vrma, VrmaAnimationPlayers};
    use homunculus_api::prelude::VrmaInfo;
    use http_body_util::BodyExt;

    #[tokio::test]
    async fn test_get_vrma_list() {
        let (mut app, router) = test_app();
        let vrm_entity = spawn_character_with_vrm(&mut app, "test-char");
        let vrma_entity = app
            .world_mut()
            .spawn((Name::new("idle"), Vrma, VrmaAnimationPlayers(vec![])))
            .id();
        app.world_mut()
            .commands()
            .entity(vrm_entity)
            .add_child(vrma_entity);

        let request =
            axum::http::Request::get("/characters/test-char/vrm/vrma")
                .body(axum::body::Body::empty())
                .unwrap();
        let response = call(&mut app, router, request).await;
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let infos: Vec<VrmaInfo> = serde_json::from_slice(&body).unwrap();
        assert_eq!(infos.len(), 1);
        assert_eq!(infos[0].name, "idle");
        assert_eq!(infos[0].playing, false);
    }
}
