use reqwest::Method;
use serde_json::Value;

use crate::api::{LastfmMethod, ParameterBuilder};
use crate::{Error, Lastfm, Result};

#[derive(Debug)]
pub struct TrackGetTags<'a> {
    lastfm: &'a Lastfm,
    pub artist: Option<String>,
    pub mbid: Option<String>,
    pub user: Option<String>,
    pub autocorrect: Option<bool>,
    method: LastfmMethod,
}

impl<'a> TrackGetTags<'a> {
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        TrackGetTags {
            lastfm,
            artist: None,
            mbid: None,
            user: None,
            autocorrect: Some(false),
            method: LastfmMethod::TrackGetTags,
        }
    }

    pub fn artist(mut self, artist: &str) -> Self {
        self.artist = Some(artist.to_string());
        self
    }

    pub fn mbid(mut self, mbid: &str) -> Self {
        self.mbid = Some(mbid.to_string());
        self
    }

    pub fn user(mut self, user: &str) -> Self {
        self.user = Some(user.to_string());
        self
    }

    pub fn autocorrect(mut self, autocorrect: bool) -> Self {
        self.autocorrect = Some(autocorrect);
        self
    }

    fn validate(&self) -> Result<()> {
        if self.mbid.is_none() && (self.artist.is_none()) {
            return Err(Error::Generic(
                "Either 'mbid' or 'artist' must be provided.".to_string(),
            ));
        }
        Ok(())
    }

    pub async fn send(self) -> Result<Value> {
        self.validate()?;

        let mut builder = ParameterBuilder::new();

        builder = builder
            .add_optional("artist", self.artist)
            .add_optional("mbid", self.mbid)
            .add_optional("user", self.user)
            .add_optional("autocorrect", self.autocorrect.map(|b| b.to_string()));

        let mut params = builder.build();

        let response = self
            .lastfm
            .send_request(self.method, &mut params, Method::GET)
            .await?;

        Ok(response)
    }
}
