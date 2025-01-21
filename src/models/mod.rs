mod album_get_info_response;
mod album_get_tags;
mod album_get_top_tags;
mod auth_get_token;

use std::fmt;

pub use album_get_info_response::AlbumGetInfoResponse;
pub use album_get_tags::AlbumGetTagsResponse;
pub use auth_get_token::AuthGetTokenResponse;
pub use album_get_top_tags::AlbumGetTopTagsResponse;
use serde_json::{to_string_pretty, Value};

use crate::ApiError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum APIResponse<T> {
    Success(T),
    Error(ApiError),
}

impl fmt::Display for APIResponse<Value> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            APIResponse::Success(value) => {
                let pretty_value =
                    to_string_pretty(value).unwrap_or_else(|_| "Invalid JSON".to_string());
                write!(f, "{}", pretty_value)
            }
            APIResponse::Error(api_error) => {
                write!(f, "Error: {} - {}", api_error.error, api_error.message)
            }
        }
    }
}
