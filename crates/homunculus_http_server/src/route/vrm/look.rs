use crate::extract::EntityId;
use axum::extract::{Path, State};
use bevy::prelude::Entity;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_api::vrm::VrmApi;

/// Enables look-at control for the specified VRM entity, targeting another entity.
///
/// ## Path
///
/// `DELETE /vrm/:vrm/look`
pub async fn unlook(State(api): State<VrmApi>, EntityId(vrm): EntityId) -> HttpResult {
    api.unlook(vrm).await.into_http_result()
}

/// Enables look-at control for the specified VRM entity, targeting another entity.
///
/// ## Path
///
/// `PUT /vrm/:vrm/look/target/:target`
pub async fn target(
    State(api): State<VrmApi>,
    Path((vrm, target)): Path<(u64, u64)>,
) -> HttpResult {
    let vrm = Entity::from_bits(vrm);
    let target = Entity::from_bits(target);
    api.look_at_target(vrm, target).await.into_http_result()
}

/// Enables look-at control for the specified VRM entity, making it look at the cursor.
///
/// ## Path
///
/// `PUT /vrm/:vrm/look/cursor`
pub async fn cursor(State(api): State<VrmApi>, EntityId(vrm): EntityId) -> HttpResult {
    api.look_at_cursor(vrm).await.into_http_result()
}

#[cfg(test)]
mod tests {
    use crate::tests::{call, test_app};
    use bevy::prelude::*;
    use bevy_vrm1::prelude::LookAt;

    #[tokio::test]
    async fn test_unlook() {
        let (mut app, router) = test_app();
        let entity = app.world_mut().spawn(LookAt::Cursor { camera: None }).id();
        app.update();

        let request = axum::http::Request::delete(format!("/vrm/{}/look", entity.to_bits()))
            .body(axum::body::Body::empty())
            .unwrap();
        call(&mut app, router, request).await;
        assert!(
            !app.world().entity(entity).contains::<LookAt>(),
            "LookAt component should be removed"
        );
    }

    #[tokio::test]
    async fn test_look_at_target() {
        let (mut app, router) = test_app();
        let entity = app.world_mut().spawn(LookAt::Cursor { camera: None }).id();
        let target = app.world_mut().spawn_empty().id();
        app.update();

        let request = axum::http::Request::put(format!(
            "/vrm/{}/look/target/{}",
            entity.to_bits(),
            target.to_bits()
        ))
        .body(axum::body::Body::empty())
        .unwrap();
        call(&mut app, router, request).await;
        assert_eq!(
            app.world().entity(entity).get::<LookAt>().unwrap(),
            &LookAt::Target(target),
        );
    }

    #[tokio::test]
    async fn test_look_at_cursor() {
        let (mut app, router) = test_app();
        let entity = app.world_mut().spawn(LookAt::Cursor { camera: None }).id();
        app.update();

        let request = axum::http::Request::put(format!("/vrm/{}/look/cursor", entity.to_bits(),))
            .body(axum::body::Body::empty())
            .unwrap();
        call(&mut app, router, request).await;
        assert_eq!(
            app.world().entity(entity).get::<LookAt>().unwrap(),
            &LookAt::default(),
        );
    }
}
