use reqwest::Method;
use serde_json::Value;
use std::collections::HashMap;

use crate::{Error, Lastfm, Result};

// #[derive(Deserialize, Debug)]
// struct LfmErrorResponse {
//     status: String,
//     #[serde(rename = "error")]
//     error: LfmError,
// }
//
// #[derive(Deserialize, Debug)]
// struct LfmError {
//     code: u32,
//     #[serde(rename = "$value")]
//     message: String,
// }

#[derive(Debug)]
pub struct AlbumAddTagsRequest<'a> {
    lastfm: &'a Lastfm,
    artist: Option<String>,
    album: Option<String>,
    tags: Option<String>,
    method: String,
}

// #[derive(Debug)]
// pub struct FullResponse {
//     pub status: reqwest::StatusCode,
//     pub response: Response,
// }

// #[derive(Debug)]
// pub struct ApiResponse<T> {
//     pub data: T,
//     pub status: u16,
//     pub headers: HeaderMap,
// }

impl<'a> AlbumAddTagsRequest<'a> {
    pub fn new(lastfm: &'a Lastfm) -> Self {
        AlbumAddTagsRequest {
            lastfm,
            artist: None,
            album: None,
            tags: None,
            method: "album.addTags".into(),
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

    pub fn tags(mut self, tags: &str) -> Self {
        self.tags = Some(tags.to_string());
        self
    }

    fn validate(&self) -> Result<()> {
        if self.artist.is_none() {
            return Err(Error::Generic("Field 'artist' is required.".to_string()));
        }

        if self.album.is_none() {
            return Err(Error::Generic("Field 'album' is required.".to_string()));
        }

        if self.tags.is_none() {
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

        if let Some(tags) = self.tags {
            params.insert("tags".to_string(), tags);
        }

        let response = self
            .lastfm
            .send_request(&self.method, &mut params, Method::POST, true)
            .await?;

        Ok(response)
    }
}
