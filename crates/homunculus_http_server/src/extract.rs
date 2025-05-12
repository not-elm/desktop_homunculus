use axum::RequestPartsExt;
use axum::extract::{FromRequestParts, Path};
use axum::http::StatusCode;
use axum::http::request::Parts;
use bevy::prelude::{Deref, Entity};

#[derive(Deref)]
pub struct EntityId(pub Entity);

impl<S: Send + Sync> FromRequestParts<S> for EntityId {
    type Rejection = (StatusCode, &'static str);
    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let entity: Path<u64> = parts
            .extract()
            .await
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid entity"))?;
        Ok(EntityId(Entity::from_bits(entity.0)))
    }
}
