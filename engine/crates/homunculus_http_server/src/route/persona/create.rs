use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use homunculus_api::persona::{CreatePersona, PersonaApi, PersonaSnapshot};

/// Create a new persona.
#[utoipa::path(
    post,
    path = "/",
    tag = "personas",
    request_body = CreatePersona,
    responses(
        (status = 201, description = "Persona created", body = PersonaSnapshot),
        (status = 400, description = "Invalid input"),
        (status = 409, description = "Persona already exists"),
    ),
)]
pub async fn create(State(api): State<PersonaApi>, Json(body): Json<CreatePersona>) -> Response {
    match api.create(body).await {
        Ok(persona) => (StatusCode::CREATED, Json(persona)).into_response(),
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
    async fn test_create_persona_201() {
        let (mut app, router) = test_app();
        let request = Request::post("/personas")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"id":"alice","name":"Alice"}"#))
            .unwrap();

        let response = call_any_status(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::CREATED);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let snap: PersonaSnapshot = serde_json::from_slice(&body).unwrap();
        assert_eq!(snap.persona.id.0, "alice");
        assert_eq!(snap.persona.name, Some("Alice".to_string()));
        assert_eq!(snap.state, "");
        assert!(!snap.spawned);
    }

    #[tokio::test]
    async fn test_create_persona_duplicate_409() {
        let (mut app, router) = test_app();

        let request = Request::post("/personas")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"id":"dup"}"#))
            .unwrap();
        let response = call_any_status(&mut app, router.clone(), request).await;
        assert_eq!(response.status(), StatusCode::CREATED);

        let request = Request::post("/personas")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"id":"dup"}"#))
            .unwrap();
        let response = call_any_status(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::CONFLICT);
    }

    #[tokio::test]
    async fn test_get_persona_200() {
        let (mut app, router) = test_app();

        let request = Request::post("/personas")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"id":"bob","name":"Bob"}"#))
            .unwrap();
        let response = call_any_status(&mut app, router.clone(), request).await;
        assert_eq!(response.status(), StatusCode::CREATED);

        let request = Request::get("/personas/bob").body(Body::empty()).unwrap();
        let response = call_any_status(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let snap: PersonaSnapshot = serde_json::from_slice(&body).unwrap();
        assert_eq!(snap.persona.id.0, "bob");
        assert_eq!(snap.persona.name, Some("Bob".to_string()));
        assert!(!snap.spawned);
        assert_eq!(snap.state, "");
    }

    #[tokio::test]
    async fn test_list_personas_200() {
        let (mut app, router) = test_app();

        let request = Request::get("/personas").body(Body::empty()).unwrap();
        let response = call_any_status(&mut app, router.clone(), request).await;
        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let personas: Vec<PersonaSnapshot> = serde_json::from_slice(&body).unwrap();
        assert!(personas.is_empty());

        let request = Request::post("/personas")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"id":"list-test"}"#))
            .unwrap();
        call_any_status(&mut app, router.clone(), request).await;

        let request = Request::get("/personas").body(Body::empty()).unwrap();
        let response = call_any_status(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let personas: Vec<PersonaSnapshot> = serde_json::from_slice(&body).unwrap();
        assert_eq!(personas.len(), 1);
        assert_eq!(personas[0].persona.id.0, "list-test");
    }

    #[tokio::test]
    async fn test_delete_persona_204() {
        let (mut app, router) = test_app();

        // Create persona (DB only)
        let request = Request::post("/personas")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"id":"to-delete"}"#))
            .unwrap();
        let response = call_any_status(&mut app, router.clone(), request).await;
        assert_eq!(response.status(), StatusCode::CREATED);

        // Spawn so delete can find the ECS entity
        let request = Request::post("/personas/to-delete/spawn")
            .body(Body::empty())
            .unwrap();
        let response = call_any_status(&mut app, router.clone(), request).await;
        assert_eq!(response.status(), StatusCode::OK);

        let request = Request::delete("/personas/to-delete")
            .body(Body::empty())
            .unwrap();
        let response = call_any_status(&mut app, router.clone(), request).await;
        assert_eq!(response.status(), StatusCode::NO_CONTENT);

        let request = Request::get("/personas/to-delete")
            .body(Body::empty())
            .unwrap();
        let response = call_any_status(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_patch_persona() {
        let (mut app, router) = test_app();

        // Create persona (DB only)
        let request = Request::post("/personas")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"id":"patch-me","name":"Before"}"#))
            .unwrap();
        let response = call_any_status(&mut app, router.clone(), request).await;
        assert_eq!(response.status(), StatusCode::CREATED);

        // Spawn so patch can access the ECS entity
        let request = Request::post("/personas/patch-me/spawn")
            .body(Body::empty())
            .unwrap();
        let response = call_any_status(&mut app, router.clone(), request).await;
        assert_eq!(response.status(), StatusCode::OK);

        let request = Request::patch("/personas/patch-me")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"name":"After"}"#))
            .unwrap();
        let response = call_any_status(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let snap: PersonaSnapshot = serde_json::from_slice(&body).unwrap();
        assert_eq!(snap.persona.name, Some("After".to_string()));
    }

    #[tokio::test]
    async fn test_get_persona_not_found_404() {
        let (mut app, router) = test_app();

        let request = Request::get("/personas/nonexistent")
            .body(Body::empty())
            .unwrap();
        let response = call_any_status(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_persona_accepts_vrm_asset_id() {
        let (mut app, router) = test_app();
        let request = Request::post("/personas")
            .header("content-type", "application/json")
            .body(Body::from(
                r#"{"id":"vrm-carrier","vrmAssetId":"vrm:example:model"}"#,
            ))
            .unwrap();

        let response = call_any_status(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::CREATED);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let snap: PersonaSnapshot = serde_json::from_slice(&body).unwrap();
        assert_eq!(snap.persona.vrm_asset_id.as_deref(), Some("vrm:example:model"));
    }

    #[tokio::test]
    async fn test_create_persona_accepts_thumbnail() {
        let (mut app, router) = test_app();
        let request = Request::post("/personas")
            .header("content-type", "application/json")
            .body(Body::from(
                r#"{"id":"thumb-carrier","thumbnail":"image:example:thumb"}"#,
            ))
            .unwrap();

        let response = call_any_status(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::CREATED);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let snap: PersonaSnapshot = serde_json::from_slice(&body).unwrap();
        assert_eq!(snap.persona.thumbnail.as_deref(), Some("image:example:thumb"));
    }
}
