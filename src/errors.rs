use thiserror::Error;

#[derive(Debug, Error)]
pub enum AnalyzerError {
    #[error("The provided solution directory does not exist: {0}")]
    InvalidPathError(String),
    #[error("rust-analyzer does not support the '{0}' exercise")]
    InvalidSlugError(String),
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Solution code parsing error: {0}")]
    SynError(#[from] syn::Error),
    #[error("Serialization/deserialization error: {0}")]
    SerdeError(#[from] serde_json::error::Error),
}
