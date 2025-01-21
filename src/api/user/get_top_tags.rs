use crate::{
    api::{LastfmMethod, ParameterBuilder},
    APIResponse, Error, Lastfm, Result,
};
use reqwest::Method;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct UserGetTopTags<'a> {
    lastfm: &'a Lastfm,
    pub user: Option<String>,
    pub limit: Option<u32>,
    method: LastfmMethod,
}

impl<'a> UserGetTopTags<'a> {
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        UserGetTopTags {
            lastfm,
            user: None,
            limit: Some(50),
            method: LastfmMethod::UserGetTopTags,
        }
    }

    pub fn user(mut self, user: &str) -> Self {
        self.user = Some(user.to_string());
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    fn validate(&self) -> Result<()> {
        if self.user.is_none() {
            return Err(Error::Generic("Username is required.".to_string()));
        }
        Ok(())
    }

    pub async fn send(self) -> Result<APIResponse<Value>> {
        self.validate()?;

        let mut builder = ParameterBuilder::new();

        builder = builder
            .add_optional("user", self.user)
            .add_optional("limit", self.limit.map(|l| l.to_string()));

        let mut params = builder.build();

        let response = self
            .lastfm
            .send_request(self.method, &mut params, Method::GET)
            .await?;

        Ok(response)
    }
}
