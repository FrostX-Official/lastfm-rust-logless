use reqwest::Method;
use serde_json::Value;
use std::collections::HashMap;

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
    method: String,
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
            method: "album.getInfo".into(),
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

        let mut params: std::collections::HashMap<String, String> = HashMap::new();
        params.insert("api_key".to_string(), self.lastfm.get_api_key());

        if let Some(artist) = self.artist {
            params.insert("artist".to_string(), artist);
        }

        if let Some(album) = self.album {
            params.insert("album".to_string(), album);
        }

        if let Some(mbid) = self.mbid {
            params.insert("mbid".to_string(), mbid);
        }

        if let Some(autocorrect) = self.autocorrect {
            params.insert("autocorrect".to_string(), autocorrect.to_string());
        }

        if let Some(username) = self.username {
            params.insert("username".to_string(), username);
        }

        if let Some(lang) = self.lang {
            params.insert("lang".to_string(), lang);
        }

        let response = self
            .lastfm
            .send_request(&self.method, &mut params, Method::GET, false)
            .await?;

        Ok(response)
    }
}
