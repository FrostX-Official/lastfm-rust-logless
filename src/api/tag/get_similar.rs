use reqwest::Method;
use serde_json::Value;

use crate::api::{LastfmMethod, ParameterBuilder};
use crate::{Lastfm, Result};

#[derive(Debug)]
pub struct TagGetSimilar<'a> {
    lastfm: &'a Lastfm,
    pub tag: Option<String>,
    method: LastfmMethod,
}

impl<'a> TagGetSimilar<'a> {
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        TagGetSimilar {
            lastfm,
            tag: None,
            method: LastfmMethod::TagGetSimilar,
        }
    }

    pub fn tag(mut self, tag: &str) -> Self {
        self.tag = Some(tag.to_string());
        self
    }

    fn validate(&self) -> Result<()> {
        Ok(())
    }

    pub async fn send(self) -> Result<Value> {
        self.validate()?;

        let mut builder = ParameterBuilder::new();

        builder = builder.add_optional("tag", self.tag);

        let mut params = builder.build();

        let response = self
            .lastfm
            .send_request(self.method, &mut params, Method::GET)
            .await?;

        Ok(response)
    }
}
