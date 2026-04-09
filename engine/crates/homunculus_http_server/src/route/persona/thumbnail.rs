use axum::extract::State;
use axum::http::StatusCode;
use axum::http::header::{CACHE_CONTROL, CONTENT_TYPE};
use axum::response::{IntoResponse, Response};
use homunculus_api::persona::PersonaApi;
use std::path::Path;

use super::PersonaPath;

/// Minimal 1x1 transparent PNG (67 bytes).
const PLACEHOLDER_PNG: &[u8] = &[
    0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
    0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, // IHDR chunk
    0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, // 1x1
    0x08, 0x06, 0x00, 0x00, 0x00, 0x1F, 0x15, 0xC4, // RGBA, 8-bit
    0x89, 0x00, 0x00, 0x00, 0x0A, 0x49, 0x44, 0x41, // IDAT chunk
    0x54, 0x78, 0x9C, 0x62, 0x00, 0x00, 0x00, 0x02, // compressed data
    0x00, 0x01, 0xE5, 0x27, 0xDE, 0xFC, 0x00, 0x00, //
    0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, // IEND chunk
    0x60, 0x82,
];

/// Get persona thumbnail image.
///
/// Resolution chain:
/// 1. Custom thumbnail from `persona_metadata["thumbnail"]` path
/// 2. Default 1x1 transparent PNG placeholder
///
/// # Note
///
/// VRM-embedded thumbnail extraction is not yet implemented and is planned
/// as a follow-up enhancement.
#[utoipa::path(
    get,
    path = "/thumbnail",
    tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    responses(
        (status = 200, description = "Thumbnail image", content_type = "image/png"),
        (status = 404, description = "Persona not found"),
    ),
)]
pub async fn thumbnail(
    State(api): State<PersonaApi>,
    path: PersonaPath,
) -> Result<Response, (StatusCode, &'static str)> {
    let snap = api
        .get(path.persona_id)
        .await
        .map_err(|_| (StatusCode::NOT_FOUND, "Persona not found"))?;

    // 1. Check custom thumbnail path in metadata
    if let Some(thumbnail_value) = snap.persona.metadata.get("thumbnail")
        && let Some(thumbnail_path) = thumbnail_value.as_str()
        && let Some(response) = try_serve_file(thumbnail_path).await
    {
        return Ok(response);
    }

    // 2. TODO: Extract VRM-embedded thumbnail from VRM 1.0 glTF extensions.
    //    This requires reading the VRM file directly and parsing the thumbnail
    //    image from the glTF extensions. Skipped for now — go directly to placeholder.

    // 3. Default placeholder
    Ok(placeholder_response())
}

/// Attempts to serve a file from the given path. Returns `None` if the file
/// cannot be read or does not exist.
async fn try_serve_file(path: &str) -> Option<Response> {
    let file_path = Path::new(path);
    let bytes = tokio::fs::read(file_path).await.ok()?;
    let content_type = guess_content_type(path);
    Some(image_response(bytes, content_type))
}

/// Returns a response with the default 1x1 transparent PNG placeholder.
fn placeholder_response() -> Response {
    image_response(PLACEHOLDER_PNG.to_vec(), "image/png")
}

/// Builds an HTTP response with image bytes and the given content type.
fn image_response(bytes: Vec<u8>, content_type: &str) -> Response {
    (
        [
            (CONTENT_TYPE, content_type.to_string()),
            (CACHE_CONTROL, "no-cache".to_string()),
        ],
        bytes,
    )
        .into_response()
}

/// Guesses the MIME content type from a file extension.
fn guess_content_type(path: &str) -> &'static str {
    match Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_ascii_lowercase())
        .as_deref()
    {
        Some("png") => "image/png",
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        Some("svg") => "image/svg+xml",
        _ => "image/png",
    }
}
