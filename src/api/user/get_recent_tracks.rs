use crate::{
    api::{LastfmMethod, ParameterBuilder},
    APIResponse, Error, Lastfm, Result,
};
use reqwest::Method;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct UserGetRecentTracks<'a> {
    lastfm: &'a Lastfm,
    pub username: Option<String>,
    pub limit: Option<u32>,
    pub page: Option<u32>,
    pub from: Option<u64>,
    pub extended: Option<u32>,
    pub to: Option<u64>,
    method: LastfmMethod,
}

impl<'a> UserGetRecentTracks<'a> {
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        UserGetRecentTracks {
            lastfm,
            username: None,
            limit: Some(50),
            page: Some(1),
            from: None,
            extended: None,
            to: None,
            method: LastfmMethod::UserGetRecentTracks,
        }
    }

    pub fn username(mut self, username: &str) -> Self {
        self.username = Some(username.to_string());
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

    pub fn from(mut self, from: u64) -> Self {
        self.from = Some(from);
        self
    }

    pub fn extended(mut self, extended: u32) -> Self {
        self.extended = Some(extended);
        self
    }

    pub fn to(mut self, to: u64) -> Self {
        self.to = Some(to);
        self
    }

    fn validate(&self) -> Result<()> {
        if self.username.is_none() {
            return Err(Error::Generic("Username is required.".to_string()));
        }
        Ok(())
    }

    pub async fn send(self) -> Result<APIResponse<Value>> {
        self.validate()?;

        let mut builder = ParameterBuilder::new();

        // Add only the necessary parameters
        builder = builder
            .add_optional("user", self.username)
            .add_optional("limit", self.limit.map(|l| l.to_string()))
            .add_optional("page", self.page.map(|p| p.to_string()))
            .add_optional("from", self.from.map(|f| f.to_string()))
            .add_optional("extended", self.extended.map(|e| e.to_string()))
            .add_optional("to", self.to.map(|t| t.to_string()));

        let mut params = builder.build();

        // Send the request
        let response = self
            .lastfm
            .send_request(self.method, &mut params, Method::GET)
            .await?;

        Ok(response)
    }
}
