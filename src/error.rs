use failure::Fail;
use std::io;

#[derive(Fail, Debug)]
pub enum MoonError {
    /// IO error.
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),
    #[fail(display = "{}", _0)]
    Serde(#[cause] serde_json::Error),
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

pub type Result<T> = std::result::Result<T, MoonError>;
