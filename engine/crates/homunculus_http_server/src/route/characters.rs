pub mod extensions;

use crate::extract::character::CharacterIdExtractor;
use axum::Json;
use axum::extract::State;
use homunculus_api::character::{CharacterApi, CharacterInfo};
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_core::prelude::{CharacterId, CharacterState, Persona};
use homunculus_utils::prelude::AssetId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Detailed information about a single character, including persona.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CharacterDetail {
    /// The unique character identifier.
    pub id: String,
    /// The display name.
    pub name: String,
    /// The asset ID of the VRM model bound to this character.
    pub asset_id: String,
    /// The current behavioral state (e.g. "idle", "sitting").
    pub state: String,
    /// Whether a VRM model is currently attached.
    pub has_vrm: bool,
    /// The character's persona configuration.
    pub persona: Persona,
}

/// Request body for creating a new character.
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateBody {
    /// The unique character identifier.
    pub id: String,
    /// The asset ID of the VRM model to bind.
    pub asset_id: String,
    /// Optional display name. Defaults to the `id` when omitted.
    pub name: Option<String>,
}

/// Request body for updating the character state.
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PutStateBody {
    /// The new behavioral state value.
    #[schema(value_type = Object)]
    pub state: CharacterState,
}

/// Request body for updating the character display name.
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

/// Create a new character.
///
/// If a character with the given ID already exists, its info is returned
/// without creating a duplicate (upsert semantics).
#[utoipa::path(
    post,
    path = "",
    tag = "characters",
    request_body = CreateBody,
    responses(
        (status = 200, description = "Character created or existing returned", body = CharacterInfo),
        (status = 400, description = "Invalid character ID"),
    ),
)]
pub async fn create(
    State(api): State<CharacterApi>,
    Json(body): Json<CreateBody>,
) -> HttpResult<CharacterInfo> {
    let id = CharacterId::new(&body.id)
        .map_err(|e| homunculus_api::prelude::ApiError::InvalidCharacterId(e.to_string()))?;
    let asset_id = AssetId::new(&body.asset_id);
    let name = body.name.unwrap_or_else(|| body.id.clone());

    api.create(id, asset_id, name).await.into_http_result()
}

/// List all characters.
#[utoipa::path(
    get,
    path = "",
    tag = "characters",
    responses(
        (status = 200, description = "List of characters", body = Vec<CharacterInfo>),
    ),
)]
pub async fn list(State(api): State<CharacterApi>) -> HttpResult<Vec<CharacterInfo>> {
    api.list().await.into_http_result()
}

/// Get detailed information about a single character.
#[utoipa::path(
    get,
    path = "",
    tag = "characters",
    params(("id" = String, Path, description = "Character ID")),
    responses(
        (status = 200, description = "Character detail", body = CharacterDetail),
        (status = 404, description = "Character not found"),
    ),
)]
pub async fn get(
    State(api): State<CharacterApi>,
    CharacterIdExtractor { id, .. }: CharacterIdExtractor,
) -> HttpResult<CharacterDetail> {
    let info = api.get_info(id.clone()).await?;
    let persona = api.get_persona(id).await?;
    let detail = CharacterDetail {
        id: info.id,
        name: info.name,
        asset_id: info.asset_id,
        state: info.state,
        has_vrm: info.has_vrm,
        persona,
    };
    Ok(Json(detail))
}

/// Destroy a character.
#[utoipa::path(
    delete,
    path = "",
    tag = "characters",
    params(("id" = String, Path, description = "Character ID")),
    responses(
        (status = 200, description = "Character destroyed"),
        (status = 404, description = "Character not found"),
    ),
)]
pub async fn destroy(
    State(api): State<CharacterApi>,
    CharacterIdExtractor { id, .. }: CharacterIdExtractor,
) -> HttpResult {
    api.destroy(id).await.into_http_result()
}

/// Get the current state of a character.
#[utoipa::path(
    get,
    path = "/state",
    tag = "characters",
    params(("id" = String, Path, description = "Character ID")),
    responses(
        (status = 200, description = "Character state", body = PutStateBody),
        (status = 404, description = "Character not found"),
    ),
)]
pub async fn get_state(
    State(api): State<CharacterApi>,
    CharacterIdExtractor { id, .. }: CharacterIdExtractor,
) -> HttpResult<serde_json::Value> {
    let state = api.get_state(id).await?;
    Ok(Json(serde_json::json!({ "state": state.0 })))
}

/// Update the state of a character.
#[utoipa::path(
    put,
    path = "/state",
    tag = "characters",
    params(("id" = String, Path, description = "Character ID")),
    request_body = PutStateBody,
    responses(
        (status = 200, description = "State updated"),
        (status = 404, description = "Character not found"),
    ),
)]
pub async fn put_state(
    State(api): State<CharacterApi>,
    CharacterIdExtractor { id, .. }: CharacterIdExtractor,
    Json(body): Json<PutStateBody>,
) -> HttpResult {
    api.set_state(id, body.state).await.into_http_result()
}

/// Get the persona of a character.
#[utoipa::path(
    get,
    path = "/persona",
    tag = "characters",
    params(("id" = String, Path, description = "Character ID")),
    responses(
        (status = 200, description = "Character persona", body = Persona),
        (status = 404, description = "Character not found"),
    ),
)]
pub async fn get_persona(
    State(api): State<CharacterApi>,
    CharacterIdExtractor { id, .. }: CharacterIdExtractor,
) -> HttpResult<Persona> {
    api.get_persona(id).await.into_http_result()
}

/// Update the persona of a character.
#[utoipa::path(
    put,
    path = "/persona",
    tag = "characters",
    params(("id" = String, Path, description = "Character ID")),
    request_body = Persona,
    responses(
        (status = 200, description = "Persona updated"),
        (status = 404, description = "Character not found"),
    ),
)]
pub async fn put_persona(
    State(api): State<CharacterApi>,
    CharacterIdExtractor { id, .. }: CharacterIdExtractor,
    Json(body): Json<Persona>,
) -> HttpResult {
    api.set_persona(id, body).await.into_http_result()
}

/// Get the display name of a character.
#[utoipa::path(
    get,
    path = "/name",
    tag = "characters",
    params(("id" = String, Path, description = "Character ID")),
    responses(
        (status = 200, description = "Character name", body = PutNameBody),
        (status = 404, description = "Character not found"),
    ),
)]
pub async fn get_name(
    State(api): State<CharacterApi>,
    CharacterIdExtractor { id, .. }: CharacterIdExtractor,
) -> HttpResult<serde_json::Value> {
    let name = api.get_name(id).await?;
    Ok(Json(serde_json::json!({ "name": name })))
}

/// Update the display name of a character.
#[utoipa::path(
    put,
    path = "/name",
    tag = "characters",
    params(("id" = String, Path, description = "Character ID")),
    request_body = PutNameBody,
    responses(
        (status = 200, description = "Name updated"),
        (status = 404, description = "Character not found"),
    ),
)]
pub async fn put_name(
    State(api): State<CharacterApi>,
    CharacterIdExtractor { id, .. }: CharacterIdExtractor,
    Json(body): Json<PutNameBody>,
) -> HttpResult {
    api.set_name(id, body.name).await.into_http_result()
}

/// Attach a VRM model to a character.
#[utoipa::path(
    post,
    path = "/vrm/attach",
    tag = "characters",
    params(("id" = String, Path, description = "Character ID")),
    request_body = AttachVrmBody,
    responses(
        (status = 200, description = "VRM attached"),
        (status = 404, description = "Character not found"),
    ),
)]
pub async fn attach_vrm(
    State(api): State<CharacterApi>,
    CharacterIdExtractor { id, .. }: CharacterIdExtractor,
    Json(body): Json<AttachVrmBody>,
) -> HttpResult {
    let asset_id = AssetId::new(&body.asset_id);
    api.attach_vrm(id, asset_id).await?;
    Ok(Json(()))
}

/// Detach the VRM model from a character.
#[utoipa::path(
    delete,
    path = "/vrm",
    tag = "characters",
    params(("id" = String, Path, description = "Character ID")),
    responses(
        (status = 200, description = "VRM detached"),
        (status = 404, description = "Character not found"),
        (status = 422, description = "No VRM attached"),
    ),
)]
pub async fn detach_vrm(
    State(api): State<CharacterApi>,
    CharacterIdExtractor { id, .. }: CharacterIdExtractor,
) -> HttpResult {
    api.detach_vrm(id).await.into_http_result()
}

#[cfg(test)]
mod tests {
    use crate::tests::{assert_response, call, call_any_status, test_app};
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use bevy::prelude::*;
    use homunculus_api::character::CharacterInfo;
    use homunculus_core::prelude::{
        AssetId, AssetIdComponent, Character, CharacterId, CharacterName, CharacterState, Persona,
    };

    /// Spawns a character entity with the necessary components and runs an
    /// update so the `CharacterRegistry` observer registers it.
    fn spawn_character(app: &mut App, id: &str, name: &str, asset_id: &str) -> Entity {
        let character_id = CharacterId::new(id).unwrap();
        let entity = app
            .world_mut()
            .spawn((
                Character,
                character_id,
                CharacterName(name.to_string()),
                Name::new(name.to_string()),
                AssetIdComponent(AssetId::new(asset_id)),
                CharacterState::default(),
                Persona::default(),
            ))
            .id();
        app.update();
        entity
    }

    #[tokio::test]
    async fn test_list_characters_empty() {
        let (mut app, router) = test_app();
        let request = Request::get("/characters").body(Body::empty()).unwrap();
        assert_response::<Vec<CharacterInfo>>(&mut app, router, request, vec![]).await;
    }

    #[tokio::test]
    async fn test_list_characters_with_entry() {
        let (mut app, router) = test_app();
        spawn_character(&mut app, "elmer", "Elmer", "test:model.vrm");

        let request = Request::get("/characters").body(Body::empty()).unwrap();
        assert_response(
            &mut app,
            router,
            request,
            vec![CharacterInfo {
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
    async fn test_get_character_detail() {
        let (mut app, router) = test_app();
        spawn_character(&mut app, "elmer", "Elmer", "test:model.vrm");

        let request = Request::get("/characters/elmer")
            .body(Body::empty())
            .unwrap();
        assert_response(
            &mut app,
            router,
            request,
            super::CharacterDetail {
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
    async fn test_get_character_not_found() {
        let (mut app, router) = test_app();
        let request = Request::get("/characters/nonexistent")
            .body(Body::empty())
            .unwrap();
        let response = call_any_status(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_destroy_character() {
        let (mut app, router) = test_app();
        spawn_character(&mut app, "elmer", "Elmer", "test:model.vrm");

        let request = Request::delete("/characters/elmer")
            .body(Body::empty())
            .unwrap();
        let response = call(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_character_state() {
        let (mut app, router) = test_app();
        spawn_character(&mut app, "elmer", "Elmer", "test:model.vrm");

        let request = Request::get("/characters/elmer/state")
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
    async fn test_put_character_state() {
        let (mut app, router) = test_app();
        spawn_character(&mut app, "elmer", "Elmer", "test:model.vrm");

        let request = Request::put("/characters/elmer/state")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"state":"dancing"}"#))
            .unwrap();
        let response = call(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_character_persona() {
        let (mut app, router) = test_app();
        spawn_character(&mut app, "elmer", "Elmer", "test:model.vrm");

        let request = Request::get("/characters/elmer/persona")
            .body(Body::empty())
            .unwrap();
        assert_response(&mut app, router, request, Persona::default()).await;
    }

    #[tokio::test]
    async fn test_put_character_persona() {
        let (mut app, router) = test_app();
        spawn_character(&mut app, "elmer", "Elmer", "test:model.vrm");

        let persona = Persona {
            profile: "A cheerful assistant".to_string(),
            personality: Some("Friendly".to_string()),
            ..Default::default()
        };
        let request = Request::put("/characters/elmer/persona")
            .header("Content-Type", "application/json")
            .body(Body::from(serde_json::to_string(&persona).unwrap()))
            .unwrap();
        let response = call(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_character_name() {
        let (mut app, router) = test_app();
        spawn_character(&mut app, "elmer", "Elmer", "test:model.vrm");

        let request = Request::get("/characters/elmer/name")
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
    async fn test_put_character_name() {
        let (mut app, router) = test_app();
        spawn_character(&mut app, "elmer", "Elmer", "test:model.vrm");

        let request = Request::put("/characters/elmer/name")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"name":"New Name"}"#))
            .unwrap();
        let response = call(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_invalid_character_id_rejected() {
        let (mut app, router) = test_app();
        let request = Request::get("/characters/INVALID")
            .body(Body::empty())
            .unwrap();
        let response = call_any_status(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    /// Spawns a character entity AND creates the corresponding database row
    /// so that extension operations (which need the FK) succeed.
    fn spawn_character_with_db(app: &mut App, id: &str, name: &str, asset_id: &str) -> Entity {
        use homunculus_prefs::PrefsDatabase;
        use homunculus_prefs::character_repo::CharacterRepo;

        let db = app
            .world()
            .get_non_send_resource::<PrefsDatabase>()
            .unwrap();
        CharacterRepo::new(db)
            .create(id, asset_id, name, "{}", "{}")
            .unwrap();

        spawn_character(app, id, name, asset_id)
    }

    #[tokio::test]
    async fn test_set_and_get_extension() {
        let (mut app, router) = test_app();
        spawn_character_with_db(&mut app, "elmer", "Elmer", "test:model.vrm");

        let set_req = Request::put("/characters/elmer/extensions?mod=voicevox")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"speakerId":1}"#))
            .unwrap();
        let response = call(&mut app, router.clone(), set_req).await;
        assert_eq!(response.status(), StatusCode::OK);

        let get_req = Request::get("/characters/elmer/extensions?mod=voicevox")
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
        spawn_character_with_db(&mut app, "elmer", "Elmer", "test:model.vrm");

        // Set first
        let set_req = Request::put("/characters/elmer/extensions?mod=voicevox")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"v":1}"#))
            .unwrap();
        let response = call(&mut app, router.clone(), set_req).await;
        assert_eq!(response.status(), StatusCode::OK);

        // Delete
        let del_req = Request::delete("/characters/elmer/extensions?mod=voicevox")
            .body(Body::empty())
            .unwrap();
        let response = call(&mut app, router.clone(), del_req).await;
        assert_eq!(response.status(), StatusCode::OK);

        // Get should 404
        let get_req = Request::get("/characters/elmer/extensions?mod=voicevox")
            .body(Body::empty())
            .unwrap();
        let response = call_any_status(&mut app, router, get_req).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_extension_not_found() {
        let (mut app, router) = test_app();
        spawn_character_with_db(&mut app, "elmer", "Elmer", "test:model.vrm");

        let request = Request::get("/characters/elmer/extensions?mod=nonexistent")
            .body(Body::empty())
            .unwrap();
        let response = call_any_status(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_extension_character_not_found() {
        let (mut app, router) = test_app();

        let request = Request::get("/characters/nonexistent/extensions?mod=voicevox")
            .body(Body::empty())
            .unwrap();
        let response = call_any_status(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_extension_scoped_mod_name() {
        let (mut app, router) = test_app();
        spawn_character_with_db(&mut app, "elmer", "Elmer", "test:model.vrm");

        let set_req = Request::put("/characters/elmer/extensions?mod=%40hmcs%2Fvoicevox")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"speakerId":1}"#))
            .unwrap();
        let response = call(&mut app, router.clone(), set_req).await;
        assert_eq!(response.status(), StatusCode::OK);

        let get_req = Request::get("/characters/elmer/extensions?mod=%40hmcs%2Fvoicevox")
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
}
