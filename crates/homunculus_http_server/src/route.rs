use homunculus_core::prelude::ModModuleSource;
use serde::{Deserialize, Serialize};

pub(crate) mod app;
pub(crate) mod cameras;
pub(crate) mod commands;
pub(crate) mod displays;
pub(crate) mod effects;
pub(crate) mod entities;
pub(crate) mod gpt;
pub(crate) mod mods;
pub(crate) mod preferences;
pub(crate) mod scripts;
pub(crate) mod settings;
pub(crate) mod shadow_panel;
pub(crate) mod vrm;
pub(crate) mod vrma;
pub(crate) mod webviews;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModuleSourceRequest {
    pub source: ModModuleSource,
}
