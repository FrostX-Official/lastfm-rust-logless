use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error as StdError;
use std::fmt;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiError {
    pub error: i64,
    pub message: String,
    pub links: Vec<Value>,
}

#[derive(Debug)]
pub enum Error {
    Generic(String),
    IO(std::io::Error),
    NetworkError(reqwest::Error),
    JsonError(serde_json::Error),
    XmlError(serde_xml_rs::Error),
    ApiError(ApiError),
    UnknownError(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "API Error {}: {}", self.error, self.message)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Generic(msg) => write!(f, "Generic error: {}", msg),
            Error::IO(err) => write!(f, "IO error: {}", err),
            Error::NetworkError(err) => write!(f, "Network error: {}", err),
            Error::JsonError(err) => write!(f, "JSON error: {}", err),
            Error::XmlError(err) => write!(f, "XML error: {}", err),
            Error::ApiError(err) => write!(f, "API error: {}", err),
            Error::UnknownError(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

impl StdError for ApiError {}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::NetworkError(err) => Some(err),
            Error::JsonError(err) => Some(err),
            Error::XmlError(err) => Some(err),
            Error::ApiError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::NetworkError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::JsonError(err)
    }
}

impl From<ApiError> for Error {
    fn from(err: ApiError) -> Self {
        Error::ApiError(err)
    }
}

