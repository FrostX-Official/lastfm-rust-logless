use reqwest::Method;
use serde_json::Value;

use crate::api::{LastfmMethod, ParameterBuilder};
use crate::{Error, Lastfm, Result};

#[derive(Debug)]
pub struct AlbumSearch<'a> {
    lastfm: &'a Lastfm,
    pub album: Option<String>,
    pub limit: Option<i64>,
    pub page: Option<i64>,
    method: LastfmMethod,
}

impl<'a> AlbumSearch<'a> {
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        AlbumSearch {
            lastfm,
            album: None,
            limit: None,
            page: None,
            method: LastfmMethod::AlbumSearch,
        }
    }

    /// Sets the album name for the request.
    pub fn album(mut self, album: &str) -> Self {
        self.album = Some(album.to_string());
        self
    }

    /// The number of results to fetch per page. Defaults to 30.
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    /// The page number to fetch. Defaults to first page.
    pub fn page(mut self, page: i64) -> Self {
        self.page = Some(page);
        self
    }

    /// Validates the request parameters.
    fn validate(&self) -> Result<()> {
        if self.album.is_none() || self.album.as_ref().unwrap().is_empty() {
            return Err(Error::Generic("The album name is required.".to_string()));
        }

        Ok(())
    }

    /// Sends the request and retrieves the tags for the album.
    pub async fn send(self) -> Result<Value> {
        self.validate()?;

        let mut builder = ParameterBuilder::new();

        builder = builder
            .add("album", self.album.expect("The album name is required!"))
            .add_optional("limit", self.limit.map(|b| b.to_string()))
            .add_optional("page", self.page.map(|b| b.to_string()));

        let mut params = builder.build();

        let response = self
            .lastfm
            .send_request(self.method, &mut params, Method::GET, false)
            .await?;

        Ok(response)
    }
}
