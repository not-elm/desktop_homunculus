use crate::error::DownloadError;
use futures_lite::StreamExt;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use whisper_rs::WhisperContext;

/// Shared newtype for HTTP state.
#[derive(Clone)]
pub struct SharedSttModelCache(pub Arc<tokio::sync::Mutex<SttModelCache>>);

impl Default for SharedSttModelCache {
    fn default() -> Self {
        Self(Arc::new(tokio::sync::Mutex::new(SttModelCache::default())))
    }
}

impl SharedSttModelCache {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Cache of loaded WhisperContext instances with download-in-progress tracking.
#[derive(Default)]
pub struct SttModelCache {
    pub contexts: HashMap<SttModelSize, Arc<WhisperContext>>,
    pub downloading: HashSet<SttModelSize>,
}

impl SttModelCache {
    pub fn get_context(&self, size: SttModelSize) -> Option<Arc<WhisperContext>> {
        self.contexts.get(&size).cloned()
    }

    pub fn insert_context(&mut self, size: SttModelSize, ctx: Arc<WhisperContext>) {
        self.contexts.insert(size, ctx);
    }

    pub fn mark_downloading(&mut self, size: SttModelSize) -> bool {
        self.downloading.insert(size)
    }

    pub fn unmark_downloading(&mut self, size: SttModelSize) {
        self.downloading.remove(&size);
    }

    pub fn is_downloading(&self, size: SttModelSize) -> bool {
        self.downloading.contains(&size)
    }
}

/// Whisper model size variants.
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "lowercase")]
pub enum SttModelSize {
    Tiny,
    Base,
    #[default]
    Small,
    Medium,
}

impl SttModelSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Tiny => "tiny",
            Self::Base => "base",
            Self::Small => "small",
            Self::Medium => "medium",
        }
    }

    pub fn expected_size(&self) -> u64 {
        match self {
            Self::Tiny => 32_506_944,
            Self::Base => 59_846_080,
            Self::Small => 189_804_736,
            Self::Medium => 491_766_272,
        }
    }
}

/// Load a `WhisperContext` from the model file on disk.
///
/// This is a blocking operation and should be called from a blocking task.
pub fn load_whisper_context(size: SttModelSize) -> Result<Arc<WhisperContext>, String> {
    let path = model_path(size);
    WhisperContext::new_with_params(&path, whisper_rs::WhisperContextParameters::default())
        .map(Arc::new)
        .map_err(|e| e.to_string())
}

/// Returns the path to the model file.
pub fn model_path(size: SttModelSize) -> PathBuf {
    homunculus_utils::path::homunculus_dir()
        .join("models")
        .join(format!("ggml-{}-q5_1.bin", size.as_str()))
}

/// Checks whether the model has been downloaded.
pub fn is_model_available(size: SttModelSize) -> bool {
    let path = model_path(size);
    match std::fs::metadata(&path) {
        Ok(meta) => meta.len() == size.expected_size(),
        Err(_) => false,
    }
}

/// Returns a list of downloaded models.
pub fn list_available_models() -> Vec<(SttModelSize, u64, PathBuf)> {
    let sizes = [
        SttModelSize::Tiny,
        SttModelSize::Base,
        SttModelSize::Small,
        SttModelSize::Medium,
    ];
    sizes
        .iter()
        .filter_map(|&size| {
            if is_model_available(size) {
                let path = model_path(size);
                Some((size, size.expected_size(), path))
            } else {
                None
            }
        })
        .collect()
}

/// Downloads a model from HuggingFace.
pub async fn download_model(
    size: SttModelSize,
    cancel: &tokio_util::sync::CancellationToken,
) -> Result<(), DownloadError> {
    let url = model_url(size);
    let path = model_path(size);
    ensure_model_dir(&path)?;

    let client = reqwest::Client::new();
    let response = fetch_model(&client, &url).await?;

    let tmp_path = path.with_extension("bin.tmp");
    stream_to_file(response, &tmp_path, cancel).await?;

    tokio::fs::rename(&tmp_path, &path)
        .await
        .map_err(DownloadError::Io)?;

    verify_model_size(&path, size.expected_size()).await?;

    Ok(())
}

fn model_url(size: SttModelSize) -> String {
    format!(
        "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-{}-q5_1.bin",
        size.as_str()
    )
}

fn ensure_model_dir(path: &Path) -> Result<(), DownloadError> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    Ok(())
}

async fn fetch_model(
    client: &reqwest::Client,
    url: &str,
) -> Result<reqwest::Response, DownloadError> {
    let response = client
        .get(url)
        .send()
        .await
        .map_err(DownloadError::Request)?;

    if !response.status().is_success() {
        return Err(DownloadError::HttpStatus(response.status().as_u16()));
    }

    Ok(response)
}

async fn stream_to_file(
    response: reqwest::Response,
    tmp_path: &Path,
    cancel: &tokio_util::sync::CancellationToken,
) -> Result<(), DownloadError> {
    let mut file = tokio::fs::File::create(tmp_path)
        .await
        .map_err(DownloadError::Io)?;

    let mut stream = response.bytes_stream();
    while let Some(chunk) = stream.next().await {
        if cancel.is_cancelled() {
            drop(file);
            let _ = tokio::fs::remove_file(tmp_path).await;
            return Err(DownloadError::Cancelled);
        }
        let chunk = chunk.map_err(DownloadError::Request)?;
        file.write_all(&chunk).await.map_err(DownloadError::Io)?;
    }

    file.flush().await.map_err(DownloadError::Io)?;
    Ok(())
}

async fn verify_model_size(path: &Path, expected: u64) -> Result<(), DownloadError> {
    let actual = tokio::fs::metadata(path)
        .await
        .map_err(DownloadError::Io)?
        .len();
    if actual != expected {
        let _ = tokio::fs::remove_file(path).await;
        return Err(DownloadError::SizeMismatch { expected, actual });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn model_size_as_str() {
        assert_eq!(SttModelSize::Tiny.as_str(), "tiny");
        assert_eq!(SttModelSize::Base.as_str(), "base");
        assert_eq!(SttModelSize::Small.as_str(), "small");
        assert_eq!(SttModelSize::Medium.as_str(), "medium");
    }

    #[test]
    fn model_size_default_is_small() {
        assert_eq!(SttModelSize::default(), SttModelSize::Small);
    }

    #[test]
    fn model_size_serde() {
        let json = serde_json::to_string(&SttModelSize::Small).unwrap();
        assert_eq!(json, r#""small""#);
        let parsed: SttModelSize = serde_json::from_str(r#""tiny""#).unwrap();
        assert_eq!(parsed, SttModelSize::Tiny);
    }

    #[test]
    fn model_path_format() {
        let path = model_path(SttModelSize::Small);
        let path_str = path.to_string_lossy();
        assert!(path_str.contains("models"));
        assert!(path_str.ends_with("ggml-small-q5_1.bin"));
    }

    #[test]
    fn model_not_available_when_missing() {
        assert!(!is_model_available(SttModelSize::Medium));
    }

    #[test]
    fn cache_get_returns_none_for_empty() {
        let cache = SttModelCache::default();
        assert!(cache.get_context(SttModelSize::Small).is_none());
    }

    #[test]
    fn cache_mark_downloading_singleflight() {
        let mut cache = SttModelCache::default();
        assert!(cache.mark_downloading(SttModelSize::Small));
        assert!(!cache.mark_downloading(SttModelSize::Small));
        assert!(cache.is_downloading(SttModelSize::Small));
        cache.unmark_downloading(SttModelSize::Small);
        assert!(!cache.is_downloading(SttModelSize::Small));
    }
}
