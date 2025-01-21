use crate::{
    api::{LastfmMethod, ParameterBuilder},
    APIResponse, Error, Lastfm, Result,
};
use reqwest::Method;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct UserGetLovedTracks<'a> {
    lastfm: &'a Lastfm,
    pub user: Option<String>,
    pub limit: Option<i64>,
    pub page: Option<i64>,
    method: LastfmMethod,
}

impl<'a> UserGetLovedTracks<'a> {
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        UserGetLovedTracks {
            lastfm,
            user: None,
            limit: None,
            page: None,
            method: LastfmMethod::TrackSearch,
        }
    }

    pub fn user(mut self, user: &str) -> Self {
        self.user = Some(user.to_string());
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
        if self.user.is_none() || self.user.as_ref().unwrap().is_empty() {
            return Err(Error::Generic("The user name is required.".to_string()));
        }

        Ok(())
    }

    pub async fn send(self) -> Result<APIResponse<Value>> {
        self.validate()?;

        let mut builder = ParameterBuilder::new();

        builder = builder
            .add_optional("user", self.user)
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
