use crate::error::DownloadError;
use futures_lite::StreamExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio_util::sync::CancellationToken;
use whisper_rs::WhisperContext;

/// Shared newtype for HTTP state.
#[derive(Clone)]
pub struct SharedSttModelCache(pub Arc<tokio::sync::Mutex<SttModelCache>>);

impl SharedSttModelCache {
    /// Creates a new `SharedSttModelCache` with the given parent cancellation token.
    ///
    /// The parent token enables bulk cancellation of all in-progress downloads
    /// (e.g., on app shutdown). Each download derives a child token from this parent.
    pub fn new(parent: CancellationToken) -> Self {
        Self(Arc::new(tokio::sync::Mutex::new(SttModelCache {
            contexts: HashMap::new(),
            downloading: HashMap::new(),
            parent,
        })))
    }
}

/// Progress state for model downloads, sent via watch channel.
#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadProgress {
    pub downloaded_bytes: u64,
    pub total_bytes: u64,
    pub percentage: f64,
}

/// Cache of loaded WhisperContext instances with download-in-progress tracking.
pub struct SttModelCache {
    pub contexts: HashMap<SttModelSize, Arc<WhisperContext>>,
    /// In-progress downloads keyed by model size. Each value is a child `CancellationToken`
    /// derived from `parent`, enabling per-model or bulk cancellation.
    downloading: HashMap<SttModelSize, CancellationToken>,
    /// Parent cancellation token. Cancelling this propagates to all child download tokens.
    parent: CancellationToken,
}

impl SttModelCache {
    pub fn get_context(&self, size: SttModelSize) -> Option<Arc<WhisperContext>> {
        self.contexts.get(&size).cloned()
    }

    pub fn insert_context(&mut self, size: SttModelSize, ctx: Arc<WhisperContext>) {
        self.contexts.insert(size, ctx);
    }

    /// Marks a model as downloading. Creates a child cancellation token derived from
    /// the parent and stores it in the map. Returns a clone for the download task.
    ///
    /// Returns `None` if a download for this size is already in progress.
    pub fn mark_downloading(&mut self, size: SttModelSize) -> Option<CancellationToken> {
        if self.downloading.contains_key(&size) {
            return None;
        }
        let token = self.parent.child_token();
        self.downloading.insert(size, token.clone());
        Some(token)
    }

    pub fn unmark_downloading(&mut self, size: SttModelSize) {
        self.downloading.remove(&size);
    }

    pub fn is_downloading(&self, size: SttModelSize) -> bool {
        self.downloading.contains_key(&size)
    }

    /// Gets a clone of the download cancellation token for the given model size.
    pub fn get_download_token(&self, size: SttModelSize) -> Option<CancellationToken> {
        self.downloading.get(&size).cloned()
    }

    /// Cancels all in-progress downloads by cancelling each child token.
    /// Returns the number of downloads cancelled.
    pub fn cancel_all_downloads(&self) -> usize {
        let mut count = 0;
        for token in self.downloading.values() {
            token.cancel();
            count += 1;
        }
        count
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
}

impl SttModelSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Tiny => "tiny",
            Self::Base => "base",
            Self::Small => "small",
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
    model_path(size).exists()
}

/// Returns a list of downloaded models.
pub fn list_available_models() -> Vec<(SttModelSize, u64, PathBuf)> {
    let sizes = [SttModelSize::Tiny, SttModelSize::Base, SttModelSize::Small];
    sizes
        .iter()
        .filter_map(|&size| {
            let path = model_path(size);
            let file_size = std::fs::metadata(&path).ok()?.len();
            Some((size, file_size, path))
        })
        .collect()
}

/// Downloads a model from HuggingFace.
///
/// Returns a watch receiver for progress tracking and a join handle for the spawned task.
pub fn download_model(
    size: SttModelSize,
    cancel: &tokio_util::sync::CancellationToken,
) -> (
    tokio::sync::watch::Receiver<DownloadProgress>,
    tokio::task::JoinHandle<Result<(), DownloadError>>,
) {
    let (progress_tx, progress_rx) = tokio::sync::watch::channel(DownloadProgress::default());
    let cancel = cancel.clone();
    let handle = tokio::spawn(async move {
        let url = model_url(size);
        let path = model_path(size);
        ensure_model_dir(&path)?;

        let client = reqwest::Client::new();
        let response = fetch_model(&client, &url).await?;

        let total_bytes = response.content_length().unwrap_or(0);
        let tmp_path = path.with_extension("bin.tmp");
        stream_to_file(response, &tmp_path, &cancel, total_bytes, &progress_tx).await?;

        tokio::fs::rename(&tmp_path, &path)
            .await
            .map_err(DownloadError::Io)?;

        Ok(())
    });
    (progress_rx, handle)
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
    total_bytes: u64,
    progress_tx: &tokio::sync::watch::Sender<DownloadProgress>,
) -> Result<(), DownloadError> {
    let mut file = tokio::fs::File::create(tmp_path)
        .await
        .map_err(DownloadError::Io)?;

    let mut downloaded_bytes: u64 = 0;
    let mut stream = response.bytes_stream();
    while let Some(chunk) = stream.next().await {
        if cancel.is_cancelled() {
            drop(file);
            let _ = tokio::fs::remove_file(tmp_path).await;
            return Err(DownloadError::Cancelled);
        }
        let chunk = chunk.map_err(DownloadError::Request)?;
        downloaded_bytes += chunk.len() as u64;
        file.write_all(&chunk).await.map_err(DownloadError::Io)?;
        let percentage = if total_bytes > 0 {
            (downloaded_bytes as f64 / total_bytes as f64 * 100.0).min(100.0)
        } else {
            0.0
        };
        let _ = progress_tx.send(DownloadProgress {
            downloaded_bytes,
            total_bytes,
            percentage,
        });
    }

    file.flush().await.map_err(DownloadError::Io)?;
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
    fn cache_get_returns_none_for_empty() {
        let cache = SttModelCache {
            contexts: HashMap::new(),
            downloading: HashMap::new(),
            parent: CancellationToken::new(),
        };
        assert!(cache.get_context(SttModelSize::Small).is_none());
    }

    #[test]
    fn cache_mark_downloading_singleflight() {
        let parent = CancellationToken::new();
        let mut cache = SttModelCache {
            contexts: HashMap::new(),
            downloading: HashMap::new(),
            parent,
        };
        assert!(cache.mark_downloading(SttModelSize::Small).is_some());
        assert!(cache.mark_downloading(SttModelSize::Small).is_none());
        assert!(cache.is_downloading(SttModelSize::Small));
        cache.unmark_downloading(SttModelSize::Small);
        assert!(!cache.is_downloading(SttModelSize::Small));
    }

    #[test]
    fn parent_cancel_propagates_to_child() {
        let parent = CancellationToken::new();
        let mut cache = SttModelCache {
            contexts: HashMap::new(),
            downloading: HashMap::new(),
            parent: parent.clone(),
        };
        let child = cache.mark_downloading(SttModelSize::Small).unwrap();
        assert!(!child.is_cancelled());
        parent.cancel();
        assert!(child.is_cancelled());
    }
}
