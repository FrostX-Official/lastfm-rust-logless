mod add_tags;
mod get_info;
mod get_tags;
mod get_top_tags;
mod remove_tags;
mod search;

use crate::Lastfm;

pub use add_tags::{AlbumAddTagsRequest, WithArtist};
pub use get_info::AlbumGetInfo;
pub use get_tags::AlbumGetTags;
pub use get_top_tags::AlbumGetTopTags;
pub use remove_tags::AlbumRemoveTag;
pub use search::AlbumSearch;

/// Represents album-related operations in the Last.fm API.
#[derive(Debug)]
pub struct Album<'a> {
    lastfm: &'a Lastfm,
}

impl<'a> Album<'a> {
    /// Creates a new `Album` instance.
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        Self { lastfm }
    }

    /// Creates a request to get tags for the album.
    ///
    /// # Example
    ///
    /// Ensure you have the following dependencies in your `Cargo.toml`:
    ///
    /// ```toml
    /// [dependencies]
    /// tokio = { version = "1", features = ["full"] }
    /// dotenv = "0.15"
    /// lastfm_rust = "0.1"
    /// ```
    ///
    /// ```no_run
    /// use dotenv::dotenv;
    /// use lastfm_rust::Lastfm;
    /// use std::error::Error;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///     dotenv().ok();
    ///     let api_key = std::env::var("API_KEY").expect("API_KEY env variable is required");
    ///     let api_secret = std::env::var("API_SECRET").expect("API_SECRET env variable is required");
    ///
    ///     let lastfm = Lastfm::builder()
    ///         .api_key(api_key)
    ///         .api_secret(api_secret)
    ///         .build()?;
    ///
    ///     // Fetch tags for the album
    ///     let tags = lastfm
    ///         .album()
    ///         .get_tags()
    ///         .artist("Name_of_the_artist")
    ///         .album("Name_of_the_album")
    ///         .user("username")
    ///         .autocorrect(true)
    ///         .send()
    ///         .await?;
    ///
    ///     println!("{:?}", tags);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn get_tags(&mut self) -> AlbumGetTags<'_> {
        AlbumGetTags::new(self.lastfm)
    }

    /// Creates a request to add tags to the album.
    ///
    /// # Example
    ///
    /// Ensure you have the following dependencies in your `Cargo.toml`:
    ///
    /// ```toml
    /// [dependencies]
    /// tokio = { version = "1", features = ["full"] }
    /// dotenv = "0.15"
    /// lastfm_rust = "0.1"
    /// ```
    ///
    /// ```no_run
    /// use dotenv::dotenv;
    /// use lastfm_rust::Lastfm;
    /// use std::error::Error;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///     dotenv().ok();
    ///     let api_key = std::env::var("API_KEY").expect("API_KEY env variable is required");
    ///     let api_secret = std::env::var("API_SECRET").expect("API_SECRET env variable is required");
    ///
    ///     let mut lastfm = Lastfm::builder()
    ///         .api_key(api_key)
    ///         .api_secret(api_secret)
    ///         .build()?;
    ///
    ///     lastfm.auth().await?;
    ///
    ///     let response = lastfm
    ///         .album()
    ///         .add_tags()
    ///         .artist("Name_of_the_artist")
    ///         .album("Name_of_the_album")
    ///         .tags("comma,seperated,tags")
    ///         .send()
    ///         .await?;
    ///
    ///     println!("{:?}", response);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn add_tags(&mut self) -> AlbumAddTagsRequest<'_> {
        AlbumAddTagsRequest::new(self.lastfm)
    }

    pub fn get_info(&mut self) -> AlbumGetInfo<'_> {
        AlbumGetInfo::new(self.lastfm)
    }

    pub fn get_top_tags(&mut self) -> AlbumGetTopTags<'_> {
        AlbumGetTopTags::new(self.lastfm)
    }

    pub fn remove_tag(&mut self) -> AlbumRemoveTag<'_> {
        AlbumRemoveTag::new(self.lastfm)
    }

    pub fn search(&mut self) -> AlbumSearch<'_> {
        AlbumSearch::new(self.lastfm)
    }
}
