use reqwest::Client as ReqwestClient;
use reqwest::Method;
use serde_json::Value;
use std::collections::HashMap;

use crate::{
    api::{Chart, Geo, LastfmMethod, Library},
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

    pub async fn send_request(
        &self,
        method: LastfmMethod,
        params: &mut std::collections::HashMap<String, String>,
        http_method: Method,
    ) -> Result<Value> {
        // TODO: Rate Limiting
        // let rate_limiter = &self.lastfm.rate_limiter;
        // rate_limiter.until_ready().await;

        params.insert("method".to_string(), method.clone().into());
        params.insert("api_key".to_string(), self.get_api_key());

        if method.requires_auth() {
            params.insert("sk".to_string(), self.get_sk());
            let api_sig = self.sign_api(params);
            params.insert("api_sig".to_string(), api_sig);
        }

        params.insert("format".to_string(), "json".to_string());
        // println!("PARAMS: {:?}", params);

        let url = self.get_base_url();

        let response = match http_method {
            Method::GET => self.get_client().get(url).query(&params).send().await?,
            Method::POST => self.get_client().post(url).form(&params).send().await?,
            _ => return Err(Error::Generic("Unsupported HTTP method".to_string())),
        };

        let json_response: Value = response.json().await?;

        if json_response.get("error").is_some() {
            let api_error: ApiError = serde_json::from_value(json_response)?;
            return Err(Error::ApiError(api_error));
        }

        Ok(json_response)
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
}
