#[allow(unused)]
use reqwest::Client as ReqwestClient;

mod error;
mod utils;

pub use error::{Error, Result};

pub const LASTFM_API_URL: &str = "http://ws.audioscrobbler.com/2.0/";

#[derive(Debug)]
pub struct Lastfm {
    api_key: String,
    client: ReqwestClient,
}

impl Lastfm {
    pub fn builder() -> LastfmBuilder {
        LastfmBuilder {
            api_key: None,
            client: None,
        }
    }

    pub fn get_client(&self) -> &ReqwestClient {
        &self.client
    }
}

#[derive(Default)]
pub struct LastfmBuilder {
    api_key: Option<String>,
    client: Option<ReqwestClient>,
}

impl LastfmBuilder {
    pub fn api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key.to_string());
        self
    }

    pub fn client(mut self, client: ReqwestClient) -> Self {
        self.client = Some(client);
        self
    }

    pub fn build(self) -> Result<Lastfm> {
        match self.api_key {
            Some(api_key) => Ok(Lastfm {
                api_key,
                client: self.client.unwrap_or_default(),
            }),
            None => Err(Error::Generic("Api Key is required.".into())),
        }
    }
}
