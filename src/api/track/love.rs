use crate::{
    api::{LastfmMethod, ParameterBuilder},
    APIResponse, Error, Lastfm, Result,
};
use reqwest::Method;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct TrackLove<'a> {
    lastfm: &'a Lastfm,
    artist: Option<String>,
    track: Option<String>,
    method: LastfmMethod,
}

impl<'a> TrackLove<'a> {
    pub fn new(lastfm: &'a Lastfm) -> Self {
        TrackLove {
            lastfm,
            artist: None,
            track: None,
            method: LastfmMethod::TrackLove,
        }
    }

    pub fn artist<T>(mut self, artist: T) -> Self
    where
        T: Into<String>,
    {
        self.artist = Some(artist.into());
        self
    }

    pub fn track<T>(mut self, track: T) -> Self
    where
        T: Into<String>,
    {
        self.track = Some(track.into());
        self
    }

    fn validate(&self) -> Result<()> {
        if self.artist.is_none() {
            return Err(Error::Generic("Field 'artist' is required.".to_string()));
        }

        if self.track.is_none() {
            return Err(Error::Generic("Field 'track' is required.".to_string()));
        }

        Ok(())
    }

    pub async fn send(self) -> Result<APIResponse<Value>> {
        self.validate()?;
        let mut builder = ParameterBuilder::new();

        builder = builder
            .add("artist", self.artist.expect("The artist name is required!"))
            .add("track", self.track.expect("The track name is required!"));

        let mut params = builder.build();

        let response = self
            .lastfm
            .send_request(self.method, &mut params, Method::POST)
            .await?;

        Ok(response)
    }
}
