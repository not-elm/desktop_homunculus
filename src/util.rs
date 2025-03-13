use std::path::{Path, PathBuf};

pub fn models_dir() -> PathBuf {
    assets_dir().join("models")
}

pub fn assets_dir() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|exe_path| Some(exe_path.parent()?.to_path_buf()))
        .or_else(|| std::env::current_dir().ok())
        .unwrap_or_default()
        .join("assets")
}

pub fn app_data_dir() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_default()
        .join(env!("CARGO_PKG_NAME"))
}

pub fn remove_mystery_file_if_exists(dir: &Path) {
    let path = dir.join(".DS_Store");
    if path.exists() {
        let _ = std::fs::remove_file(path);
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