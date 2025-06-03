pub type AppResult<T = ()> = Result<T, anyhow::Error>;

pub trait OutputLog {
    fn output_log_if_error(&self, tag: &str);
}

impl<T, E: std::error::Error> OutputLog for Result<T, E> {
    fn output_log_if_error(&self, tag: &str) {
        if let Err(e) = self {
            bevy::log::error!("[{tag}]: {e}");
        }
    }
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
