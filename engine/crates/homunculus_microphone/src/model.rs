use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::Arc;
use whisper_rs::WhisperContext;

/// HTTP State に持たせる NewType
#[derive(Clone)]
pub struct SharedSttModelCache(pub Arc<tokio::sync::Mutex<SttModelCache>>);

impl SharedSttModelCache {
    pub fn new() -> Self {
        Self(Arc::new(tokio::sync::Mutex::new(SttModelCache::default())))
    }
}

/// ロード済み WhisperContext のキャッシュ + ダウンロード中トラッキング
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

/// Whisper モデルサイズ
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
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

/// モデルファイルのパスを返す
pub fn model_path(size: SttModelSize) -> PathBuf {
    homunculus_utils::path::homunculus_dir()
        .join("models")
        .join(format!("ggml-{}-q5_1.bin", size.as_str()))
}

/// モデルがダウンロード済みかチェック
pub fn is_model_available(size: SttModelSize) -> bool {
    let path = model_path(size);
    match std::fs::metadata(&path) {
        Ok(meta) => meta.len() == size.expected_size(),
        Err(_) => false,
    }
}

/// ダウンロード済みモデルの一覧を返す
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

/// HuggingFace からモデルをダウンロード
pub async fn download_model(
    size: SttModelSize,
    cancel: &tokio_util::sync::CancellationToken,
) -> Result<(), DownloadError> {
    let url = format!(
        "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-{}-q5_1.bin",
        size.as_str()
    );
    let path = model_path(size);

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(DownloadError::Request)?;

    if !response.status().is_success() {
        return Err(DownloadError::HttpStatus(response.status().as_u16()));
    }

    let tmp_path = path.with_extension("bin.tmp");
    let mut file = tokio::fs::File::create(&tmp_path)
        .await
        .map_err(DownloadError::Io)?;

    use futures_lite::StreamExt;
    use tokio::io::AsyncWriteExt;

    let mut stream = response.bytes_stream();
    while let Some(chunk) = stream.next().await {
        if cancel.is_cancelled() {
            drop(file);
            let _ = tokio::fs::remove_file(&tmp_path).await;
            return Err(DownloadError::Cancelled);
        }
        let chunk = chunk.map_err(DownloadError::Request)?;
        file.write_all(&chunk).await.map_err(DownloadError::Io)?;
    }

    file.flush().await.map_err(DownloadError::Io)?;
    drop(file);

    tokio::fs::rename(&tmp_path, &path)
        .await
        .map_err(DownloadError::Io)?;

    let actual = tokio::fs::metadata(&path)
        .await
        .map_err(DownloadError::Io)?
        .len();
    if actual != size.expected_size() {
        let _ = tokio::fs::remove_file(&path).await;
        return Err(DownloadError::SizeMismatch {
            expected: size.expected_size(),
            actual,
        });
    }

    Ok(())
}

/// モデルダウンロード時のエラー
#[derive(Debug, thiserror::Error)]
pub enum DownloadError {
    #[error("HTTP request failed: {0}")]
    Request(reqwest::Error),
    #[error("HTTP status {0}")]
    HttpStatus(u16),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Download cancelled")]
    Cancelled,
    #[error("Size mismatch: expected {expected}, got {actual}")]
    SizeMismatch { expected: u64, actual: u64 },
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
