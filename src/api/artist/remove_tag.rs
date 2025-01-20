use reqwest::Method;
use serde_json::Value;

use crate::api::{LastfmMethod, ParameterBuilder};
use crate::{Error, Lastfm, Result};

#[derive(Debug)]
pub struct ArtistRemoveTag<'a> {
    lastfm: &'a Lastfm,
    artist: Option<String>,
    tag: Option<String>,
    method: LastfmMethod,
}

impl<'a> ArtistRemoveTag<'a> {
    pub fn new(lastfm: &'a Lastfm) -> Self {
        ArtistRemoveTag {
            lastfm,
            artist: None,
            tag: None,
            method: LastfmMethod::ArtistRemoveTag,
        }
    }

    pub fn artist(mut self, artist: &str) -> Self {
        self.artist = Some(artist.to_string());
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

        if self.tag.is_none() {
            return Err(Error::Generic("Field 'tags' is required.".to_string()));
        }

        let tag_count = self
            .tag
            .as_ref()
            .unwrap()
            .split(',')
            .collect::<Vec<_>>()
            .len();
        if tag_count > 1 {
            return Err(Error::Generic(
                "Only 1 tag is allowed to remove at a time".to_string(),
            ));
        }

        Ok(())
    }

    pub async fn send(self) -> Result<Value> {
        self.validate()?;

        let mut builder = ParameterBuilder::new();

        builder = builder
            .add("artist", self.artist.unwrap())
            .add("tag", self.tag.unwrap());

        let mut params = builder.build();

        let response = self
            .lastfm
            .send_request(self.method, &mut params, Method::POST)
            .await?;

        Ok(response)
    }
}
