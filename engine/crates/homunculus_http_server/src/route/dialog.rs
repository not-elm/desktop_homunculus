//! `/dialog` provides native OS dialog operations.

use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Response from the folder picker dialog.
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PickFolderResponse {
    /// The selected directory path, or `null` if the user cancelled.
    pub path: Option<String>,
}

/// Open a native OS directory picker dialog.
///
/// Returns the selected directory path. If the user cancels the dialog,
/// `path` will be `null`.
#[utoipa::path(
    post,
    path = "/pick-folder",
    tag = "dialog",
    responses(
        (status = 200, description = "Folder selection result", body = PickFolderResponse),
    ),
)]
pub async fn pick_folder() -> Json<PickFolderResponse> {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("Select Directory")
        .pick_folder()
        .await;

    let path = handle.map(|h| h.path().to_string_lossy().to_string());
    Json(PickFolderResponse { path })
}
