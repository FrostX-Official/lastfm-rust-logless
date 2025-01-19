use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiError {
    pub error: i32,
    pub message: String,
    pub links: Option<Vec<String>>,
}

/// An error that could have occurred while using [`crate`].
#[derive(Error, Debug)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("XML parsing error: {0}")]
    XmlError(#[from] serde_xml_rs::Error),

    #[error("API returned an error: {0:?}")]
    ApiError(ApiError),

    #[error("Unknown error: {0}")]
    UnknownError(String),
}
