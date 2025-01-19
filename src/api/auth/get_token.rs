use reqwest::Method;
use serde_json::Value;
use std::collections::HashMap;

use crate::{Lastfm, Result};

// #[derive(Debug, Deserialize)]
// pub struct TokenResponse {
//     #[serde(rename = "token")]
//     pub token: String,
// }

#[derive(Debug)]
pub struct AuthGetToken<'a> {
    lastfm: &'a Lastfm,
    method: String,
}

impl<'a> AuthGetToken<'a> {
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        AuthGetToken {
            lastfm,
            method: "auth.getToken".into(),
        }
    }

    pub fn request_token_params(&self) -> HashMap<String, String> {
        let mut params: HashMap<String, String> = HashMap::new();

        params.insert("api_key".to_string(), self.lastfm.get_api_key());

        params
    }

    pub async fn send(self) -> Result<Value> {
        let mut token_params = self.request_token_params();
        let response = self
            .lastfm
            .send_request(&self.method, &mut token_params, Method::GET, false)
            .await?;

        Ok(response)
    }
}
