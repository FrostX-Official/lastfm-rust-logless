use crate::{
    api::{LastfmMethod, ParameterBuilder},
    APIResponse, Lastfm, Result,
};
use reqwest::Method;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct UserGetFriends<'a> {
    lastfm: &'a Lastfm,
    pub user: Option<String>,
    pub recenttracks: Option<String>,
    pub limit: Option<String>,
    pub page: Option<String>,
    method: LastfmMethod,
}

impl<'a> UserGetFriends<'a> {
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        UserGetFriends {
            lastfm,
            user: None,
            recenttracks: None,
            limit: None,
            page: None,
            method: LastfmMethod::UserGetFriends,
        }
    }

    pub fn user(mut self, user: &str) -> Self {
        self.user = Some(user.to_string());
        self
    }

    pub fn recenttracks(mut self, recenttracks: &str) -> Self {
        self.recenttracks = Some(recenttracks.to_string());
        self
    }

    pub fn page(mut self, page: &str) -> Self {
        self.page = Some(page.to_string());
        self
    }

    pub fn limit(mut self, limit: bool) -> Self {
        self.limit = Some(limit.to_string());
        self
    }

    fn validate(&self) -> Result<()> {
        Ok(())
    }

    pub async fn send(self) -> Result<APIResponse<Value>> {
        self.validate()?;

        let mut builder = ParameterBuilder::new();

        builder = builder
            .add_optional("user", self.user)
            .add_optional("recenttracks", self.recenttracks)
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
