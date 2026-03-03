use crate::extract::EntityId;
use axum::Json;
use axum::extract::State;
use homunculus_api::entities::MoveTarget;
use homunculus_api::prelude::EntitiesApi;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};

/// Move an entity to a target position.
#[utoipa::path(
    post,
    path = "/move",
    tag = "entities",
    params(("entity" = String, Path, description = "Entity ID")),
    request_body = MoveTarget,
    responses(
        (status = 200, description = "Entity moved"),
        (status = 404, description = "Entity not found"),
        (status = 422, description = "Invalid move target"),
    ),
)]
pub async fn move_to(
    State(api): State<EntitiesApi>,
    EntityId(entity): EntityId,
    Json(body): Json<MoveTarget>,
) -> HttpResult {
    api.move_to(entity, body).await.into_http_result()
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use bevy::prelude::*;

    fn move_request(entity_bits: u64, body: serde_json::Value) -> Request<Body> {
        Request::post(format!("/entities/{entity_bits}/move"))
            .header("Content-Type", "application/json")
            .body(Body::from(body.to_string()))
            .unwrap()
    }

    fn get_transform(app: &App, entity: Entity) -> Transform {
        *app.world().entity(entity).get::<Transform>().unwrap()
    }

    #[tokio::test]
    async fn move_world_sets_xyz_when_z_provided() {
        let (mut app, router) = crate::tests::test_app();
        let entity = app
            .world_mut()
            .spawn(Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)))
            .id();

        let request = move_request(
            entity.to_bits(),
            serde_json::json!({ "type": "world", "position": [1.0, 2.0], "z": 3.0 }),
        );
        crate::tests::call(&mut app, router, request).await;

        let tf = get_transform(&app, entity);
        assert_eq!(tf.translation, Vec3::new(1.0, 2.0, 3.0));
    }

    #[tokio::test]
    async fn move_world_sets_xy_and_preserves_z_when_z_omitted() {
        let (mut app, router) = crate::tests::test_app();
        let entity = app
            .world_mut()
            .spawn(Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)))
            .id();

        let request = move_request(
            entity.to_bits(),
            serde_json::json!({ "type": "world", "position": [10.0, 20.0] }),
        );
        crate::tests::call(&mut app, router, request).await;

        let tf = get_transform(&app, entity);
        assert_eq!(tf.translation, Vec3::new(10.0, 20.0, 5.0));
    }

    #[tokio::test]
    async fn move_world_sets_xy_and_preserves_z_when_z_null() {
        let (mut app, router) = crate::tests::test_app();
        let entity = app
            .world_mut()
            .spawn(Transform::from_translation(Vec3::new(0.0, 0.0, 7.0)))
            .id();

        let request = move_request(
            entity.to_bits(),
            serde_json::json!({ "type": "world", "position": [1.0, 2.0], "z": null }),
        );
        crate::tests::call(&mut app, router, request).await;

        let tf = get_transform(&app, entity);
        assert_eq!(tf.translation, Vec3::new(1.0, 2.0, 7.0));
    }

    #[tokio::test]
    async fn move_returns_404_when_entity_not_found() {
        let (mut app, router) = crate::tests::test_app();
        let request = move_request(
            9999999,
            serde_json::json!({ "type": "world", "position": [1.0, 2.0] }),
        );
        let response = crate::tests::call_any_status(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn move_returns_422_when_type_missing() {
        let (mut app, router) = crate::tests::test_app();
        let entity = app.world_mut().spawn(Transform::default()).id();
        let request = move_request(
            entity.to_bits(),
            serde_json::json!({ "position": [1.0, 2.0] }),
        );
        let response = crate::tests::call_any_status(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn move_returns_422_when_type_unknown() {
        let (mut app, router) = crate::tests::test_app();
        let entity = app.world_mut().spawn(Transform::default()).id();
        let request = move_request(
            entity.to_bits(),
            serde_json::json!({ "type": "foo", "position": [1.0, 2.0] }),
        );
        let response = crate::tests::call_any_status(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn move_returns_422_when_world_missing_position() {
        let (mut app, router) = crate::tests::test_app();
        let entity = app.world_mut().spawn(Transform::default()).id();
        let request = move_request(entity.to_bits(), serde_json::json!({ "type": "world" }));
        let response = crate::tests::call_any_status(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn move_viewport_returns_error_without_camera() {
        let (mut app, router) = crate::tests::test_app();
        let entity = app.world_mut().spawn(Transform::default()).id();
        let request = move_request(
            entity.to_bits(),
            serde_json::json!({ "type": "viewport", "position": [100.0, 200.0] }),
        );
        let response = crate::tests::call_any_status(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn move_world_without_z_after_world_with_z_preserves_latest_z() {
        let (mut app, router) = crate::tests::test_app();
        let entity = app
            .world_mut()
            .spawn(Transform::from_translation(Vec3::ZERO))
            .id();

        let req1 = move_request(
            entity.to_bits(),
            serde_json::json!({ "type": "world", "position": [1.0, 2.0], "z": 10.0 }),
        );
        crate::tests::call(&mut app, router.clone(), req1).await;

        let req2 = move_request(
            entity.to_bits(),
            serde_json::json!({ "type": "world", "position": [5.0, 6.0] }),
        );
        crate::tests::call(&mut app, router, req2).await;

        let tf = get_transform(&app, entity);
        assert_eq!(tf.translation, Vec3::new(5.0, 6.0, 10.0));
    }
}
