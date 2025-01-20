use reqwest::Method;
use serde_json::Value;

use crate::api::{LastfmMethod, ParameterBuilder};
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

#[derive(Debug, Clone)]
pub struct AlbumAddTagsRequest<'a> {
    lastfm: &'a Lastfm,
    artist: Option<String>,
    album: Option<String>,
    tags: Option<String>,
    method: LastfmMethod,
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
            method: LastfmMethod::AlbumAddTags,
        }
    }

    pub fn artist<T>(mut self, artist: T) -> Self
    where
        T: Into<String>,
    {
        self.artist = Some(artist.into());
        self
    }

    pub fn album<T>(mut self, album: T) -> Self
    where
        T: Into<String>,
    {
        self.album = Some(album.into());
        self
    }

    pub fn tags<T>(mut self, tags: T) -> Self
    where
        T: Into<String>,
    {
        self.tags = Some(tags.into());
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

        let tag_count = self
            .tags
            .as_ref()
            .unwrap()
            .split(',')
            .collect::<Vec<_>>()
            .len();
        if tag_count > 10 {
            return Err(Error::Generic("Cannot exceed 10 tags.".to_string()));
        }

        Ok(())
    }

    pub async fn send(self) -> Result<Value> {
        self.validate()?;
        let mut builder = ParameterBuilder::new();

        builder = builder
            .add_optional("artist", self.artist)
            .add_optional("album", self.album)
            .add_optional("tags", self.tags);

        let mut params = builder.build();

        let response = self
            .lastfm
            .send_request(self.method, &mut params, Method::POST)
            .await?;

        Ok(response)
    }
}
