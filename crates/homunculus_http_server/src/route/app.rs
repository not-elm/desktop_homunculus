use axum::extract::State;
use homunculus_api::prelude::AppApi;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};

/// Exists the application without any problems.
///
/// ## Path
///
/// `POST /app/exit`
pub async fn exit(State(api): State<AppApi>) -> HttpResult {
    api.exit().await.into_http_result()
}
