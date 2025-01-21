use crate::{
    api::{LastfmMethod, ParameterBuilder},
    APIResponse, Error, Lastfm, Result,
};
use reqwest::Method;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct TrackSearch<'a> {
    lastfm: &'a Lastfm,
    pub artist: Option<String>,
    pub track: Option<String>,
    pub limit: Option<i64>,
    pub page: Option<i64>,
    method: LastfmMethod,
}

impl<'a> TrackSearch<'a> {
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        TrackSearch {
            lastfm,
            artist: None,
            track: None,
            limit: None,
            page: None,
            method: LastfmMethod::TrackSearch,
        }
    }

    pub fn artist(mut self, artist: &str) -> Self {
        self.artist = Some(artist.to_string());
        self
    }

    pub fn track(mut self, track: &str) -> Self {
        self.track = Some(track.to_string());
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn page(mut self, page: i64) -> Self {
        self.page = Some(page);
        self
    }

    fn validate(&self) -> Result<()> {
        if self.artist.is_none() || self.artist.as_ref().unwrap().is_empty() {
            return Err(Error::Generic("The artist name is required.".to_string()));
        }

        Ok(())
    }

    pub async fn send(self) -> Result<APIResponse<Value>> {
        self.validate()?;

        let mut builder = ParameterBuilder::new();

        builder = builder
            .add("track", self.track.expect("The track name is required!"))
            .add_optional("artist", self.artist)
            .add_optional("limit", self.limit.map(|b| b.to_string()))
            .add_optional("page", self.page.map(|b| b.to_string()));

        let mut params = builder.build();

        let response = self
            .lastfm
            .send_request(self.method, &mut params, Method::GET)
            .await?;

        Ok(response)
    }
}
