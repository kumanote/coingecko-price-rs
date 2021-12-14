use std::io;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("API response error: [{status_code:?}]{reason:?}")]
    Gateway { status_code: u16, reason: String },

    #[error("Hyper error: {cause:?}")]
    Hyper { cause: hyper::Error },

    #[error("IO error: {cause:?}")]
    Io { cause: io::Error },

    #[error("Deserialize error: {cause:?}")]
    Deserialize { cause: serde_json::Error },
}

impl From<hyper::Error> for Error {
    fn from(cause: hyper::Error) -> Error {
        Error::Hyper { cause }
    }
}

impl From<io::Error> for Error {
    fn from(cause: io::Error) -> Error {
        Error::Io { cause }
    }
}

impl From<serde_json::Error> for Error {
    fn from(cause: serde_json::Error) -> Error {
        Error::Deserialize { cause }
    }
}
