use reqwest::Client as ReqwestClient;
use reqwest::Method;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

use crate::APIResponse;
use crate::{
    api::{Chart, Geo, LastfmMethod, Library, Tag, Track, User},
    error::{ApiError, Error, Result},
    Album, Artist, Auth,
};

pub const LASTFM_API_URL: &str = "http://ws.audioscrobbler.com/2.0/";

#[derive(Debug, Clone, Default)]
pub struct Lastfm {
    base_url: String,
    api_key: String,
    client: ReqwestClient,
    api_secret: String,
    sk: Option<String>,
}

#[derive(Default)]
pub struct LastfmBuilder {
    api_key: Option<String>,
    client: Option<ReqwestClient>,
    api_secret: Option<String>,
    sk: Option<String>,
}

impl LastfmBuilder {
    pub fn api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key.to_string());
        self
    }

    pub fn api_secret(mut self, api_secret: String) -> Self {
        self.api_secret = Some(api_secret.to_string());
        self
    }

    pub fn session_key(mut self, sk: String) -> Self {
        self.sk = Some(sk.to_string());
        self
    }

    pub fn client(mut self, client: ReqwestClient) -> Self {
        self.client = Some(client);
        self
    }

    pub fn build(self) -> Result<Lastfm> {
        Ok(Lastfm {
            api_secret: self.api_secret.expect("API_SECRET is Required."),
            api_key: self.api_key.expect("API_KEY is required."),
            client: self.client.unwrap_or_default(),
            base_url: LASTFM_API_URL.to_string(),
            sk: self.sk,
        })
    }
}

impl Lastfm {
    pub fn builder() -> LastfmBuilder {
        LastfmBuilder {
            api_key: None,
            client: None,
            api_secret: None,
            sk: None,
        }
    }

    pub fn get_client(&self) -> &ReqwestClient {
        &self.client
    }

    pub fn get_api_key(&self) -> String {
        self.api_key.clone()
    }

    pub fn get_api_secret(&self) -> String {
        self.api_secret.clone()
    }

    pub fn get_base_url(&self) -> &String {
        &self.base_url
    }

    pub fn get_sk(&self) -> String {
        self.sk.clone().expect("A user must be authenticated")
    }

    pub fn set_sk(&mut self, sk: String) -> &mut Self {
        self.sk = Some(sk.to_string());
        self
    }

    pub fn sign_api(&self, params: &mut HashMap<String, String>) -> String {
        let mut sorted_keys: Vec<String> = params.keys().cloned().collect();
        sorted_keys.sort();
        let mut concatenated_string = String::new();

        for key in sorted_keys {
            if let Some(value) = params.get(&key) {
                concatenated_string.push_str(&key);
                concatenated_string.push_str(value);
            }
        }
        concatenated_string.push_str(&self.get_api_secret());

        // println!(
        //     "Concatenated string for API signature: {}",
        //     concatenated_string
        // );

        let string_bytes = concatenated_string.as_bytes();
        let digest = md5::compute(string_bytes);

        format!("{:x}", digest)
    }

    async fn send_http_request(
        &self,
        params: &mut HashMap<String, String>,
        http_method: Method,
    ) -> Result<Value> {
        let url = self.get_base_url();

        let client = self.get_client();
        let response = match http_method {
            Method::GET => client.get(url).query(&params).send().await?,
            Method::POST => client.post(url).form(&params).send().await?,
            _ => return Err(Error::Generic("Unsupported HTTP method".to_string())),
        };

        let json_response: Value = response.json().await?;
        Ok(json_response)
    }

    // This function processes the response and returns either Success or Error
    async fn process_response<T>(&self, json_response: Value) -> Result<APIResponse<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        if json_response.get("error").is_some() {
            let api_error: ApiError = serde_json::from_value(json_response)?;
            return Ok(APIResponse::Error(api_error));
        }

        let response_data: T = serde_json::from_value(json_response)?;
        Ok(APIResponse::Success(response_data))
    }

    pub async fn send_request<T>(
        &self,
        method: LastfmMethod,
        params: &mut HashMap<String, String>,
        http_method: Method,
    ) -> Result<APIResponse<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        params.insert("method".to_string(), method.clone().into());
        params.insert("api_key".to_string(), self.get_api_key());
        if method.requires_auth() {
            params.insert("sk".to_string(), self.get_sk());
            let api_sig = self.sign_api(params);
            params.insert("api_sig".to_string(), api_sig);
        }
        params.insert("format".to_string(), "json".to_string());

        let json_response = self.send_http_request(params, http_method).await?;
        self.process_response(json_response).await
    }

    /// Creates a new `Album` instance for interacting with album-related methods.
    pub fn album(&self) -> Album {
        Album::new(self)
    }

    /// Creates a new `Artist` instance for interacting with artist-related methods.
    pub fn artist(&self) -> Artist {
        Artist::new(self)
    }

    /// Creates a new `Auth` instance for interacting with auth-related methods.
    pub fn auth(&self) -> Auth {
        Auth::new(self)
    }

    /// Creates a new `Chart` instance for interacting with chart-related methods.
    pub fn chart(&self) -> Chart {
        Chart::new(self)
    }

    /// Creates a new `Geo` instance for interacting with geo-related methods.
    pub fn geo(&self) -> Geo {
        Geo::new(self)
    }

    /// Creates a new `Library` instance for interacting with library-related methods.
    pub fn library(&self) -> Library {
        Library::new(self)
    }

    /// Creates a new `Tag` instance for interacting with tag-related methods.
    pub fn tag(&self) -> Tag {
        Tag::new(self)
    }

    /// Creates a new `Track` instance for interacting with track-related methods.
    pub fn track(&self) -> Track {
        Track::new(self)
    }

    /// Creates a new `User` instance for interacting with user-related methods.
    pub fn user(&self) -> User {
        User::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Client;
    use std::collections::HashMap;

    fn get_lastfm_instance() -> Lastfm {
        let client = Client::new();
        Lastfm::builder()
            .api_key("test_api_key".to_string())
            .api_secret("test_api_secret".to_string())
            .session_key("test_session_key".to_string())
            .client(client)
            .build()
            .unwrap()
    }

    #[tokio::test]
    async fn test_get_api_key() {
        let lastfm = get_lastfm_instance();
        assert_eq!(lastfm.get_api_key(), "test_api_key");
    }

    #[tokio::test]
    async fn test_get_api_secret() {
        let lastfm = get_lastfm_instance();
        assert_eq!(lastfm.get_api_secret(), "test_api_secret");
    }

    #[tokio::test]
    async fn test_get_base_url() {
        let lastfm = get_lastfm_instance();
        assert_eq!(lastfm.get_base_url(), "http://ws.audioscrobbler.com/2.0/");
    }

    #[tokio::test]
    async fn test_get_sk() {
        let lastfm = get_lastfm_instance();
        assert_eq!(lastfm.get_sk(), "test_session_key");
    }

    #[tokio::test]
    async fn test_set_sk() {
        let mut lastfm = get_lastfm_instance();
        lastfm.set_sk("new_session_key".to_string());
        assert_eq!(lastfm.get_sk(), "new_session_key");
    }

    #[tokio::test]
    async fn test_sign_api() {
        let lastfm = get_lastfm_instance();
        let mut params = HashMap::new();
        params.insert("method".to_string(), "test_method".to_string());
        params.insert("api_key".to_string(), lastfm.get_api_key());
        let signature = lastfm.sign_api(&mut params);
        assert!(!signature.is_empty());
    }
}
