use reqwest::Method;
use serde_json::Value;

use crate::api::{LastfmMethod, ParameterBuilder};
use crate::{Error, Lastfm, Result};

#[derive(Debug)]
pub struct ArtistGetSimilar<'a> {
    lastfm: &'a Lastfm,
    pub artist: Option<String>,
    pub mbid: Option<String>,
    pub autocorrect: Option<bool>,
    pub limit: Option<i64>,
    method: LastfmMethod,
}

impl<'a> ArtistGetSimilar<'a> {
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        ArtistGetSimilar {
            lastfm,
            artist: None,
            mbid: None,
            limit: None,
            autocorrect: Some(false),
            method: LastfmMethod::ArtistGetSimilar,
        }
    }

    /// Sets the artist for the request.
    pub fn artist(mut self, artist: &str) -> Self {
        self.artist = Some(artist.to_string());
        self
    }

    /// Sets the mbid for the request.
    pub fn mbid(mut self, mbid: &str) -> Self {
        self.mbid = Some(mbid.to_string());
        self
    }

    /// Limit the number of similar artists returned
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Sets whether to apply autocorrection.
    pub fn autocorrect(mut self, autocorrect: bool) -> Self {
        self.autocorrect = Some(autocorrect);
        self
    }

    /// Validates the request parameters.
    fn validate(&self) -> Result<()> {
        if self.mbid.is_none() && (self.artist.is_none()) {
            return Err(Error::Generic(
                "Either 'mbid' or 'artist' must be provided.".to_string(),
            ));
        }
        Ok(())
    }

    /// Sends the request and retrieves the tags for the artist.
    pub async fn send(self) -> Result<Value> {
        self.validate()?;

        let mut builder = ParameterBuilder::new();

        builder = builder
            .add_optional("artist", self.artist)
            .add_optional("mbid", self.mbid)
            .add_optional("limit", self.limit.map(|b| b.to_string()))
            .add_optional("autocorrect", self.autocorrect.map(|b| b.to_string()));

        let mut params = builder.build();

        let response = self
            .lastfm
            .send_request(self.method, &mut params, Method::GET)
            .await?;

        Ok(response)
    }
}
