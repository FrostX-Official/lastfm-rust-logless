use reqwest::Method;
use serde_json::Value;

use crate::api::{LastfmMethod, ParameterBuilder};
use crate::{Error, Lastfm, Result};

#[derive(Debug, Clone)]
pub struct ArtistGetCorrection<'a> {
    lastfm: &'a Lastfm,
    artist: Option<String>,
    method: LastfmMethod,
}

impl<'a> ArtistGetCorrection<'a> {
    pub fn new(lastfm: &'a Lastfm) -> Self {
        ArtistGetCorrection {
            lastfm,
            artist: None,
            method: LastfmMethod::ArtistGetCorrection,
        }
    }

    pub fn artist<T>(mut self, artist: T) -> Self
    where
        T: Into<String>,
    {
        self.artist = Some(artist.into());
        self
    }

    fn validate(&self) -> Result<()> {
        if self.artist.is_none() {
            return Err(Error::Generic("Field 'artist' is required.".to_string()));
        }

        Ok(())
    }

    pub async fn send(self) -> Result<Value> {
        self.validate()?;
        let mut builder = ParameterBuilder::new();

        builder = builder.add_optional("artist", self.artist);

        let mut params = builder.build();

        let response = self
            .lastfm
            .send_request(self.method, &mut params, Method::POST)
            .await?;

        Ok(response)
    }
}
