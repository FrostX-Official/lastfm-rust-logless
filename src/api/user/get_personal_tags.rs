use crate::{
    api::{LastfmMethod, ParameterBuilder},
    APIResponse, Error, Lastfm, Result,
};
use reqwest::Method;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct UserGetPersonalTags<'a> {
    lastfm: &'a Lastfm,
    pub user: Option<String>,
    pub tag: Option<String>,
    //TODO: make an enum to store tagging type
    pub taggingtype: Option<String>,
    pub limit: Option<String>,
    pub page: Option<String>,
    method: LastfmMethod,
}

impl<'a> UserGetPersonalTags<'a> {
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        UserGetPersonalTags {
            lastfm,
            user: None,
            tag: None,
            taggingtype: None,
            limit: None,
            page: None,
            method: LastfmMethod::UserGetPersonalTags,
        }
    }

    pub fn user(mut self, user: &str) -> Self {
        self.user = Some(user.to_string());
        self
    }

    pub fn tag(mut self, tag: &str) -> Self {
        self.tag = Some(tag.to_string());
        self
    }

    pub fn limit(mut self, limit: &str) -> Self {
        self.limit = Some(limit.to_string());
        self
    }

    pub fn page(mut self, page: &str) -> Self {
        self.page = Some(page.to_string());
        self
    }

    fn validate(&self) -> Result<()> {
        if self.tag.is_none() && (self.user.is_none()) {
            return Err(Error::Generic(
                "Either 'mbid' or 'artist' must be provided.".to_string(),
            ));
        }
        Ok(())
    }

    pub async fn send(self) -> Result<APIResponse<Value>> {
        self.validate()?;

        let mut builder = ParameterBuilder::new();

        builder = builder
            .add_optional("user", self.user)
            .add_optional("tag", self.tag)
            .add_optional("taggingtype", self.taggingtype)
            .add_optional("limit", self.limit)
            .add_optional("page", self.page);

        let mut params = builder.build();

        let response = self
            .lastfm
            .send_request(self.method, &mut params, Method::GET)
            .await?;

        Ok(response)
    }
}
