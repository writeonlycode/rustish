use std::num::ParseIntError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}: not found")]
    CommandNotFound(String),

    #[error("{context}: {source}")]
    InvalidExitCode {
        context: String,
        source: ParseIntError,
    },

    #[error("{context}: {source}")]
    Io {
        context: String,
        source: std::io::Error,
    },

    #[error("{context}: {message}")]
    Cd {
        context: String,
        message: String,
        source: std::io::Error,
    },
}
