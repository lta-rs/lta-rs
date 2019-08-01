//! Client for interacting with LTA API
use crate::utils::commons::Client;

/// A `Client` to make requests with
/// The `Client` holds a connection pool internally, so it is advised that you create one and reuse it
#[derive(Debug, Clone)]
pub struct LTAClient {
    api_key: Option<String>,
    client: reqwest::Client,
}

impl Client<reqwest::Client, reqwest::RequestBuilder> for LTAClient {
    type Output = LTAClient;

    fn new(api_key: Option<String>, client: reqwest::Client) -> LTAClient {
        LTAClient { api_key, client }
    }

    fn with_api_key<S>(api_key: S) -> LTAClient
    where
        S: Into<String>,
    {
        let api_key = api_key.into();

        let api_opt = if api_key.is_empty() {
            None
        } else {
            Some(api_key)
        };

        let client = reqwest::Client::new();

        LTAClient {
            api_key: api_opt,
            client,
        }
    }

    fn get_req_builder(&self, url: &str) -> reqwest::RequestBuilder {
        match &self.api_key {
            Some(s) => self.client.get(url).header("AccountKey", s.as_str()),
            None => panic!("API key not init!"),
        }
    }
}
