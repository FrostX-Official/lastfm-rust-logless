use crate::{
    api::{LastfmMethod, ParameterBuilder},
    APIResponse, Lastfm, Result,
};
use reqwest::Method;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct GeoGetTopTracks<'a> {
    lastfm: &'a Lastfm,
    pub country: Option<String>,
    pub limit: Option<i64>,
    pub page: Option<i64>,
    method: LastfmMethod,
}

impl<'a> GeoGetTopTracks<'a> {
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        GeoGetTopTracks {
            lastfm,
            country: None,
            limit: None,
            page: None,
            method: LastfmMethod::GeoGetTopTracks,
        }
    }

    pub fn country(mut self, country: String) -> Self {
        self.country = Some(country);
        self
    }

    /// The number of results to fetch per page. Defaults to 50.
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    /// The page number to fetch. Defaults to first page.
    pub fn page(mut self, page: i64) -> Self {
        self.page = Some(page);
        self
    }

    /// Validates the request parameters.
    fn validate(&self) -> Result<()> {
        Ok(())
    }

    /// Sends the request and retrieves the top tracks for the country.
    pub async fn send(self) -> Result<APIResponse<Value>> {
        self.validate()?;

        let mut builder = ParameterBuilder::new();

        builder = builder
            .add(
                "country",
                self.country.expect("The country name is required!"),
            )
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
