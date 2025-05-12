use crate::extract::EntityId;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_api::vrm::VrmApi;
use homunculus_core::prelude::VrmState;
use serde::{Deserialize, Serialize};

/// Get the state of a VRM model.
///
/// ### Path
///
/// `GET /vrm/:entity_id/state`
pub async fn get(
    axum::extract::State(api): axum::extract::State<VrmApi>,
    EntityId(entity): EntityId,
) -> HttpResult<serde_json::Value> {
    api.state(entity)
        .await
        .map(|s| {
            serde_json::json!({
                "state": s.0
            })
        })
        .into_http_result()
}

/// Set the state of a VRM model.
///
/// ### Path
///
/// `PUT /vrm/:entity_id/state`
pub async fn put(
    axum::extract::State(api): axum::extract::State<VrmApi>,
    EntityId(entity): EntityId,
    axum::extract::Json(body): axum::extract::Json<PutBody>,
) -> HttpResult {
    api.set_state(entity, body.state).await.into_http_result()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PutBody {
    state: VrmState,
}

#[cfg(test)]
mod tests {
    use crate::route::vrm::state::PutBody;
    use crate::tests::{assert_response, call, test_app};
    use axum::http::StatusCode;
    use bevy::prelude::*;
    use homunculus_core::prelude::VrmState;

    #[tokio::test]
    async fn test_get_vrm_state() {
        let (mut app, router) = test_app();
        let entity = app.world_mut().spawn(VrmState::default()).id();
        app.update();

        let request = axum::http::Request::get(format!("/vrm/{}/state", entity.to_bits()))
            .body(axum::body::Body::empty())
            .unwrap();
        assert_response(
            &mut app,
            router,
            request,
            serde_json::json!({
                "state": "idle"
            }),
        )
        .await;
    }

    #[tokio::test]
    async fn test_put_vrm_state() {
        let (mut app, router) = test_app();
        let entity = app.world_mut().spawn(VrmState::default()).id();
        app.update();

        let request = axum::http::Request::put(format!("/vrm/{}/state", entity.to_bits()))
            .header("Content-Type", "application/json")
            .body(axum::body::Body::from(
                serde_json::to_string(&PutBody {
                    state: VrmState("dancing".to_string()),
                })
                .unwrap(),
            ))
            .unwrap();
        let response = call(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::OK);
    }
}
