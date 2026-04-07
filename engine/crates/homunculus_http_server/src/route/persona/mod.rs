//! `/personas` provides CRUD operations and VRM management for persona entities.

pub(crate) mod create;
pub(crate) mod delete;
pub(crate) mod events;
pub(crate) mod fields;
pub(crate) mod get;
pub(crate) mod snapshot;
pub(crate) mod state;
pub(crate) mod update;
pub(crate) mod vrm;
pub(crate) mod vrm_ops;

use axum::RequestPartsExt;
use axum::extract::{FromRequestParts, Path};
use axum::http::StatusCode;
use axum::http::request::Parts;
use bevy::prelude::{Entity, error};
use homunculus_core::prelude::PersonaId;

/// Extracts and resolves a persona `{id}` path parameter to `(Entity, PersonaId)`.
///
/// Returns 404 if the persona is not found in the [`PersonaIndex`].
pub struct PersonaPath {
    pub entity: Entity,
    pub persona_id: PersonaId,
}

impl FromRequestParts<crate::state::HttpState> for PersonaPath {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &crate::state::HttpState,
    ) -> Result<Self, Self::Rejection> {
        let Path(id): Path<String> = parts.extract().await.map_err(|e| {
            error!("Failed to extract persona id from path: {e}");
            (StatusCode::BAD_REQUEST, "Invalid persona id")
        })?;

        let persona_id =
            PersonaId::validate(&id).map_err(|_| (StatusCode::BAD_REQUEST, "Invalid persona id"))?;

        let entity = state
            .persona
            .resolve(persona_id.clone())
            .await
            .map_err(|_| (StatusCode::NOT_FOUND, "Persona not found"))?;

        Ok(PersonaPath { entity, persona_id })
    }
}
