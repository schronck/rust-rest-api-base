#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Other(String),
}
