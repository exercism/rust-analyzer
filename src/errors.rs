use failure::Fail;

#[derive(Debug, Fail)]
pub enum AnalyzerError {
    #[fail(display = "The provided path does not exist: {}", _0)]
    InvalidPathError(String),
    #[fail(display = "rust-analyzer does not support the '{}' exercise", _0)]
    InvalidTypeError(String),
    #[fail(display = "IO error: {}", _0)]
    IOError(#[cause] std::io::Error),
}

impl From<std::io::Error> for AnalyzerError {
    fn from(err: std::io::Error) -> Self {
        AnalyzerError::IOError(err)
    }
}
