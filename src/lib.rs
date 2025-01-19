mod api;
mod error;
mod lastfm;
mod models;

pub use api::Album;
pub use api::Auth;
pub use error::{ApiError, Error, Result};
pub use lastfm::Lastfm;
pub use models::*;
