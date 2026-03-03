use crate::extract::EntityId;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_api::vrm::VrmApi;
use homunculus_core::prelude::Persona;

/// Get the persona of a VRM model.
#[utoipa::path(
    get,
    path = "/persona",
    tag = "vrm",
    params(("entity" = String, Path, description = "Entity ID")),
    responses(
        (status = 200, description = "VRM persona", body = Persona),
        (status = 404, description = "Entity not found"),
    ),
)]
pub async fn get(
    axum::extract::State(api): axum::extract::State<VrmApi>,
    EntityId(entity): EntityId,
) -> HttpResult<Persona> {
    api.persona(entity).await.into_http_result()
}

/// Set the persona of a VRM model.
#[utoipa::path(
    put,
    path = "/persona",
    tag = "vrm",
    params(("entity" = String, Path, description = "Entity ID")),
    request_body = Persona,
    responses(
        (status = 200, description = "Persona updated"),
        (status = 404, description = "Entity not found"),
    ),
)]
pub async fn put(
    axum::extract::State(api): axum::extract::State<VrmApi>,
    EntityId(entity): EntityId,
    axum::extract::Json(body): axum::extract::Json<Persona>,
) -> HttpResult {
    api.set_persona(entity, body).await.into_http_result()
}

#[cfg(test)]
mod tests {
    use crate::tests::{assert_response, call, test_app};
    use axum::http::StatusCode;
    use bevy::prelude::*;
    use homunculus_core::prelude::{AssetId, AssetIdComponent, Persona};

    #[tokio::test]
    async fn test_get_persona() {
        let (mut app, router) = test_app();
        let entity = app.world_mut().spawn(Persona::default()).id();
        app.update();

        let request = axum::http::Request::get(format!("/vrm/{}/persona", entity.to_bits()))
            .body(axum::body::Body::empty())
            .unwrap();
        assert_response(&mut app, router, request, Persona::default()).await;
    }

    #[tokio::test]
    async fn test_put_persona() {
        let (mut app, router) = test_app();
        let entity = app
            .world_mut()
            .spawn((
                Persona::default(),
                AssetIdComponent(AssetId::new("test::model.vrm")),
            ))
            .id();
        app.update();

        let persona = Persona {
            profile: "A cheerful assistant".to_string(),
            personality: Some("Friendly and helpful".to_string()),
            ..Default::default()
        };

        let request = axum::http::Request::put(format!("/vrm/{}/persona", entity.to_bits()))
            .header("Content-Type", "application/json")
            .body(axum::body::Body::from(
                serde_json::to_string(&persona).unwrap(),
            ))
            .unwrap();
        let response = call(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::OK);
    }
}
