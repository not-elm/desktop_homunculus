use std::path::{Path, PathBuf};

pub fn homunculus_dir() -> PathBuf {
    dirs::home_dir().unwrap_or_default().join(".homunculus")
}

pub fn mod_dir() -> PathBuf {
    homunculus_dir().join("mods")
}

pub fn mod_node_modules_dir() -> PathBuf {
    mod_dir().join("node_modules")
}

pub fn remove_mystery_file_if_exists(dir: &Path) {
    let path = dir.join(".DS_Store");
    if path.exists() {
        let _ = std::fs::remove_file(path);
    }
    if let Ok(d) = std::fs::read_dir(dir) {
        for entry in d {
            let Ok(entry) = entry else {
                continue;
            };
            if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                remove_mystery_file_if_exists(&entry.path());
            }
        }
    }
}

pub fn create_dir_all_if_need(dir: &Path) {
    if !dir.exists() {
        let _ = std::fs::create_dir_all(dir);
    } else {
        remove_mystery_file_if_exists(dir);
    }
}

pub fn create_parent_dir_all_if_need(dir: &Path) {
    if let Some(parent) = dir.parent() {
        create_dir_all_if_need(parent);
    }
}

pub fn vrm_settings_path(dir: &Path, vrm_path: &Path) -> Option<PathBuf> {
    let stem = vrm_path.file_stem().and_then(|s| s.to_str())?;
    Some(dir.join(stem).with_extension("json"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_concat_path() {
        let dir = PathBuf::from("/test");
        let save_file_path = vrm_settings_path(&dir, &PathBuf::from("/sample.vrm"));
        assert_eq!(save_file_path, Some(PathBuf::from("/test/sample.json")));
    }

    #[test]
    fn test_mod_dir() {
        assert_eq!(super::mod_dir(), super::homunculus_dir().join("mods"));
    }

    #[test]
    fn test_mod_node_modules_dir() {
        assert_eq!(
            super::mod_node_modules_dir(),
            super::homunculus_dir().join("mods").join("node_modules")
        );
    }
}
