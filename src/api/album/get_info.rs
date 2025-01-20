use reqwest::Method;
use serde_json::Value;

use crate::api::{LastfmMethod, ParameterBuilder};
use crate::{Error, Lastfm, Result};

#[derive(Debug)]
pub struct AlbumGetInfo<'a> {
    lastfm: &'a Lastfm,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub mbid: Option<String>,
    pub autocorrect: Option<bool>,
    pub username: Option<String>,
    pub lang: Option<String>,
    method: LastfmMethod,
}

impl<'a> AlbumGetInfo<'a> {
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        AlbumGetInfo {
            lastfm,
            artist: None,
            album: None,
            mbid: None,
            autocorrect: Some(false),
            username: None,
            lang: None,
            method: LastfmMethod::AlbumGetInfo,
        }
    }

    /// Sets the artist for the request.
    pub fn artist(mut self, artist: &str) -> Self {
        self.artist = Some(artist.to_string());
        self
    }

    /// Sets the album name for the request.
    pub fn album(mut self, album: &str) -> Self {
        self.album = Some(album.to_string());
        self
    }

    /// The musicbrainz id for the album
    pub fn mbid(mut self, mbid: &str) -> Self {
        self.mbid = Some(mbid.to_string());
        self
    }

    /// The username for the context of the request.
    /// If supplied, the user's playcount for this album is included in the response.
    pub fn username(mut self, username: &str) -> Self {
        self.username = Some(username.to_string());
        self
    }

    /// The language to return the biography in, expressed as an ISO 639 alpha-2 code.
    pub fn lang(mut self, lang: &str) -> Self {
        self.lang = Some(lang.to_string());
        self
    }

    /// Transform misspelled artist names into correct artist names, returning the correct version instead.
    /// The corrected artist name will be returned in the response.
    pub fn autocorrect(mut self, autocorrect: bool) -> Self {
        self.autocorrect = Some(autocorrect);
        self
    }

    /// Validates the request parameters.
    fn validate(&self) -> Result<()> {
        if self.mbid.is_none() && (self.artist.is_none() || self.album.is_none()) {
            return Err(Error::Generic(
                "Either 'mbid' or both 'artist' and 'album' must be provided.".to_string(),
            ));
        }
        Ok(())
    }

    /// Sends the request and retrieves the tags for the album.
    pub async fn send(self) -> Result<Value> {
        self.validate()?;

        let mut builder = ParameterBuilder::new();

        builder = builder
            .add_optional("artist", self.artist)
            .add_optional("album", self.album)
            .add_optional("mbid", self.mbid)
            .add_optional("username", self.username)
            .add_optional("lang", self.lang)
            .add_optional("autocorrect", self.autocorrect.map(|b| b.to_string()));

        let mut params = builder.build();

        let response = self
            .lastfm
            .send_request(self.method, &mut params, Method::GET, false)
            .await?;

        Ok(response)
    }
}
