use reqwest::Method;
use serde_json::Value;
use std::collections::HashMap;

use crate::{Error, Lastfm, Result};

#[derive(Debug)]
pub struct AlbumRemoveTag<'a> {
    lastfm: &'a Lastfm,
    artist: Option<String>,
    album: Option<String>,
    tag: Option<String>,
    method: String,
}

impl<'a> AlbumRemoveTag<'a> {
    pub fn new(lastfm: &'a Lastfm) -> Self {
        AlbumRemoveTag {
            lastfm,
            artist: None,
            album: None,
            tag: None,
            method: "album.removeTag".into(),
        }
    }

    pub fn artist(mut self, artist: &str) -> Self {
        self.artist = Some(artist.to_string());
        self
    }

    pub fn album(mut self, album: &str) -> Self {
        self.album = Some(album.to_string());
        self
    }

    pub fn tag(mut self, tag: &str) -> Self {
        self.tag = Some(tag.to_string());
        self
    }

    fn validate(&self) -> Result<()> {
        if self.artist.is_none() {
            return Err(Error::Generic("Field 'artist' is required.".to_string()));
        }

        if self.album.is_none() {
            return Err(Error::Generic("Field 'album' is required.".to_string()));
        }

        if self.tag.is_none() {
            return Err(Error::Generic("Field 'tags' is required.".to_string()));
        }
        //TODO: check a max of 10 tags...
        Ok(())
    }

    pub async fn send(self) -> Result<Value> {
        self.validate()?;
        let mut params: HashMap<String, String> = HashMap::new();

        if let Some(artist) = self.artist {
            params.insert("artist".to_string(), artist);
        }

        if let Some(album) = self.album {
            params.insert("album".to_string(), album);
        }

        if let Some(tag) = self.tag {
            params.insert("tags".to_string(), tag);
        }

        let response = self
            .lastfm
            .send_request(&self.method, &mut params, Method::POST, true)
            .await?;

        Ok(response)
    }
}
