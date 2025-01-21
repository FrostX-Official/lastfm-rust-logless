use crate::{
    api::{LastfmMethod, ParameterBuilder},
    APIResponse, Lastfm, Result,
};
use reqwest::Method;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct TagGetWeeklyChartList<'a> {
    lastfm: &'a Lastfm,
    pub tag: Option<String>,
    method: LastfmMethod,
}

impl<'a> TagGetWeeklyChartList<'a> {
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        TagGetWeeklyChartList {
            lastfm,
            tag: None,
            method: LastfmMethod::TagGetWeeklyChartList,
        }
    }

    pub fn tag(mut self, tag: &str) -> Self {
        self.tag = Some(tag.to_string());
        self
    }

    fn validate(&self) -> Result<()> {
        Ok(())
    }

    pub async fn send(self) -> Result<APIResponse<Value>> {
        self.validate()?;

        let mut builder = ParameterBuilder::new();

        builder = builder.add_optional("tag", self.tag);

        let mut params = builder.build();

        let response = self
            .lastfm
            .send_request(self.method, &mut params, Method::GET)
            .await?;

        Ok(response)
    }
}
