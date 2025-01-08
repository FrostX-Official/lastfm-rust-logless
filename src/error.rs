use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

/// An error that could have occurred while using [`crate::lastfm_rust`].
#[derive(Error, Debug)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}

