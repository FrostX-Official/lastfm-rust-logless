use crate::{
    api::{LastfmMethod, ParameterBuilder},
    APIResponse, Error, Lastfm, Result,
};
use reqwest::Method;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct UserGetTopAlbums<'a> {
    lastfm: &'a Lastfm,
    pub user: Option<String>,
    pub period: Option<String>,
    pub limit: Option<u32>,
    pub page: Option<u32>,
    method: LastfmMethod,
}

impl<'a> UserGetTopAlbums<'a> {
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        UserGetTopAlbums {
            lastfm,
            user: None,
            period: None,
            limit: Some(50),
            page: Some(1),
            method: LastfmMethod::UserGetTopAlbums,
        }
    }

    pub fn user(mut self, user: &str) -> Self {
        self.user = Some(user.to_string());
        self
    }

    pub fn period(mut self, period: &str) -> Self {
        self.period = Some(period.to_string());
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
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
            .add_optional("period", self.period)
            .add_optional("limit", self.limit.map(|l| l.to_string()))
            .add_optional("page", self.page.map(|p| p.to_string()));

        let mut params = builder.build();

        let response = self
            .lastfm
            .send_request(self.method, &mut params, Method::GET)
            .await?;

        Ok(response)
    }
}
