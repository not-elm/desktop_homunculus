use crate::extract::EntityId;
use axum::Json;
use axum::extract::State;
use bevy::prelude::*;
use homunculus_api::prelude::EntitiesApi;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_api::vrm::OptionalTransform;

/// Get the transform of a VRM model or its bone.
///
/// ### Path
///
/// `GET /entities/:entity_id/transform`
pub async fn get(
    State(api): State<EntitiesApi>,
    EntityId(entity): EntityId,
) -> HttpResult<Transform> {
    api.transform(entity).await.into_http_result()
}

/// Set the transform of a VRM model.
///
/// ### Path
///
/// `PUT /entities/:entity_id/transform`
pub async fn put(
    State(api): State<EntitiesApi>,
    EntityId(entity): EntityId,
    Json(body): Json<OptionalTransform>,
) -> HttpResult<Option<Transform>> {
    api.set_transform(entity, body).await.into_http_result()
}

#[cfg(test)]
mod tests {
    use crate::tests::assert_response;
    use axum::body::Body;
    use axum::http::Request;
    use bevy::prelude::*;

    #[tokio::test]
    async fn test_get_transform() {
        let (mut app, router) = crate::tests::test_app();

        let entity = app.world_mut().spawn(Transform::default()).id();
        let request = axum::http::Request::get(format!("/entities/{}/transform", entity.to_bits()))
            .body(axum::body::Body::empty())
            .unwrap();
        assert_response(&mut app, router, request, Transform::default()).await;
    }

    #[tokio::test]
    async fn test_rescale() {
        let (mut app, router) = crate::tests::test_app();
        let entity = app.world_mut().spawn(Transform::default()).id();
        let request = Request::put(format!("/entities/{}/transform", entity.to_bits()))
            .header("Content-Type", "application/json")
            .body(Body::from(
                serde_json::json!({
                    "scale": [2.0, 2.0, 2.0]
                })
                .to_string(),
            ))
            .unwrap();
        assert_response(
            &mut app,
            router,
            request,
            Transform::from_scale(Vec3::splat(2.0)),
        )
        .await;
    }

    #[tokio::test]
    async fn test_move() {
        let (mut app, router) = crate::tests::test_app();
        let entity = app.world_mut().spawn(Transform::default()).id();
        let request = Request::put(format!("/entities/{}/transform", entity.to_bits()))
            .header("Content-Type", "application/json")
            .body(Body::from(
                serde_json::json!({
                    "translation": [2.0, 2.0, 2.0]
                })
                .to_string(),
            ))
            .unwrap();
        assert_response(
            &mut app,
            router,
            request,
            Transform::from_translation(Vec3::splat(2.0)),
        )
        .await;
    }

    #[tokio::test]
    async fn test_rotate() {
        let (mut app, router) = crate::tests::test_app();
        let entity = app.world_mut().spawn(Transform::default()).id();
        let request = Request::put(format!("/entities/{}/transform", entity.to_bits()))
            .header("Content-Type", "application/json")
            .body(Body::from(
                serde_json::json!({
                    "rotation": [2.0, 2.0, 2.0, 1.0]
                })
                .to_string(),
            ))
            .unwrap();
        assert_response(
            &mut app,
            router,
            request,
            Transform::from_rotation(Quat::from_array([2.0, 2.0, 2.0, 1.0])),
        )
        .await;
    }
}
