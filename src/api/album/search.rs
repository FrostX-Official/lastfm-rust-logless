use std::collections::HashMap;

use reqwest::Method;
use serde_json::Value;

use crate::{Lastfm, Result};

#[derive(Debug)]
pub struct AlbumSearch<'a> {
    lastfm: &'a Lastfm,
    pub album: Option<String>,
    pub limit: Option<i64>,
    pub page: Option<i64>,
    method: String,
}

impl<'a> AlbumSearch<'a> {
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        AlbumSearch {
            lastfm,
            album: None,
            limit: None,
            page: None,
            method: "album.search".into(),
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
        Ok(())
    }

    /// Sends the request and retrieves the tags for the album.
    pub async fn send(self) -> Result<Value> {
        self.validate()?;

        let mut params: std::collections::HashMap<String, String> = HashMap::new();
        params.insert("api_key".to_string(), self.lastfm.get_api_key());

        if let Some(album) = self.album {
            params.insert("album".to_string(), album);
        }

        if let Some(limit) = self.limit {
            params.insert("limit".to_string(), limit.to_string());
        }

        if let Some(page) = self.page {
            params.insert("page".to_string(), page.to_string());
        }

        let response = self
            .lastfm
            .send_request(&self.method, &mut params, Method::GET, false)
            .await?;

        Ok(response)
    }
}
