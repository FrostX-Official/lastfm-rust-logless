use crate::{
    api::{LastfmMethod, ParameterBuilder},
    APIResponse, Error, Lastfm, Result,
};
use reqwest::Method;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct ArtistSearch<'a> {
    lastfm: &'a Lastfm,
    pub artist: Option<String>,
    pub limit: Option<i64>,
    pub page: Option<i64>,
    method: LastfmMethod,
}

impl<'a> ArtistSearch<'a> {
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        ArtistSearch {
            lastfm,
            artist: None,
            limit: None,
            page: None,
            method: LastfmMethod::ArtistSearch,
        }
    }

    /// Sets the artist name for the request.
    pub fn artist(mut self, artist: &str) -> Self {
        self.artist = Some(artist.to_string());
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
        if self.artist.is_none() || self.artist.as_ref().unwrap().is_empty() {
            return Err(Error::Generic("The artist name is required.".to_string()));
        }

        Ok(())
    }

    /// Sends the request and retrieves the tags for the artist.
    pub async fn send(self) -> Result<APIResponse<Value>> {
        self.validate()?;

        let mut builder = ParameterBuilder::new();

        builder = builder
            .add("artist", self.artist.expect("The artist name is required!"))
            .add_optional("limit", self.limit.map(|b| b.to_string()))
            .add_optional("page", self.page.map(|b| b.to_string()));

        let mut params = builder.build();

        let response = self
            .lastfm
            .send_request(self.method, &mut params, Method::GET)
            .await?;

        Ok(response)
    }
}
