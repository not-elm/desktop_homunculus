pub type AppResult<T = ()> = Result<T, anyhow::Error>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Failed to parse vrm extensions:\n{0}")]
    FailedParseExtensions(#[from] serde_json::error::Error),
}

#[macro_export]
macro_rules! app_error {
    ($tag:expr, $message: literal) => {
        anyhow::anyhow!("[{}] {}", $tag, $message)
    };

     ($tag:expr, $fmt:expr, $($arg:tt)*) => {
        anyhow::anyhow!("[{}] {}", $tag, format!($fmt, $($arg)*))
    };
}