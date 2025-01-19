use std::collections::HashMap;

use reqwest::Method;
use serde_json::Value;

use crate::{Error, Lastfm, Result};

#[derive(Debug)]
pub struct AlbumGetTags<'a> {
    lastfm: &'a Lastfm,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub mbid: Option<String>,
    pub user: Option<String>,
    pub autocorrect: Option<bool>,
    method: String,
}

impl<'a> AlbumGetTags<'a> {
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        AlbumGetTags {
            lastfm,
            artist: None,
            album: None,
            mbid: None,
            user: None,
            autocorrect: Some(false),
            method: "album.getTags".into(),
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

    /// Sets the mbid for the request.
    pub fn mbid(mut self, mbid: &str) -> Self {
        self.mbid = Some(mbid.to_string());
        self
    }

    /// Sets the user for the request.
    pub fn user(mut self, user: &str) -> Self {
        self.user = Some(user.to_string());
        self
    }

    /// Sets whether to apply autocorrection.
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
        if let Some(user) = self.user {
            params.insert("user".to_string(), user);
        }
        if let Some(autocorrect) = self.autocorrect {
            params.insert("autocorrect".to_string(), autocorrect.to_string());
        }

        let response = self
            .lastfm
            .send_request(&self.method, &mut params, Method::GET, false)
            .await?;

        Ok(response)
    }
}
