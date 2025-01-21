use reqwest::Method;
use serde_json::Value;

use crate::api::{LastfmMethod, ParameterBuilder};
use crate::{Error, Lastfm, Result};

#[derive(Debug, Clone)]
pub struct TrackGetCorrection<'a> {
    lastfm: &'a Lastfm,
    artist: Option<String>,
    tags: Option<String>,
    method: LastfmMethod,
}

impl<'a> TrackGetCorrection<'a> {
    pub fn new(lastfm: &'a Lastfm) -> Self {
        TrackGetCorrection {
            lastfm,
            artist: None,
            tags: None,
            method: LastfmMethod::TrackGetCorrection,
        }
    }

    pub fn artist<T>(mut self, artist: T) -> Self
    where
        T: Into<String>,
    {
        self.artist = Some(artist.into());
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
            .add("artist", self.artist.expect("The artist name is required!"))
            .add_optional("tags", self.tags);

        let mut params = builder.build();

        let response = self
            .lastfm
            .send_request(self.method, &mut params, Method::POST)
            .await?;

        Ok(response)
    }
}
