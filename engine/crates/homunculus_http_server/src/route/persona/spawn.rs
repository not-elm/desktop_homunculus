use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use homunculus_api::persona::{PersonaApi, PersonaSnapshot};
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};

use super::PersonaPath;

/// Spawn persona ECS entity from DB record.
#[utoipa::path(
    post,
    path = "/spawn",
    tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    responses(
        (status = 200, description = "Persona spawned", body = PersonaSnapshot),
        (status = 404, description = "Persona not found in DB"),
        (status = 409, description = "Already spawned"),
    ),
)]
pub async fn spawn(
    State(api): State<PersonaApi>,
    path: PersonaPath,
) -> HttpResult<PersonaSnapshot> {
    api.spawn(path.persona_id).await.into_http_result()
}

/// Despawn persona entity, retaining DB record.
#[utoipa::path(
    post,
    path = "/despawn",
    tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    responses(
        (status = 200, description = "Persona despawned"),
        (status = 404, description = "Not spawned"),
    ),
)]
pub async fn despawn(State(api): State<PersonaApi>, path: PersonaPath) -> Response {
    match api.despawn(path.persona_id).await {
        Ok(()) => StatusCode::OK.into_response(),
        Err(e) => e.into_response(),
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::{call_any_status, test_app};
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use homunculus_api::persona::PersonaSnapshot;
    use http_body_util::BodyExt;

    #[tokio::test]
    async fn test_spawn_persona_200() {
        let (mut app, router) = test_app();

        // 1. Create persona via POST /personas (DB only, not spawned)
        let request = Request::post("/personas")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"id":"spawn-test","name":"SpawnTest"}"#))
            .unwrap();
        let response = call_any_status(&mut app, router.clone(), request).await;
        assert_eq!(response.status(), StatusCode::CREATED);

        // 2. Check — should have spawned: false (DB only)
        let request = Request::get("/personas/spawn-test")
            .body(Body::empty())
            .unwrap();
        let response = call_any_status(&mut app, router.clone(), request).await;
        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let snap: PersonaSnapshot = serde_json::from_slice(&body).unwrap();
        assert!(!snap.spawned);

        // 3. Spawn via POST /personas/{id}/spawn
        let request = Request::post("/personas/spawn-test/spawn")
            .body(Body::empty())
            .unwrap();
        let response = call_any_status(&mut app, router.clone(), request).await;
        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let snap: PersonaSnapshot = serde_json::from_slice(&body).unwrap();
        assert!(snap.spawned);
        assert_eq!(snap.persona.name, Some("SpawnTest".to_string()));

        // 4. Check — should have spawned: true
        let request = Request::get("/personas/spawn-test")
            .body(Body::empty())
            .unwrap();
        let response = call_any_status(&mut app, router.clone(), request).await;
        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let snap: PersonaSnapshot = serde_json::from_slice(&body).unwrap();
        assert!(snap.spawned);

        // 5. Despawn via POST /personas/{id}/despawn
        let request = Request::post("/personas/spawn-test/despawn")
            .body(Body::empty())
            .unwrap();
        let response = call_any_status(&mut app, router.clone(), request).await;
        assert_eq!(response.status(), StatusCode::OK);

        // 6. Check — should have spawned: false again
        let request = Request::get("/personas/spawn-test")
            .body(Body::empty())
            .unwrap();
        let response = call_any_status(&mut app, router.clone(), request).await;
        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let snap: PersonaSnapshot = serde_json::from_slice(&body).unwrap();
        assert!(!snap.spawned);

        // 7. Re-spawn via POST /personas/{id}/spawn
        let request = Request::post("/personas/spawn-test/spawn")
            .body(Body::empty())
            .unwrap();
        let response = call_any_status(&mut app, router.clone(), request).await;
        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let snap: PersonaSnapshot = serde_json::from_slice(&body).unwrap();
        assert!(snap.spawned);

        // 8. Check — should have spawned: true again
        let request = Request::get("/personas/spawn-test")
            .body(Body::empty())
            .unwrap();
        let response = call_any_status(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let snap: PersonaSnapshot = serde_json::from_slice(&body).unwrap();
        assert!(snap.spawned);
    }

    #[tokio::test]
    async fn test_spawn_already_spawned_409() {
        let (mut app, router) = test_app();

        // Create persona (DB only)
        let request = Request::post("/personas")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"id":"already-spawned"}"#))
            .unwrap();
        let response = call_any_status(&mut app, router.clone(), request).await;
        assert_eq!(response.status(), StatusCode::CREATED);

        // Spawn the persona
        let request = Request::post("/personas/already-spawned/spawn")
            .body(Body::empty())
            .unwrap();
        let response = call_any_status(&mut app, router.clone(), request).await;
        assert_eq!(response.status(), StatusCode::OK);

        // Spawn again — should get 409
        let request = Request::post("/personas/already-spawned/spawn")
            .body(Body::empty())
            .unwrap();
        let response = call_any_status(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::CONFLICT);
    }

    #[tokio::test]
    async fn test_despawn_not_spawned_404() {
        let (mut app, router) = test_app();

        // Create persona (DB only)
        let request = Request::post("/personas")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"id":"despawn-twice"}"#))
            .unwrap();
        let response = call_any_status(&mut app, router.clone(), request).await;
        assert_eq!(response.status(), StatusCode::CREATED);

        // Spawn
        let request = Request::post("/personas/despawn-twice/spawn")
            .body(Body::empty())
            .unwrap();
        let response = call_any_status(&mut app, router.clone(), request).await;
        assert_eq!(response.status(), StatusCode::OK);

        // Despawn
        let request = Request::post("/personas/despawn-twice/despawn")
            .body(Body::empty())
            .unwrap();
        let response = call_any_status(&mut app, router.clone(), request).await;
        assert_eq!(response.status(), StatusCode::OK);

        // Despawn again — should get 404
        let request = Request::post("/personas/despawn-twice/despawn")
            .body(Body::empty())
            .unwrap();
        let response = call_any_status(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
