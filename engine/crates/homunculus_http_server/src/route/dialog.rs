//! `/dialog` provides native OS dialog operations.

use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A file type filter for native file dialogs.
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FileFilter {
    /// Display name for the filter (e.g. "Images").
    pub name: String,
    /// File extensions without leading dot (e.g. `["png", "jpg"]`).
    pub extensions: Vec<String>,
}

/// Request body for file picker dialogs.
#[derive(Serialize, Deserialize, Debug, Clone, Default, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PickFileRequest {
    /// Dialog window title. Defaults to `"Select File"` or `"Select Files"`.
    pub title: Option<String>,
    /// File type filters shown in the dialog.
    pub filters: Option<Vec<FileFilter>>,
    /// Initial directory to open the dialog in.
    pub default_path: Option<String>,
}

/// Request body for folder picker dialog.
#[derive(Serialize, Deserialize, Debug, Clone, Default, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PickFolderRequest {
    /// Dialog window title. Defaults to `"Select Directory"`.
    pub title: Option<String>,
    /// Initial directory to open the dialog in.
    pub default_path: Option<String>,
}

/// Response from the single file picker dialog.
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PickFileResponse {
    /// The selected file path, or `null` if the user cancelled.
    pub path: Option<String>,
}

/// Response from the multi-file picker dialog.
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PickFilesResponse {
    /// The selected file paths. Empty if the user cancelled.
    pub paths: Vec<String>,
}

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
/// `path` will be `null`. Optionally accepts a title and default directory.
#[utoipa::path(
    post,
    path = "/pick-folder",
    tag = "dialog",
    request_body(content = Option<PickFolderRequest>, content_type = "application/json"),
    responses(
        (status = 200, description = "Folder selection result", body = PickFolderResponse),
    ),
)]
pub async fn pick_folder(body: Option<Json<PickFolderRequest>>) -> Json<PickFolderResponse> {
    let req = body.map(|Json(r)| r).unwrap_or_default();
    let dialog = build_file_dialog(
        req.title.as_deref(),
        "Select Directory",
        None,
        req.default_path.as_deref(),
    );
    let handle = dialog.pick_folder().await;
    let path = handle.map(|h| h.path().to_string_lossy().to_string());
    Json(PickFolderResponse { path })
}

/// Open a native OS single-file picker dialog.
///
/// Returns the selected file path. If the user cancels the dialog,
/// `path` will be `null`. Optionally accepts filters and a default directory.
#[utoipa::path(
    post,
    path = "/pick-file",
    tag = "dialog",
    request_body(content = Option<PickFileRequest>, content_type = "application/json"),
    responses(
        (status = 200, description = "File selection result", body = PickFileResponse),
    ),
)]
pub async fn pick_file(body: Option<Json<PickFileRequest>>) -> Json<PickFileResponse> {
    let req = body.map(|Json(r)| r).unwrap_or_default();
    let dialog = build_file_dialog(
        req.title.as_deref(),
        "Select File",
        req.filters.as_deref(),
        req.default_path.as_deref(),
    );
    let handle = dialog.pick_file().await;
    let path = handle.map(|h| h.path().to_string_lossy().to_string());
    Json(PickFileResponse { path })
}

/// Open a native OS multi-file picker dialog.
///
/// Returns the selected file paths. If the user cancels the dialog,
/// `paths` will be an empty array. Optionally accepts filters and a default directory.
#[utoipa::path(
    post,
    path = "/pick-files",
    tag = "dialog",
    request_body(content = Option<PickFileRequest>, content_type = "application/json"),
    responses(
        (status = 200, description = "Multi-file selection result", body = PickFilesResponse),
    ),
)]
pub async fn pick_files(body: Option<Json<PickFileRequest>>) -> Json<PickFilesResponse> {
    let req = body.map(|Json(r)| r).unwrap_or_default();
    let dialog = build_file_dialog(
        req.title.as_deref(),
        "Select Files",
        req.filters.as_deref(),
        req.default_path.as_deref(),
    );
    let handles = dialog.pick_files().await;
    let paths = handles
        .unwrap_or_default()
        .iter()
        .map(|h| h.path().to_string_lossy().to_string())
        .collect();
    Json(PickFilesResponse { paths })
}

/// Build an `AsyncFileDialog` with optional title, filters, and default path.
fn build_file_dialog(
    title: Option<&str>,
    default_title: &str,
    filters: Option<&[FileFilter]>,
    default_path: Option<&str>,
) -> rfd::AsyncFileDialog {
    let mut dialog = rfd::AsyncFileDialog::new().set_title(title.unwrap_or(default_title));
    if let Some(filters) = filters {
        for f in filters {
            let ext_refs: Vec<&str> = f.extensions.iter().map(String::as_str).collect();
            dialog = dialog.add_filter(&f.name, &ext_refs);
        }
    }
    if let Some(path) = default_path {
        dialog = dialog.set_directory(path);
    }
    dialog
}
