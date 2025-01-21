use crate::{
    api::{LastfmMethod, ParameterBuilder},
    APIResponse, Lastfm, Result,
};
use reqwest::Method;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct TagGetTopAlbums<'a> {
    lastfm: &'a Lastfm,
    pub tag: Option<String>,
    pub limit: Option<i64>,
    pub page: Option<i64>,
    method: LastfmMethod,
}

impl<'a> TagGetTopAlbums<'a> {
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        TagGetTopAlbums {
            lastfm,
            tag: None,
            limit: None,
            page: None,
            method: LastfmMethod::TagGetTopAlbums,
        }
    }

    pub fn tag(mut self, tag: &str) -> Self {
        self.tag = Some(tag.to_string());
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
        Ok(())
    }

    pub async fn send(self) -> Result<APIResponse<Value>> {
        self.validate()?;

        let mut builder = ParameterBuilder::new();

        builder = builder
            .add_optional("tag", self.tag)
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
