use crate::prelude::BoxedTask;
use async_channel::SendError;
use async_openai::error::OpenAIError;
use bevy::prelude::{Entity, Name};
use homunculus_core::prelude::ModModuleSource;

pub type ApiResult<T = ()> = Result<T, ApiError>;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("invalid asset path: {0:?}")]
    InvalidAssetPath(ModModuleSource),
    #[error("Not found entity")]
    EntityNotfound,
    #[error("Not found entity by name: {0}")]
    NotFoundEntityByName(Name),
    #[error("Missing vrm name for entity: {0}")]
    MissingName(Entity),
    #[error(transparent)]
    FailedTaskSend(#[from] SendError<BoxedTask>),
    #[error(transparent)]
    FailedSendCommands(#[from] async_broadcast::SendError<serde_json::Value>),
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
    #[error("Notfound preferences value; key={0}")]
    NotfoundPreferences(String),

    #[error(transparent)]
    OpenAI(#[from] OpenAIError),
    #[error("Invalid OpenAI response: {0}")]
    InvalidOpenAIResponse(String),

    #[error("Missing OpenAI response message")]
    MissingOpenAIResponseMessage,
    #[error("Not found system message VRM name: {0}")]
    NotfoundVrmSystemMessage(Name),
    #[error("Failed to execute SQL: {0}")]
    Sql(String),
}

pub trait ApiResultExt {
    fn error_if_notfound(self) -> ApiResult<Entity>;
}

impl ApiResultExt for ApiResult<Option<Entity>> {
    fn error_if_notfound(self) -> ApiResult<Entity> {
        match self? {
            Some(entity) => Ok(entity),
            None => Err(ApiError::EntityNotfound),
        }
    }
}

#[cfg(feature = "axum")]
pub mod axum {
    use crate::error::ApiError;
    use crate::prelude::ApiResult;
    use axum::Json;
    use axum::body::Body;
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
                ApiError::EntityNotfound => axum::http::StatusCode::NOT_FOUND,
                ApiError::MissingShadowPanel | ApiError::MissingName(_) => {
                    axum::http::StatusCode::BAD_REQUEST
                }
                _ => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            };
            Response::builder()
                .status(status)
                .body(Body::from(self.to_string()))
                .unwrap()
        }
    }
}
