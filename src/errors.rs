use failure::Fail;

#[derive(Debug, Fail)]
pub enum AnalyzerError {
    #[fail(display = "The provided solution directory does not exist: {}", _0)]
    InvalidPathError(String),
    #[fail(display = "rust-analyzer does not support the '{}' exercise", _0)]
    InvalidSlugError(String),
    #[fail(display = "IO error: {}", _0)]
    IOError(#[cause] std::io::Error),
    #[fail(display = "Solution code parsing error: {}", _0)]
    SynError(#[cause] syn::Error),
    #[fail(display = "Serialization/deserialization error: {}", _0)]
    SerdeError(#[cause] serde_json::error::Error),
}

impl From<std::io::Error> for AnalyzerError {
    fn from(err: std::io::Error) -> Self {
        AnalyzerError::IOError(err)
    }
}

impl From<syn::Error> for AnalyzerError {
    fn from(err: syn::Error) -> Self {
        AnalyzerError::SynError(err)
    }
}

impl From<serde_json::error::Error> for AnalyzerError {
    fn from(err: serde_json::error::Error) -> Self {
        AnalyzerError::SerdeError(err)
    }
}
