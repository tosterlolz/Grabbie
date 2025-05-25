use thiserror::Error;

#[derive(Error, Debug)]
pub enum GrabbieError {
    #[error("Request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Other error: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, GrabbieError>;


impl From<anyhow::Error> for GrabbieError {
    fn from(err: anyhow::Error) -> Self {
        GrabbieError::Other(format!("Anyhow error: {}", err))
    }
}