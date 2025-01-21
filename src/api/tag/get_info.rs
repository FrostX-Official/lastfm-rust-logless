use reqwest::Method;
use serde_json::Value;

use crate::api::{LastfmMethod, ParameterBuilder};
use crate::{Lastfm, Result};

#[derive(Debug)]
pub struct TagGetInfo<'a> {
    lastfm: &'a Lastfm,
    pub tag: Option<String>,
    pub lang: Option<String>,
    method: LastfmMethod,
}

impl<'a> TagGetInfo<'a> {
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        TagGetInfo {
            lastfm,
            tag: None,
            lang: None,
            method: LastfmMethod::TagGetInfo,
        }
    }

    pub fn tag(mut self, tag: &str) -> Self {
        self.tag = Some(tag.to_string());
        self
    }

    pub fn lang(mut self, lang: &str) -> Self {
        self.lang = Some(lang.to_string());
        self
    }

    fn validate(&self) -> Result<()> {
        // TODO: check if tag is provided.
        Ok(())
    }

    pub async fn send(self) -> Result<Value> {
        self.validate()?;

        let mut builder = ParameterBuilder::new();

        builder = builder
            .add_optional("tag", self.tag)
            .add_optional("lang", self.lang);

        let mut params = builder.build();

        let response = self
            .lastfm
            .send_request(self.method, &mut params, Method::GET)
            .await?;

        Ok(response)
    }
}
