use crate::prelude::BoxedTask;
use async_channel::SendError;
use bevy::prelude::{Entity, Name};
use homunculus_core::prelude::{AssetId, AssetType};

pub type ApiResult<T = ()> = Result<T, ApiError>;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Not found entity")]
    EntityNotFound,
    #[error("Not found entity by name: {0}")]
    NotFoundEntityByName(Name),
    #[error("Missing vrm name for entity: {0}")]
    MissingName(Entity),
    #[error(transparent)]
    FailedTaskSend(#[from] SendError<BoxedTask>),
    #[error(transparent)]
    FailedSendSignal(#[from] async_broadcast::SendError<serde_json::Value>),
    #[error(transparent)]
    FailedTaskReceive(#[from] async_channel::RecvError),
    #[error("Failed to load preference: {0}")]
    FailedLoad(String),
    #[error("Failed to save preference: {0}")]
    FailedSave(String),
    #[error("Failed to convert to global viewport")]
    FailedToGlobalViewport,
    #[error("Failed to convert to world position")]
    FailedToWorldPosition,
    #[error("Missing shadow panel")]
    MissingShadowPanel,
    #[error("Not found preferences value; key={0}")]
    NotFoundPreferences(String),

    #[error("Failed to execute SQL: {0}")]
    Sql(String),
    #[error("Webview not found: {0}")]
    WebviewNotFound(Entity),
    #[error("Mod '{0}' not found")]
    ModNotFound(String),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Cannot pause: BGM is not playing")]
    BgmNotPlaying,
    #[error("Cannot resume: BGM is not paused")]
    BgmNotPaused,
    #[error("Asset not found: {0}")]
    AssetNotFound(AssetId),
    #[error("Asset type mismatch for '{id}': expected {expected:?}, got {actual:?}")]
    AssetTypeMismatch {
        id: AssetId,
        expected: AssetType,
        actual: AssetType,
    },
    #[error("Character not found: {0}")]
    CharacterNotFound(String),
    #[error("VRM not attached to character: {0}")]
    VrmNotAttached(String),
    #[error("Invalid character ID: {0}")]
    InvalidCharacterId(String),
    #[error("VRM initialization timed out for entity: {0:?}")]
    VrmInitTimeout(Entity),
}

pub trait ApiResultExt {
    fn error_if_notfound(self) -> ApiResult<Entity>;
}

impl ApiResultExt for ApiResult<Option<Entity>> {
    fn error_if_notfound(self) -> ApiResult<Entity> {
        match self? {
            Some(entity) => Ok(entity),
            None => Err(ApiError::EntityNotFound),
        }
    }
}

#[cfg(feature = "axum")]
pub mod axum {
    use crate::error::ApiError;
    use crate::prelude::ApiResult;
    use axum::Json;
    use axum::response::{IntoResponse, Response};
    use serde::Serialize;

    pub type HttpResult<T = ()> = bevy::prelude::Result<Json<T>, ApiError>;

    pub trait IntoHttpResult<T: Serialize> {
        fn into_http_result(self) -> HttpResult<T>;
    }

    impl<T: Serialize> IntoHttpResult<T> for ApiResult<T> {
        fn into_http_result(self) -> HttpResult<T> {
            self.map(Json)
        }
    }

    impl IntoResponse for ApiError {
        fn into_response(self) -> Response {
            let status = match self {
                ApiError::EntityNotFound
                | ApiError::WebviewNotFound(_)
                | ApiError::ModNotFound(_)
                | ApiError::AssetNotFound(_)
                | ApiError::NotFoundPreferences(_)
                | ApiError::CharacterNotFound(_) => axum::http::StatusCode::NOT_FOUND,
                ApiError::InvalidInput(_)
                | ApiError::AssetTypeMismatch { .. }
                | ApiError::InvalidCharacterId(_) => axum::http::StatusCode::BAD_REQUEST,
                ApiError::BgmNotPlaying | ApiError::BgmNotPaused => {
                    axum::http::StatusCode::CONFLICT
                }
                ApiError::VrmNotAttached(_) => axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                ApiError::MissingShadowPanel | ApiError::MissingName(_) => {
                    axum::http::StatusCode::BAD_REQUEST
                }
                _ => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            };
            (
                status,
                Json(serde_json::json!({ "error": self.to_string() })),
            )
                .into_response()
        }
    }
}
