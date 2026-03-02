use bevy::asset::io::{AssetReader, AssetReaderError, PathStream, Reader, VecReader};
use futures_lite::Stream;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::{Arc, RwLock};
use std::task::{Context, Poll};

/// A custom [`AssetReader`] that reads assets from the mod `node_modules/` directory.
///
/// Asset paths are resolved directly relative to the `node_modules/` root,
/// or via the `dir_map` for mods that are nested inside other mods' `node_modules/`.
///
/// Security is enforced by the Asset ID system: only paths explicitly declared
/// in each mod's `package.json` `homunculus.assets` field are registered.
pub(crate) struct ModAssetReader {
    root_path: PathBuf,
    dir_map: Arc<RwLock<HashMap<String, PathBuf>>>,
}

impl ModAssetReader {
    pub fn new(root_path: PathBuf, dir_map: Arc<RwLock<HashMap<String, PathBuf>>>) -> Self {
        Self { root_path, dir_map }
    }

    fn full_path(&self, path: &Path) -> Option<PathBuf> {
        // Reject path traversal attempts
        if path
            .components()
            .any(|c| matches!(c, std::path::Component::ParentDir))
        {
            return None;
        }

        let mod_name = extract_mod_name(path);
        if !mod_name.is_empty()
            && let Ok(map) = self.dir_map.read()
            && let Some(mod_dir) = map.get(&mod_name)
        {
            let remaining = path.strip_prefix(&mod_name).unwrap_or(path);
            return Some(mod_dir.join(remaining));
        }
        // Fallback: conventional flat layout
        Some(self.root_path.join(path))
    }

    fn meta_path(path: &Path) -> PathBuf {
        let mut meta_path = path.to_path_buf();
        let mut extension = path.extension().unwrap_or_default().to_os_string();
        if !extension.is_empty() {
            extension.push(".");
        }
        extension.push("meta");
        meta_path.set_extension(extension);
        meta_path
    }
}

impl AssetReader for ModAssetReader {
    async fn read<'a>(&'a self, path: &'a Path) -> Result<impl Reader + 'a, AssetReaderError> {
        let full_path = self
            .full_path(path)
            .ok_or_else(|| AssetReaderError::NotFound(path.to_path_buf()))?;
        let bytes = std::fs::read(&full_path).map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                AssetReaderError::NotFound(full_path)
            } else {
                e.into()
            }
        })?;
        Ok(VecReader::new(bytes))
    }

    async fn read_meta<'a>(&'a self, path: &'a Path) -> Result<impl Reader + 'a, AssetReaderError> {
        let meta_path = Self::meta_path(path);
        let full_path = self
            .full_path(&meta_path)
            .ok_or_else(|| AssetReaderError::NotFound(meta_path.clone()))?;
        let bytes = std::fs::read(&full_path).map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                AssetReaderError::NotFound(full_path)
            } else {
                e.into()
            }
        })?;
        Ok(VecReader::new(bytes))
    }

    async fn read_directory<'a>(
        &'a self,
        path: &'a Path,
    ) -> Result<Box<PathStream>, AssetReaderError> {
        let full_path = self
            .full_path(path)
            .ok_or_else(|| AssetReaderError::NotFound(path.to_path_buf()))?;
        match std::fs::read_dir(&full_path) {
            Ok(read_dir) => {
                let paths: Vec<PathBuf> = read_dir
                    .filter_map(|entry| {
                        let entry = entry.ok()?;
                        let path = entry.path();
                        // Filter out meta files
                        if let Some(ext) = path.extension().and_then(|e| e.to_str())
                            && ext.eq_ignore_ascii_case("meta")
                        {
                            return None;
                        }
                        // Filter out hidden files
                        if path
                            .file_name()
                            .and_then(|f| f.to_str())
                            .is_some_and(|name| name.starts_with('.'))
                        {
                            return None;
                        }
                        path.strip_prefix(&self.root_path)
                            .ok()
                            .map(|p| p.to_owned())
                    })
                    .collect();
                Ok(Box::new(VecPathStream(paths.into_iter())))
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    Err(AssetReaderError::NotFound(full_path))
                } else {
                    Err(e.into())
                }
            }
        }
    }

    async fn is_directory<'a>(&'a self, path: &'a Path) -> Result<bool, AssetReaderError> {
        let full_path = self
            .full_path(path)
            .ok_or_else(|| AssetReaderError::NotFound(path.to_path_buf()))?;
        let metadata = full_path
            .metadata()
            .map_err(|_| AssetReaderError::NotFound(path.to_owned()))?;
        Ok(metadata.file_type().is_dir())
    }
}

struct VecPathStream(std::vec::IntoIter<PathBuf>);

impl Stream for VecPathStream {
    type Item = PathBuf;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Ready(self.0.next())
    }
}

/// Extract the mod name from an asset path's leading component(s).
///
/// Handles both normal packages (`some-mod/...`) and scoped packages
/// (`@hmcs/elmer/...`).
fn extract_mod_name(path: &Path) -> String {
    let mut components = path.components();
    let first = match components.next() {
        Some(c) => c.as_os_str().to_string_lossy().to_string(),
        None => return String::new(),
    };
    if first.starts_with('@') {
        if let Some(second) = components.next() {
            format!("{}/{}", first, second.as_os_str().to_string_lossy())
        } else {
            first
        }
    } else {
        first
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_mod_name_normal() {
        assert_eq!(
            extract_mod_name(Path::new("@hmcs/elmer/assets/model.vrm")),
            "@hmcs/elmer"
        );
    }

    #[test]
    fn test_extract_mod_name_scoped() {
        assert_eq!(
            extract_mod_name(Path::new("@myorg/hmcl-fancy/assets/model.vrm")),
            "@myorg/hmcl-fancy"
        );
    }

    #[test]
    fn test_extract_mod_name_empty() {
        assert_eq!(extract_mod_name(Path::new("")), "");
    }

    #[test]
    fn test_extract_mod_name_single_component() {
        assert_eq!(extract_mod_name(Path::new("@hmcs/elmer")), "@hmcs/elmer");
    }

    #[test]
    fn test_full_path_with_dir_map() {
        let dir_map = Arc::new(RwLock::new(HashMap::from([(
            "@hmcs/vrma".to_string(),
            PathBuf::from("/mods/node_modules/@hmcs/elmer/node_modules/@hmcs/vrma"),
        )])));
        let reader = ModAssetReader::new(PathBuf::from("/mods/node_modules"), dir_map);

        // With dir_map entry: resolves via map
        assert_eq!(
            reader.full_path(Path::new("@hmcs/vrma/assets/idle.vrma")),
            Some(PathBuf::from(
                "/mods/node_modules/@hmcs/elmer/node_modules/@hmcs/vrma/assets/idle.vrma"
            ))
        );
    }

    #[test]
    fn test_full_path_fallback() {
        let dir_map = Arc::new(RwLock::new(HashMap::new()));
        let reader = ModAssetReader::new(PathBuf::from("/mods/node_modules"), dir_map);

        // Without dir_map entry: falls back to root_path
        assert_eq!(
            reader.full_path(Path::new("@hmcs/elmer/assets/model.vrm")),
            Some(PathBuf::from(
                "/mods/node_modules/@hmcs/elmer/assets/model.vrm"
            ))
        );
    }

    #[test]
    fn test_full_path_rejects_traversal() {
        let dir_map = Arc::new(RwLock::new(HashMap::new()));
        let reader = ModAssetReader::new(PathBuf::from("/mods/node_modules"), dir_map);

        assert_eq!(
            reader.full_path(Path::new("@hmcs/elmer/../../etc/passwd")),
            None
        );
        assert_eq!(reader.full_path(Path::new("../secret")), None);
    }
}
