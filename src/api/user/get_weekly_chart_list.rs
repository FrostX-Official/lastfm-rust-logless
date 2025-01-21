use crate::{
    api::{LastfmMethod, ParameterBuilder},
    APIResponse, Error, Lastfm, Result,
};
use reqwest::Method;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct UserGetWeeklyChartList<'a> {
    lastfm: &'a Lastfm,
    pub user: Option<String>,
    method: LastfmMethod,
}

impl<'a> UserGetWeeklyChartList<'a> {
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        UserGetWeeklyChartList {
            lastfm,
            user: None,
            method: LastfmMethod::UserGetWeeklyChartList,
        }
    }

    pub fn user(mut self, user: &str) -> Self {
        self.user = Some(user.to_string());
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

        builder = builder.add("user", self.user.unwrap());

        let mut params = builder.build();

        let response = self
            .lastfm
            .send_request(self.method, &mut params, Method::GET)
            .await?;

        Ok(response)
    }
}
