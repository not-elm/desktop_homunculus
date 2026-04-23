//! `/personas` provides CRUD operations and VRM management for persona entities.

pub(crate) mod create;
pub(crate) mod delete;
pub(crate) mod events;
pub(crate) mod fields;
pub(crate) mod get;
pub(crate) mod snapshot;
pub(crate) mod spawn;
pub(crate) mod state;
pub(crate) mod stream;
pub(crate) mod update;
pub(crate) mod vrm;

use axum::RequestPartsExt;
use axum::extract::{FromRequestParts, Path};
use axum::http::StatusCode;
use axum::http::request::Parts;
use bevy::prelude::{Entity, error};
use homunculus_core::prelude::PersonaId;

/// Extracts a persona `{id}` path parameter and validates the ID format.
///
/// Does **not** require the persona to be spawned as an ECS entity.
/// Used by identity endpoints (GET/PATCH/DELETE persona, field getters/setters)
/// that operate on persona data rather than ECS state.
pub struct PersonaPath {
    pub persona_id: PersonaId,
}

impl FromRequestParts<crate::state::HttpState> for PersonaPath {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &crate::state::HttpState,
    ) -> Result<Self, Self::Rejection> {
        let Path(id): Path<String> = parts.extract().await.map_err(|e| {
            error!("Failed to extract persona id from path: {e}");
            (StatusCode::BAD_REQUEST, "Invalid persona id")
        })?;

        let persona_id = PersonaId::validate(&id)
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid persona id"))?;

        Ok(PersonaPath { persona_id })
    }
}

/// Extracts and resolves a persona `{id}` path parameter to `(Entity, PersonaId)`.
///
/// Requires the persona to be spawned as an ECS entity. Returns 404 if the
/// persona is not found in the [`PersonaIndex`](homunculus_core::prelude::PersonaIndex).
/// Used by VRM/state/events/transform endpoints that need a live ECS entity.
#[allow(dead_code)]
pub struct SpawnedPersonaPath {
    pub entity: Entity,
    pub persona_id: PersonaId,
}

impl FromRequestParts<crate::state::HttpState> for SpawnedPersonaPath {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &crate::state::HttpState,
    ) -> Result<Self, Self::Rejection> {
        let Path(id): Path<String> = parts.extract().await.map_err(|e| {
            error!("Failed to extract persona id from path: {e}");
            (StatusCode::BAD_REQUEST, "Invalid persona id")
        })?;

        let persona_id = PersonaId::validate(&id)
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid persona id"))?;

        let entity = state
            .persona
            .resolve(persona_id.clone())
            .await
            .map_err(|_| (StatusCode::NOT_FOUND, "Persona not found"))?;

        Ok(SpawnedPersonaPath { entity, persona_id })
    }
}
