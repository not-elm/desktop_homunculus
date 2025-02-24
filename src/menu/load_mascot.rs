use crate::util::models_dir;
use bevy::log::error;
use bevy::prelude::In;
use bevy_webview_wry::prelude::*;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct LoadMascotArgs {
    path: PathBuf,
}

#[command]
pub async fn load_mascot(
    In(args): In<LoadMascotArgs>,
) {
    if let Some(file_name) = args.path.file_name() {
        if let Err(e) = std::fs::copy(&args.path, models_dir().join(file_name)) {
            error!("Failed to copy mascot: {}", e);
        }
    }
}
