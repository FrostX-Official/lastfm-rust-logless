use crate::{
    api::{LastfmMethod, ParameterBuilder},
    APIResponse, Error, Lastfm, Result,
};
use reqwest::Method;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct ArtistGetTopTags<'a> {
    lastfm: &'a Lastfm,
    pub artist: Option<String>,
    pub mbid: Option<String>,
    pub autocorrect: Option<bool>,
    method: LastfmMethod,
}

impl<'a> ArtistGetTopTags<'a> {
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        ArtistGetTopTags {
            lastfm,
            artist: None,
            mbid: None,
            autocorrect: Some(false),
            method: LastfmMethod::ArtistGetTopTags,
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
    pub async fn send(self) -> Result<APIResponse<Value>> {
        self.validate()?;

        let mut builder = ParameterBuilder::new();

        builder = builder
            .add("artist", self.artist.expect("The artist name is required!"))
            .add_optional("mbid", self.mbid)
            .add_optional("autocorrect", self.autocorrect.map(|b| b.to_string()));

        let mut params = builder.build();

        let response = self
            .lastfm
            .send_request(self.method, &mut params, Method::GET)
            .await?;

        Ok(response)
    }
}
