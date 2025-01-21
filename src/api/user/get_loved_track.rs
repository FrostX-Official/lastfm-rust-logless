use reqwest::Method;
use serde_json::Value;

use crate::api::{LastfmMethod, ParameterBuilder};
use crate::{Error, Lastfm, Result};

#[derive(Debug)]
pub struct UserGetLovedTracks<'a> {
    lastfm: &'a Lastfm,
    pub artist: Option<String>,
    pub mbid: Option<String>,
    pub autocorrect: Option<bool>,
    pub username: Option<String>,
    pub lang: Option<String>,
    method: LastfmMethod,
}

impl<'a> UserGetLovedTracks<'a> {
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        UserGetLovedTracks {
            lastfm,
            artist: None,
            mbid: None,
            autocorrect: Some(false),
            username: None,
            lang: None,
            method: LastfmMethod::UserGetLovedTracks,
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

    pub fn username(mut self, username: &str) -> Self {
        self.username = Some(username.to_string());
        self
    }

    pub fn lang(mut self, lang: &str) -> Self {
        self.lang = Some(lang.to_string());
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
            .add_optional("username", self.username)
            .add_optional("lang", self.lang)
            .add_optional("autocorrect", self.autocorrect.map(|b| b.to_string()));

        let mut params = builder.build();

        let response = self
            .lastfm
            .send_request(self.method, &mut params, Method::GET)
            .await?;

        Ok(response)
    }
}
