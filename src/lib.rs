mod api;
mod error;
mod lastfm;
mod models;

pub use api::Album;
pub use api::Auth;
pub use error::{ApiError, Error, Result};
pub use lastfm::{Lastfm, LASTFM_API_URL};
pub use models::*;
