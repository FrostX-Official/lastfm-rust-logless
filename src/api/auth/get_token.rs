use crate::AuthGetTokenResponse;
use crate::{api::LastfmMethod, APIResponse, Lastfm, Result};
use reqwest::Method;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AuthGetToken<'a> {
    lastfm: &'a Lastfm,
    method: LastfmMethod,
}

impl<'a> AuthGetToken<'a> {
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        AuthGetToken {
            lastfm,
            method: LastfmMethod::AuthGetToken,
        }
    }

    pub fn request_token_params(&self) -> HashMap<String, String> {
        let mut params: HashMap<String, String> = HashMap::new();

        params.insert("api_key".to_string(), self.lastfm.get_api_key());

        params
    }

    pub async fn send(self) -> Result<APIResponse<AuthGetTokenResponse>> {
        let mut token_params = self.request_token_params();
        let response = self
            .lastfm
            .send_request(self.method, &mut token_params, Method::GET)
            .await?;

        Ok(response)
    }
}
