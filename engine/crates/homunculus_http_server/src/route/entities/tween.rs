use crate::extract::EntityId;
use axum::{Json, extract::State};
use bevy::prelude::*;
use homunculus_api::entities::{
    EasingFunction, EntitiesApi, TweenPositionArgs, TweenRotationArgs, TweenScaleArgs,
};
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TweenPositionRequest {
    pub target: [f32; 3],
    pub duration_ms: u64,
    #[serde(default)]
    pub easing: EasingFunction,
    #[serde(default)]
    pub wait: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TweenRotationRequest {
    pub target: [f32; 4], // Quaternion: [x, y, z, w]
    pub duration_ms: u64,
    #[serde(default)]
    pub easing: EasingFunction,
    #[serde(default)]
    pub wait: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TweenScaleRequest {
    pub target: [f32; 3],
    pub duration_ms: u64,
    #[serde(default)]
    pub easing: EasingFunction,
    #[serde(default)]
    pub wait: bool,
}

/// Tween an entity's position to a target value.
#[utoipa::path(
    post,
    path = "/tween/position",
    tag = "entities",
    params(("entity" = String, Path, description = "Entity ID")),
    request_body = TweenPositionRequest,
    responses(
        (status = 200, description = "Position tween started"),
        (status = 400, description = "Invalid duration"),
        (status = 404, description = "Entity not found"),
    ),
)]
pub async fn tween_position(
    State(api): State<EntitiesApi>,
    EntityId(entity): EntityId,
    Json(body): Json<TweenPositionRequest>,
) -> HttpResult {
    let args = TweenPositionArgs {
        target: Vec3::from_array(body.target),
        duration_ms: body.duration_ms,
        easing: body.easing,
        wait: body.wait,
    };

    api.tween_position(entity, args).await.into_http_result()
}

/// Tween an entity's rotation to a target value.
#[utoipa::path(
    post,
    path = "/tween/rotation",
    tag = "entities",
    params(("entity" = String, Path, description = "Entity ID")),
    request_body = TweenRotationRequest,
    responses(
        (status = 200, description = "Rotation tween started"),
        (status = 400, description = "Invalid duration"),
        (status = 404, description = "Entity not found"),
    ),
)]
pub async fn tween_rotation(
    State(api): State<EntitiesApi>,
    EntityId(entity): EntityId,
    Json(body): Json<TweenRotationRequest>,
) -> HttpResult {
    let args = TweenRotationArgs {
        target: Quat::from_array(body.target),
        duration_ms: body.duration_ms,
        easing: body.easing,
        wait: body.wait,
    };

    api.tween_rotation(entity, args).await.into_http_result()
}

/// Tween an entity's scale to a target value.
#[utoipa::path(
    post,
    path = "/tween/scale",
    tag = "entities",
    params(("entity" = String, Path, description = "Entity ID")),
    request_body = TweenScaleRequest,
    responses(
        (status = 200, description = "Scale tween started"),
        (status = 400, description = "Invalid duration"),
        (status = 404, description = "Entity not found"),
    ),
)]
pub async fn tween_scale(
    State(api): State<EntitiesApi>,
    EntityId(entity): EntityId,
    Json(body): Json<TweenScaleRequest>,
) -> HttpResult {
    let args = TweenScaleArgs {
        target: Vec3::from_array(body.target),
        duration_ms: body.duration_ms,
        easing: body.easing,
        wait: body.wait,
    };

    api.tween_scale(entity, args).await.into_http_result()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{call, call_any_status, test_app};
    use axum::http::{Request, StatusCode};
    use serde_json::json;

    #[tokio::test]
    async fn tween_position_success() {
        let (mut app, router) = test_app();
        let entity = app.world_mut().spawn(Transform::default()).id();
        app.update();

        let request = Request::post(format!("/entities/{}/tween/position", entity.to_bits()))
            .header("content-type", "application/json")
            .body(
                serde_json::to_string(&json!({
                    "target": [100.0, 50.0, 0.0],
                    "durationMs": 1000,
                    "easing": "linear",
                    "wait": false,
                }))
                .unwrap()
                .into(),
            )
            .unwrap();

        let response = call(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn tween_rotation_success() {
        let (mut app, router) = test_app();
        let entity = app.world_mut().spawn(Transform::default()).id();
        app.update();

        let request = Request::post(format!("/entities/{}/tween/rotation", entity.to_bits()))
            .header("content-type", "application/json")
            .body(
                serde_json::to_string(&json!({
                    "target": [0.0, 0.0, 0.7071, 0.7071],  // 90 degrees around Z
                    "durationMs": 500,
                    "easing": "quadraticInOut",
                    "wait": false,
                }))
                .unwrap()
                .into(),
            )
            .unwrap();

        let response = call(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn tween_scale_success() {
        let (mut app, router) = test_app();
        let entity = app.world_mut().spawn(Transform::default()).id();
        app.update();

        let request = Request::post(format!("/entities/{}/tween/scale", entity.to_bits()))
            .header("content-type", "application/json")
            .body(
                serde_json::to_string(&json!({
                    "target": [2.0, 2.0, 2.0],
                    "durationMs": 800,
                    "easing": "elasticOut",
                    "wait": false,
                }))
                .unwrap()
                .into(),
            )
            .unwrap();

        let response = call(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn tween_entity_not_found() {
        let (mut app, router) = test_app();
        let invalid_entity = Entity::from_bits(99999);

        let request = Request::post(format!(
            "/entities/{}/tween/position",
            invalid_entity.to_bits()
        ))
        .header("content-type", "application/json")
        .body(
            serde_json::to_string(&json!({
                "target": [100.0, 50.0, 0.0],
                "durationMs": 1000,
            }))
            .unwrap()
            .into(),
        )
        .unwrap();

        let response = call_any_status(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn tween_zero_duration_error() {
        let (mut app, router) = test_app();
        let entity = app.world_mut().spawn(Transform::default()).id();
        app.update();

        let request = Request::post(format!("/entities/{}/tween/position", entity.to_bits()))
            .header("content-type", "application/json")
            .body(
                serde_json::to_string(&json!({
                    "target": [100.0, 50.0, 0.0],
                    "durationMs": 0,  // Invalid: zero duration
                }))
                .unwrap()
                .into(),
            )
            .unwrap();

        let response = call_any_status(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn tween_with_wait() {
        let (mut app, router) = test_app();
        let entity = app.world_mut().spawn(Transform::default()).id();
        app.update();

        let request = Request::post(format!("/entities/{}/tween/position", entity.to_bits()))
            .header("content-type", "application/json")
            .body(
                serde_json::to_string(&json!({
                    "target": [10.0, 10.0, 0.0],
                    "durationMs": 100,  // Short duration for test
                    "wait": true,
                }))
                .unwrap()
                .into(),
            )
            .unwrap();

        let response = call(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::OK);
        // Note: Full animation testing requires time simulation
    }

    #[tokio::test]
    async fn tween_default_easing_and_wait() {
        let (mut app, router) = test_app();
        let entity = app.world_mut().spawn(Transform::default()).id();
        app.update();

        let request = Request::post(format!("/entities/{}/tween/position", entity.to_bits()))
            .header("content-type", "application/json")
            .body(
                serde_json::to_string(&json!({
                    "target": [100.0, 50.0, 0.0],
                    "durationMs": 1000,
                    // Omit easing and wait to test defaults
                }))
                .unwrap()
                .into(),
            )
            .unwrap();

        let response = call(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::OK);
    }
}
