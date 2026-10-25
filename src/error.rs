use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}: not found")]
    CommandNotFound(String),

    #[error("{context}: {source}")]
    Io {
        context: String,
        source: std::io::Error,
    },
}
