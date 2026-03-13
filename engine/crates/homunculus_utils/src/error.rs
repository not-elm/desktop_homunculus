use std::path::PathBuf;

pub type UtilResult<T = ()> = Result<T, UtilError>;

#[derive(thiserror::Error, Debug)]
pub enum UtilError {
    #[error(transparent)]
    Mods(#[from] ModsError),
    #[error(transparent)]
    Config(#[from] ConfigError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
    #[error("failed to execute child process: {0}")]
    ForkProcess(String),
}

#[derive(Debug, thiserror::Error)]
pub enum ModsError {
    #[error("failed to install the mod: {0}")]
    Install(#[source] std::io::Error),
    #[error("failed to uninstall the mod: {0}")]
    Uninstall(#[source] std::io::Error),
    #[error("failed to retrive installation mod path: {0}")]
    List(String),
    #[error("failed to update mod: {0}")]
    Update(String),
}

/// Errors that can occur when loading or saving config.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("failed to read {path}: {error}", path = .0.display(), error = .1)]
    Read(PathBuf, std::io::Error),
    #[error("failed to write {path}: {error}", path = .0.display(), error = .1)]
    Write(PathBuf, std::io::Error),
    #[error("failed to parse {path}: {error}", path = .0.display(), error = .1)]
    Parse(PathBuf, toml::de::Error),
    #[error("failed to serialize config: {0}")]
    Serialize(toml::ser::Error),
}
