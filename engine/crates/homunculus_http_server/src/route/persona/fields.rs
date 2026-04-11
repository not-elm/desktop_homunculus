use axum::Json;
use axum::extract::State;
use homunculus_api::persona::{PersonaApi, PersonaSnapshot};
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_api::vrm::OptionalTransform;
use homunculus_core::prelude::Gender;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

use super::{PersonaPath, SpawnedPersonaPath};

// ---------------------------------------------------------------------------
// Name
// ---------------------------------------------------------------------------

/// Get the display name of a persona.
#[utoipa::path(
    get, path = "/name", tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    responses((status = 200, body = NameBody), (status = 404)),
)]
pub async fn get_name(State(api): State<PersonaApi>, path: PersonaPath) -> HttpResult<NameBody> {
    let snap = api.get(path.persona_id).await?;
    Ok(Json(NameBody {
        name: snap.persona.name,
    }))
}

/// Set the display name of a persona.
#[utoipa::path(
    put, path = "/name", tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    request_body = NameBody,
    responses((status = 200, body = PersonaSnapshot), (status = 404)),
)]
pub async fn put_name(
    State(api): State<PersonaApi>,
    path: PersonaPath,
    Json(body): Json<NameBody>,
) -> HttpResult<PersonaSnapshot> {
    api.set_name(path.persona_id, body.name.unwrap_or_default())
        .await
        .into_http_result()
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct NameBody {
    pub name: Option<String>,
}

// ---------------------------------------------------------------------------
// Age
// ---------------------------------------------------------------------------

/// Get the age of a persona.
#[utoipa::path(
    get, path = "/age", tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    responses((status = 200, body = AgeBody), (status = 404)),
)]
pub async fn get_age(State(api): State<PersonaApi>, path: PersonaPath) -> HttpResult<AgeBody> {
    let snap = api.get(path.persona_id).await?;
    Ok(Json(AgeBody {
        age: snap.persona.age,
    }))
}

/// Set the age of a persona.
#[utoipa::path(
    put, path = "/age", tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    request_body = AgeBody,
    responses((status = 200, body = PersonaSnapshot), (status = 404)),
)]
pub async fn put_age(
    State(api): State<PersonaApi>,
    path: PersonaPath,
    Json(body): Json<AgeBody>,
) -> HttpResult<PersonaSnapshot> {
    match body.age {
        Some(age) => api.set_age(path.persona_id, age).await,
        None => api.clear_age(path.persona_id).await,
    }
    .into_http_result()
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AgeBody {
    pub age: Option<u32>,
}

// ---------------------------------------------------------------------------
// Gender
// ---------------------------------------------------------------------------

/// Get the gender of a persona.
#[utoipa::path(
    get, path = "/gender", tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    responses((status = 200, body = GenderBody), (status = 404)),
)]
pub async fn get_gender(
    State(api): State<PersonaApi>,
    path: PersonaPath,
) -> HttpResult<GenderBody> {
    let snap = api.get(path.persona_id).await?;
    Ok(Json(GenderBody {
        gender: snap.persona.gender,
    }))
}

/// Set the gender of a persona.
#[utoipa::path(
    put, path = "/gender", tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    request_body = GenderBody,
    responses((status = 200, body = PersonaSnapshot), (status = 404)),
)]
pub async fn put_gender(
    State(api): State<PersonaApi>,
    path: PersonaPath,
    Json(body): Json<GenderBody>,
) -> HttpResult<PersonaSnapshot> {
    api.set_gender(path.persona_id, body.gender)
        .await
        .into_http_result()
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct GenderBody {
    pub gender: Gender,
}

// ---------------------------------------------------------------------------
// First-person pronoun
// ---------------------------------------------------------------------------

/// Get the first-person pronoun of a persona.
#[utoipa::path(
    get, path = "/first-person-pronoun", tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    responses((status = 200, body = PronounBody), (status = 404)),
)]
pub async fn get_first_person_pronoun(
    State(api): State<PersonaApi>,
    path: PersonaPath,
) -> HttpResult<PronounBody> {
    let snap = api.get(path.persona_id).await?;
    Ok(Json(PronounBody {
        first_person_pronoun: snap.persona.first_person_pronoun,
    }))
}

/// Set the first-person pronoun of a persona.
#[utoipa::path(
    put, path = "/first-person-pronoun", tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    request_body = PronounBody,
    responses((status = 200, body = PersonaSnapshot), (status = 404)),
)]
pub async fn put_first_person_pronoun(
    State(api): State<PersonaApi>,
    path: PersonaPath,
    Json(body): Json<PronounBody>,
) -> HttpResult<PersonaSnapshot> {
    api.set_first_person_pronoun(
        path.persona_id,
        body.first_person_pronoun.unwrap_or_default(),
    )
    .await
    .into_http_result()
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PronounBody {
    pub first_person_pronoun: Option<String>,
}

// ---------------------------------------------------------------------------
// Profile
// ---------------------------------------------------------------------------

/// Get the profile of a persona.
#[utoipa::path(
    get, path = "/profile", tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    responses((status = 200, body = ProfileBody), (status = 404)),
)]
pub async fn get_profile(
    State(api): State<PersonaApi>,
    path: PersonaPath,
) -> HttpResult<ProfileBody> {
    let snap = api.get(path.persona_id).await?;
    Ok(Json(ProfileBody {
        profile: snap.persona.profile,
    }))
}

/// Set the profile of a persona.
#[utoipa::path(
    put, path = "/profile", tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    request_body = ProfileBody,
    responses((status = 200, body = PersonaSnapshot), (status = 404)),
)]
pub async fn put_profile(
    State(api): State<PersonaApi>,
    path: PersonaPath,
    Json(body): Json<ProfileBody>,
) -> HttpResult<PersonaSnapshot> {
    api.set_profile(path.persona_id, body.profile)
        .await
        .into_http_result()
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ProfileBody {
    pub profile: String,
}

// ---------------------------------------------------------------------------
// Personality
// ---------------------------------------------------------------------------

/// Get the personality of a persona.
#[utoipa::path(
    get, path = "/personality", tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    responses((status = 200, body = PersonalityBody), (status = 404)),
)]
pub async fn get_personality(
    State(api): State<PersonaApi>,
    path: PersonaPath,
) -> HttpResult<PersonalityBody> {
    let snap = api.get(path.persona_id).await?;
    Ok(Json(PersonalityBody {
        personality: snap.persona.personality,
    }))
}

/// Set the personality of a persona.
#[utoipa::path(
    put, path = "/personality", tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    request_body = PersonalityBody,
    responses((status = 200, body = PersonaSnapshot), (status = 404)),
)]
pub async fn put_personality(
    State(api): State<PersonaApi>,
    path: PersonaPath,
    Json(body): Json<PersonalityBody>,
) -> HttpResult<PersonaSnapshot> {
    api.set_personality(path.persona_id, body.personality.unwrap_or_default())
        .await
        .into_http_result()
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct PersonalityBody {
    pub personality: Option<String>,
}

// ---------------------------------------------------------------------------
// Metadata
// ---------------------------------------------------------------------------

/// Get the metadata of a persona.
#[utoipa::path(
    get, path = "/metadata", tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    responses((status = 200, body = MetadataBody), (status = 404)),
)]
pub async fn get_metadata(
    State(api): State<PersonaApi>,
    path: PersonaPath,
) -> HttpResult<MetadataBody> {
    let snap = api.get(path.persona_id).await?;
    Ok(Json(MetadataBody {
        metadata: snap.persona.metadata,
    }))
}

/// Set the metadata of a persona.
#[utoipa::path(
    put, path = "/metadata", tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    request_body = MetadataBody,
    responses((status = 200, body = PersonaSnapshot), (status = 404)),
)]
pub async fn put_metadata(
    State(api): State<PersonaApi>,
    path: PersonaPath,
    Json(body): Json<MetadataBody>,
) -> HttpResult<PersonaSnapshot> {
    api.set_metadata(path.persona_id, body.metadata)
        .await
        .into_http_result()
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct MetadataBody {
    #[schema(value_type = std::collections::HashMap<String, Object>)]
    pub metadata: HashMap<String, serde_json::Value>,
}

// ---------------------------------------------------------------------------
// Transform
// ---------------------------------------------------------------------------

/// Get the transform of a persona.
#[utoipa::path(
    get, path = "/transform", tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    responses((status = 200, body = OptionalTransform), (status = 404)),
)]
pub async fn get_transform(
    State(entities): State<homunculus_api::prelude::EntitiesApi>,
    path: SpawnedPersonaPath,
) -> HttpResult<bevy::prelude::Transform> {
    entities.transform(path.entity).await.into_http_result()
}

/// Set the transform of a persona.
#[utoipa::path(
    put, path = "/transform", tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    request_body = OptionalTransform,
    responses((status = 200, body = OptionalTransform), (status = 404)),
)]
pub async fn put_transform(
    State(entities): State<homunculus_api::prelude::EntitiesApi>,
    path: SpawnedPersonaPath,
    Json(body): Json<homunculus_api::vrm::OptionalTransform>,
) -> HttpResult<Option<bevy::prelude::Transform>> {
    entities
        .set_transform(path.entity, body)
        .await
        .into_http_result()
}

// ---------------------------------------------------------------------------
// Thumbnail
// ---------------------------------------------------------------------------

/// Get the thumbnail asset ID of a persona.
#[utoipa::path(
    get, path = "/thumbnail", tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    responses((status = 200, body = ThumbnailBody), (status = 404)),
)]
pub async fn get_thumbnail(
    State(api): State<PersonaApi>,
    path: PersonaPath,
) -> HttpResult<ThumbnailBody> {
    let thumbnail = api.get_thumbnail(path.persona_id).await?;
    Ok(Json(ThumbnailBody { thumbnail }))
}

/// Set (or clear) the thumbnail asset ID of a persona.
///
/// Sending `{ "thumbnail": null }` clears the field.
#[utoipa::path(
    put, path = "/thumbnail", tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    request_body = ThumbnailBody,
    responses((status = 200, body = PersonaSnapshot), (status = 404)),
)]
pub async fn put_thumbnail(
    State(api): State<PersonaApi>,
    path: PersonaPath,
    Json(body): Json<ThumbnailBody>,
) -> HttpResult<PersonaSnapshot> {
    match body.thumbnail {
        Some(asset_id) => api.set_thumbnail(path.persona_id, asset_id).await,
        None => api.clear_thumbnail(path.persona_id).await,
    }
    .into_http_result()
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ThumbnailBody {
    pub thumbnail: Option<String>,
}

#[cfg(test)]
mod tests {
    use crate::tests::{call_any_status, test_app};
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use homunculus_api::persona::PersonaSnapshot;
    use http_body_util::BodyExt;

    async fn create_persona(
        app: &mut bevy::prelude::App,
        router: axum::Router,
        body: &'static str,
    ) {
        let request = Request::post("/personas")
            .header("content-type", "application/json")
            .body(Body::from(body))
            .unwrap();
        let response = call_any_status(app, router, request).await;
        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_get_thumbnail_returns_null_when_unset() {
        let (mut app, router) = test_app();
        create_persona(&mut app, router.clone(), r#"{"id":"p1"}"#).await;

        let request = Request::get("/personas/p1/thumbnail")
            .body(Body::empty())
            .unwrap();
        let response = call_any_status(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let parsed: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(parsed, serde_json::json!({ "thumbnail": null }));
    }

    #[tokio::test]
    async fn test_get_thumbnail_returns_value_when_set() {
        let (mut app, router) = test_app();
        create_persona(
            &mut app,
            router.clone(),
            r#"{"id":"p2","thumbnail":"image:a:b"}"#,
        )
        .await;

        let request = Request::get("/personas/p2/thumbnail")
            .body(Body::empty())
            .unwrap();
        let response = call_any_status(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let parsed: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(parsed, serde_json::json!({ "thumbnail": "image:a:b" }));
    }

    #[tokio::test]
    async fn test_get_thumbnail_404_when_persona_missing() {
        let (mut app, router) = test_app();

        let request = Request::get("/personas/missing/thumbnail")
            .body(Body::empty())
            .unwrap();
        let response = call_any_status(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_put_thumbnail_sets_and_returns_snapshot() {
        let (mut app, router) = test_app();
        create_persona(&mut app, router.clone(), r#"{"id":"p3"}"#).await;

        let request = Request::builder()
            .method("PUT")
            .uri("/personas/p3/thumbnail")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"thumbnail":"image:new:id"}"#))
            .unwrap();
        let response = call_any_status(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let snap: PersonaSnapshot = serde_json::from_slice(&body).unwrap();
        assert_eq!(snap.persona.thumbnail.as_deref(), Some("image:new:id"));
    }

    #[tokio::test]
    async fn test_put_thumbnail_null_clears_and_returns_snapshot() {
        let (mut app, router) = test_app();
        create_persona(
            &mut app,
            router.clone(),
            r#"{"id":"p4","thumbnail":"image:old:id"}"#,
        )
        .await;

        let request = Request::builder()
            .method("PUT")
            .uri("/personas/p4/thumbnail")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"thumbnail":null}"#))
            .unwrap();
        let response = call_any_status(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let snap: PersonaSnapshot = serde_json::from_slice(&body).unwrap();
        assert_eq!(snap.persona.thumbnail, None);
    }

    #[tokio::test]
    async fn test_put_thumbnail_404_when_persona_missing() {
        let (mut app, router) = test_app();

        let request = Request::builder()
            .method("PUT")
            .uri("/personas/missing/thumbnail")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"thumbnail":"image:a:b"}"#))
            .unwrap();
        let response = call_any_status(&mut app, router, request).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
