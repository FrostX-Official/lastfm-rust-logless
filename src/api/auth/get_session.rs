use serde_json::Value;
use std::collections::HashMap;

use crate::error::ApiError;
use crate::{Error, Lastfm, Result};

// #[derive(Debug, Deserialize)]
// pub struct SessionResponse {
//     #[serde(rename = "session")]
//     pub session: Session,
// }
//
// #[derive(Debug, Deserialize)]
// pub struct Session {
//     pub name: String,
//     pub key: String,
//     pub subscriber: i32,
// }

#[derive(Debug)]
pub struct AuthGetSession<'a> {
    lastfm: &'a Lastfm,
    token: Option<String>,
    method: String,
}

impl<'a> AuthGetSession<'a> {
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        AuthGetSession {
            lastfm,
            token: None,
            method: "auth.getSession".into(),
        }
    }

    pub fn token(mut self, token: &str) -> Self {
        let token = token.replace("\"", "");
        self.token = Some(token.to_string());
        self
    }

    fn get_token(&self) -> String {
        self.token.clone().expect("A token is required!")
    }

    pub fn request_session_params(&self) -> HashMap<String, String> {
        let mut params: HashMap<String, String> = HashMap::new();

        params.insert("api_key".to_string(), self.lastfm.get_api_key());
        params.insert("token".to_string(), self.get_token());

        params
    }

    pub async fn send(self) -> Result<Value> {
        let mut session_params = self.request_session_params();

        session_params.insert("method".to_string(), self.method.to_string());
        let api_sig = self.lastfm.sign_api(&mut session_params);
        session_params.insert("api_sig".to_string(), api_sig);
        session_params.insert("format".to_string(), "json".to_string());

        println!("{session_params:?}");

        let response = self
            .lastfm
            .get_client()
            .get(self.lastfm.get_base_url())
            .query(&session_params)
            .send()
            .await?;

        let json_response: Value = response.json().await?;
        println!("{:?}", json_response);

        if json_response.get("error").is_some() {
            let api_error: ApiError = serde_json::from_value(json_response)?;
            return Err(Error::ApiError(api_error));
        }

        Ok(json_response)
    }
}
