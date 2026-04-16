use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("CRG Key Error: {0} is not a valid key.")]
    CRGKeyError(String),
    #[error("{0}")]
    SerdeError(#[from] serde_json::Error)
}

pub type Result<T> = std::result::Result<T, Error>;