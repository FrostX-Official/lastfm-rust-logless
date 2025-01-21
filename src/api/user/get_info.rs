use crate::{
    api::{LastfmMethod, ParameterBuilder},
    APIResponse, Lastfm, Result,
};
use reqwest::Method;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct UserGetInfo<'a> {
    lastfm: &'a Lastfm,
    pub user: Option<String>,
    method: LastfmMethod,
}

impl<'a> UserGetInfo<'a> {
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        UserGetInfo {
            lastfm,
            user: None,
            method: LastfmMethod::UserGetInfo,
        }
    }

    pub fn user(mut self, user: &str) -> Self {
        self.user = Some(user.to_string());
        self
    }

    fn validate(&self) -> Result<()> {
        Ok(())
    }

    pub async fn send(self) -> Result<APIResponse<Value>> {
        self.validate()?;

        let mut builder = ParameterBuilder::new();

        builder = builder.add_optional("user", self.user);

        let mut params = builder.build();

        let response = self
            .lastfm
            .send_request(self.method, &mut params, Method::GET)
            .await?;

        Ok(response)
    }
}
