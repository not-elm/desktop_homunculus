use axum::extract::State;
use homunculus_api::prelude::AppApi;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};

/// Exit the application gracefully.
#[utoipa::path(
    post,
    path = "/exit",
    tag = "app",
    responses(
        (status = 200, description = "Application exit initiated"),
        (status = 500, description = "Internal server error"),
    ),
)]
pub async fn exit(State(api): State<AppApi>) -> HttpResult {
    api.exit().await.into_http_result()
}
