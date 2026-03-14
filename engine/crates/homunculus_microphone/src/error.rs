pub type SttResult<T = ()> = Result<T, SttError>;

#[derive(thiserror::Error, Debug)]
pub enum SttError {
    #[error("mission input the device")]
    MissingInputDevice,
}
