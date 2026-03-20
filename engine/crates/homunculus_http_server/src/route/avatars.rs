pub mod extensions;

use crate::extract::avatar::AvatarIdExtractor;
use axum::Json;
use axum::extract::{Query, State};
use homunculus_api::avatar::{AvatarApi, AvatarInfo};
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_core::prelude::{AvatarId, AvatarState, Persona};
use homunculus_utils::prelude::AssetId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Detailed information about a single avatar, including persona.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AvatarDetail {
    /// The unique avatar identifier.
    pub id: String,
    /// The display name.
    pub name: String,
    /// The asset ID of the VRM model bound to this avatar.
    pub asset_id: String,
    /// The current behavioral state (e.g. "idle", "sitting").
    pub state: String,
    /// Whether a VRM model is currently attached.
    pub has_vrm: bool,
    /// The avatar's persona configuration.
    pub persona: Persona,
}

/// Query parameters for the create endpoint.
#[derive(Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateQuery {
    /// When true, returns the existing avatar if the ID is already taken
    /// instead of raising a conflict error.
    pub ensure: Option<bool>,
}

/// Request body for creating a new avatar.
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateBody {
    /// The unique avatar identifier.
    pub id: String,
    /// The asset ID of the VRM model to bind.
    pub asset_id: String,
    /// Optional display name. Defaults to the `id` when omitted.
    pub name: Option<String>,
}

/// Request body for updating the avatar state.
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PutStateBody {
    /// The new behavioral state value.
    #[schema(value_type = Object)]
    pub state: AvatarState,
}

/// Request body for updating the avatar display name.
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PutNameBody {
    /// The new display name.
    pub name: String,
}

/// Request body for attaching a VRM model.
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AttachVrmBody {
    /// The asset ID of the VRM model to attach.
    pub asset_id: String,
}

/// Create a new avatar.
#[utoipa::path(
    post,
    path = "",
    tag = "avatars",
    params(("ensure" = Option<bool>, Query, description = "Return existing if ID is taken")),
    request_body = CreateBody,
    responses(
        (status = 200, description = "Avatar created", body = AvatarInfo),
        (status = 400, description = "Invalid avatar ID"),
        (status = 409, description = "Avatar already exists"),
    ),
)]
pub async fn create(
    State(api): State<AvatarApi>,
    Query(query): Query<CreateQuery>,
    Json(body): Json<CreateBody>,
) -> HttpResult<AvatarInfo> {
    let id = AvatarId::new(&body.id)
        .map_err(|e| homunculus_api::prelude::ApiError::InvalidAvatarId(e.to_string()))?;
    let asset_id = AssetId::new(&body.asset_id);
    let name = body.name.unwrap_or_else(|| body.id.clone());
    let ensure = query.ensure.unwrap_or(false);

    api.create(id.clone(), asset_id, name, ensure).await?;
    api.get_info(id).await.into_http_result()
}

/// List all avatars.
#[utoipa::path(
    get,
    path = "",
    tag = "avatars",
    responses(
        (status = 200, description = "List of avatars", body = Vec<AvatarInfo>),
    ),
)]
pub async fn list(State(api): State<AvatarApi>) -> HttpResult<Vec<AvatarInfo>> {
    api.list().await.into_http_result()
}

/// Get detailed information about a single avatar.
#[utoipa::path(
    get,
    path = "",
    tag = "avatars",
    params(("id" = String, Path, description = "Avatar ID")),
    responses(
        (status = 200, description = "Avatar detail", body = AvatarDetail),
        (status = 404, description = "Avatar not found"),
    ),
)]
pub async fn get(
    State(api): State<AvatarApi>,
    AvatarIdExtractor { id, .. }: AvatarIdExtractor,
) -> HttpResult<AvatarDetail> {
    let info = api.get_info(id.clone()).await?;
    let persona = api.get_persona(id).await?;
    let detail = AvatarDetail {
        id: info.id,
        name: info.name,
        asset_id: info.asset_id,
        state: info.state,
        has_vrm: info.has_vrm,
        persona,
    };
    Ok(Json(detail))
}

/// Destroy an avatar.
#[utoipa::path(
    delete,
    path = "",
    tag = "avatars",
    params(("id" = String, Path, description = "Avatar ID")),
    responses(
        (status = 200, description = "Avatar destroyed"),
        (status = 404, description = "Avatar not found"),
    ),
)]
pub async fn destroy(
    State(api): State<AvatarApi>,
    AvatarIdExtractor { id, .. }: AvatarIdExtractor,
) -> HttpResult {
    api.destroy(id).await.into_http_result()
}

/// Get the current state of an avatar.
#[utoipa::path(
    get,
    path = "/state",
    tag = "avatars",
    params(("id" = String, Path, description = "Avatar ID")),
    responses(
        (status = 200, description = "Avatar state", body = PutStateBody),
        (status = 404, description = "Avatar not found"),
    ),
)]
pub async fn get_state(
    State(api): State<AvatarApi>,
    AvatarIdExtractor { id, .. }: AvatarIdExtractor,
) -> HttpResult<serde_json::Value> {
    let state = api.get_state(id).await?;
    Ok(Json(serde_json::json!({ "state": state.0 })))
}

/// Update the state of an avatar.
#[utoipa::path(
    put,
    path = "/state",
    tag = "avatars",
    params(("id" = String, Path, description = "Avatar ID")),
    request_body = PutStateBody,
    responses(
        (status = 200, description = "State updated"),
        (status = 404, description = "Avatar not found"),
    ),
)]
pub async fn put_state(
    State(api): State<AvatarApi>,
    AvatarIdExtractor { id, .. }: AvatarIdExtractor,
    Json(body): Json<PutStateBody>,
) -> HttpResult {
    api.set_state(id, body.state).await.into_http_result()
}

/// Get the persona of an avatar.
#[utoipa::path(
    get,
    path = "/persona",
    tag = "avatars",
    params(("id" = String, Path, description = "Avatar ID")),
    responses(
        (status = 200, description = "Avatar persona", body = Persona),
        (status = 404, description = "Avatar not found"),
    ),
)]
pub async fn get_persona(
    State(api): State<AvatarApi>,
    AvatarIdExtractor { id, .. }: AvatarIdExtractor,
) -> HttpResult<Persona> {
    api.get_persona(id).await.into_http_result()
}

/// Update the persona of an avatar.
#[utoipa::path(
    put,
    path = "/persona",
    tag = "avatars",
    params(("id" = String, Path, description = "Avatar ID")),
    request_body = Persona,
    responses(
        (status = 200, description = "Persona updated"),
        (status = 404, description = "Avatar not found"),
    ),
)]
pub async fn put_persona(
    State(api): State<AvatarApi>,
    AvatarIdExtractor { id, .. }: AvatarIdExtractor,
    Json(body): Json<Persona>,
) -> HttpResult {
    api.set_persona(id, body).await.into_http_result()
}

/// Get the display name of an avatar.
#[utoipa::path(
    get,
    path = "/name",
    tag = "avatars",
    params(("id" = String, Path, description = "Avatar ID")),
    responses(
        (status = 200, description = "Avatar name", body = PutNameBody),
        (status = 404, description = "Avatar not found"),
    ),
)]
pub async fn get_name(
    State(api): State<AvatarApi>,
    AvatarIdExtractor { id, .. }: AvatarIdExtractor,
) -> HttpResult<serde_json::Value> {
    let name = api.get_name(id).await?;
    Ok(Json(serde_json::json!({ "name": name })))
}

/// Update the display name of an avatar.
#[utoipa::path(
    put,
    path = "/name",
    tag = "avatars",
    params(("id" = String, Path, description = "Avatar ID")),
    request_body = PutNameBody,
    responses(
        (status = 200, description = "Name updated"),
        (status = 404, description = "Avatar not found"),
    ),
)]
pub async fn put_name(
    State(api): State<AvatarApi>,
    AvatarIdExtractor { id, .. }: AvatarIdExtractor,
    Json(body): Json<PutNameBody>,
) -> HttpResult {
    api.set_name(id, body.name).await.into_http_result()
}

/// Attach a VRM model to an avatar.
#[utoipa::path(
    post,
    path = "/vrm/attach",
    tag = "avatars",
    params(("id" = String, Path, description = "Avatar ID")),
    request_body = AttachVrmBody,
    responses(
        (status = 200, description = "VRM attached"),
        (status = 404, description = "Avatar not found"),
    ),
)]
pub async fn attach_vrm(
    State(api): State<AvatarApi>,
    AvatarIdExtractor { id, .. }: AvatarIdExtractor,
    Json(body): Json<AttachVrmBody>,
) -> HttpResult {
    let asset_id = AssetId::new(&body.asset_id);
    api.attach_vrm(id, asset_id).await?;
    Ok(Json(()))
}

/// Detach the VRM model from an avatar.
#[utoipa::path(
    delete,
    path = "/vrm",
    tag = "avatars",
    params(("id" = String, Path, description = "Avatar ID")),
    responses(
        (status = 200, description = "VRM detached"),
        (status = 404, description = "Avatar not found"),
        (status = 422, description = "No VRM attached"),
    ),
)]
pub async fn detach_vrm(
    State(api): State<AvatarApi>,
    AvatarIdExtractor { id, .. }: AvatarIdExtractor,
) -> HttpResult {
    api.detach_vrm(id).await.into_http_result()
}

#[cfg(test)]
mod tests {
    use crate::tests::{assert_response, call, call_any_status, test_app};
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use bevy::prelude::*;
    use homunculus_api::avatar::AvatarInfo;
    use homunculus_core::prelude::{
        AssetId, AssetIdComponent, Avatar, AvatarId, AvatarName, AvatarState, Persona,
    };

    /// Spawns an avatar entity with the necessary components and runs an
    /// update so the `AvatarRegistry` observer registers it.
    fn spawn_avatar(app: &mut App, id: &str, name: &str, asset_id: &str) -> Entity {
        let avatar_id = AvatarId::new(id).unwrap();
        let entity = app
            .world_mut()
            .spawn((
                Avatar,
                avatar_id,
                AvatarName(name.to_string()),
                Name::new(name.to_string()),
                AssetIdComponent(AssetId::new(asset_id)),
                AvatarState::default(),
                Persona::default(),
            ))
            .id();
        app.update();
        entity
    }

    #[tokio::test]
    async fn test_list_avatars_empty() {
        let (mut app, router) = test_app();
        let request = Request::get("/avatars").body(Body::empty()).unwrap();
        assert_response::<Vec<AvatarInfo>>(&mut app, router, request, vec![]).await;
    }

    #[tokio::test]
    async fn test_list_avatars_with_entry() {
        let (mut app, router) = test_app();
        spawn_avatar(&mut app, "elmer", "Elmer", "test:model.vrm");

        let request = Request::get("/avatars").body(Body::empty()).unwrap();
        assert_response(
            &mut app,
            router,
            request,
            vec![AvatarInfo {
                id: "elmer".to_string(),
                name: "Elmer".to_string(),
                asset_id: "test:model.vrm".to_string(),
                state: "idle".to_string(),
                has_vrm: false,
            }],
        )
        .await;
    }

    #[tokio::test]
    async fn test_get_avatar_detail() {
        let (mut app, router) = test_app();
        spawn_avatar(&mut app, "elmer", "Elmer", "test:model.vrm");

        let request = Request::get("/avatars/elmer").body(Body::empty()).unwrap();
        assert_response(
            &mut app,
            router,
            request,
            super::AvatarDetail {
                id: "elmer".to_string(),
                name: "Elmer".to_string(),
                asset_id: "test:model.vrm".to_string(),
                state: "idle".to_string(),
                has_vrm: false,
                persona: Persona::default(),
            },
        )
        .await;
    }

    #[tokio::test]
    async fn test_get_avatar_not_found() {
        let (mut app, router) = test_app();
        let request = Request::get("/avatars/nonexistent")
            .body(Body::empty())
            .unwrap();
        let response = call_any_status(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_destroy_avatar() {
        let (mut app, router) = test_app();
        spawn_avatar(&mut app, "elmer", "Elmer", "test:model.vrm");

        let request = Request::delete("/avatars/elmer")
            .body(Body::empty())
            .unwrap();
        let response = call(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_avatar_state() {
        let (mut app, router) = test_app();
        spawn_avatar(&mut app, "elmer", "Elmer", "test:model.vrm");

        let request = Request::get("/avatars/elmer/state")
            .body(Body::empty())
            .unwrap();
        assert_response(
            &mut app,
            router,
            request,
            serde_json::json!({ "state": "idle" }),
        )
        .await;
    }

    #[tokio::test]
    async fn test_put_avatar_state() {
        let (mut app, router) = test_app();
        spawn_avatar(&mut app, "elmer", "Elmer", "test:model.vrm");

        let request = Request::put("/avatars/elmer/state")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"state":"dancing"}"#))
            .unwrap();
        let response = call(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_avatar_persona() {
        let (mut app, router) = test_app();
        spawn_avatar(&mut app, "elmer", "Elmer", "test:model.vrm");

        let request = Request::get("/avatars/elmer/persona")
            .body(Body::empty())
            .unwrap();
        assert_response(&mut app, router, request, Persona::default()).await;
    }

    #[tokio::test]
    async fn test_put_avatar_persona() {
        let (mut app, router) = test_app();
        spawn_avatar(&mut app, "elmer", "Elmer", "test:model.vrm");

        let persona = Persona {
            profile: "A cheerful assistant".to_string(),
            personality: Some("Friendly".to_string()),
            ..Default::default()
        };
        let request = Request::put("/avatars/elmer/persona")
            .header("Content-Type", "application/json")
            .body(Body::from(serde_json::to_string(&persona).unwrap()))
            .unwrap();
        let response = call(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_avatar_name() {
        let (mut app, router) = test_app();
        spawn_avatar(&mut app, "elmer", "Elmer", "test:model.vrm");

        let request = Request::get("/avatars/elmer/name")
            .body(Body::empty())
            .unwrap();
        assert_response(
            &mut app,
            router,
            request,
            serde_json::json!({ "name": "Elmer" }),
        )
        .await;
    }

    #[tokio::test]
    async fn test_put_avatar_name() {
        let (mut app, router) = test_app();
        spawn_avatar(&mut app, "elmer", "Elmer", "test:model.vrm");

        let request = Request::put("/avatars/elmer/name")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"name":"New Name"}"#))
            .unwrap();
        let response = call(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_invalid_avatar_id_rejected() {
        let (mut app, router) = test_app();
        let request = Request::get("/avatars/INVALID")
            .body(Body::empty())
            .unwrap();
        let response = call_any_status(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    /// Spawns an avatar entity AND creates the corresponding database row
    /// so that extension operations (which need the FK) succeed.
    fn spawn_avatar_with_db(app: &mut App, id: &str, name: &str, asset_id: &str) -> Entity {
        use homunculus_prefs::avatar_repo::AvatarRepo;
        use homunculus_prefs::PrefsDatabase;

        let db = app.world().get_non_send_resource::<PrefsDatabase>().unwrap();
        AvatarRepo::new(db)
            .create(id, asset_id, name, "{}", "{}")
            .unwrap();

        spawn_avatar(app, id, name, asset_id)
    }

    #[tokio::test]
    async fn test_set_and_get_extension() {
        let (mut app, router) = test_app();
        spawn_avatar_with_db(&mut app, "elmer", "Elmer", "test:model.vrm");

        let set_req = Request::put("/avatars/elmer/extensions/voicevox")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"speakerId":1}"#))
            .unwrap();
        let response = call(&mut app, router.clone(), set_req).await;
        assert_eq!(response.status(), StatusCode::OK);

        let get_req = Request::get("/avatars/elmer/extensions/voicevox")
            .body(Body::empty())
            .unwrap();
        assert_response(
            &mut app,
            router,
            get_req,
            serde_json::json!({"speakerId": 1}),
        )
        .await;
    }

    #[tokio::test]
    async fn test_delete_extension() {
        let (mut app, router) = test_app();
        spawn_avatar_with_db(&mut app, "elmer", "Elmer", "test:model.vrm");

        // Set first
        let set_req = Request::put("/avatars/elmer/extensions/voicevox")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"v":1}"#))
            .unwrap();
        let response = call(&mut app, router.clone(), set_req).await;
        assert_eq!(response.status(), StatusCode::OK);

        // Delete
        let del_req = Request::delete("/avatars/elmer/extensions/voicevox")
            .body(Body::empty())
            .unwrap();
        let response = call(&mut app, router.clone(), del_req).await;
        assert_eq!(response.status(), StatusCode::OK);

        // Get should 404
        let get_req = Request::get("/avatars/elmer/extensions/voicevox")
            .body(Body::empty())
            .unwrap();
        let response = call_any_status(&mut app, router, get_req).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_extension_not_found() {
        let (mut app, router) = test_app();
        spawn_avatar_with_db(&mut app, "elmer", "Elmer", "test:model.vrm");

        let request = Request::get("/avatars/elmer/extensions/nonexistent")
            .body(Body::empty())
            .unwrap();
        let response = call_any_status(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_extension_avatar_not_found() {
        let (mut app, router) = test_app();

        let request = Request::get("/avatars/nonexistent/extensions/voicevox")
            .body(Body::empty())
            .unwrap();
        let response = call_any_status(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
