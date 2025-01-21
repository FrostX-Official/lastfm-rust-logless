use crate::{
    api::{LastfmMethod, ParameterBuilder},
    APIResponse, Error, Lastfm, Result,
};
use reqwest::Method;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct UserGetWeeklyArtistChart<'a> {
    lastfm: &'a Lastfm,
    pub user: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    method: LastfmMethod,
}

impl<'a> UserGetWeeklyArtistChart<'a> {
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        UserGetWeeklyArtistChart {
            lastfm,
            user: None,
            from: None,
            to: None,
            method: LastfmMethod::UserGetWeeklyArtistChart,
        }
    }

    pub fn user(mut self, user: &str) -> Self {
        self.user = Some(user.to_string());
        self
    }

    pub fn from(mut self, from: &str) -> Self {
        self.from = Some(from.to_string());
        self
    }

    pub fn to(mut self, to: &str) -> Self {
        self.to = Some(to.to_string());
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
            .add_optional("from", self.from)
            .add_optional("to", self.to);

        let mut params = builder.build();

        let response = self
            .lastfm
            .send_request(self.method, &mut params, Method::GET)
            .await?;

        Ok(response)
    }
}
