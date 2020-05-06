use failure::Fail;
use std::io;

/// Error type for kvs.
#[derive(Fail, Debug)]
pub enum MoonError {
    /// IO error.
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),
    /// Serialization or deserialization error.
    #[fail(display = "{}", _0)]
    Serde(#[cause] serde_json::Error),
    /// Unexpected command type error.
    /// It indicated a corrupted log or a program bug.
    #[fail(display = "Unexpected command type")]
    UnexpectedCommandType,
}

impl From<io::Error> for MoonError {
    fn from(err: io::Error) -> MoonError {
        MoonError::Io(err)
    }
}

impl From<serde_json::Error> for MoonError {
    fn from(err: serde_json::Error) -> MoonError {
        MoonError::Serde(err)
    }
}

/// Result type for kvs.
pub type Result<T> = std::result::Result<T, MoonError>;
