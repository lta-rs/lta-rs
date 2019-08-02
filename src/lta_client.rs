//! Client for interacting with LTA API
use crate::utils::commons::Client;

/// A `Client` to make requests with
/// The `Client` holds a connection pool internally, so it is advised that you create one and reuse it
/// There are some instance where you might need to customise your client due to certain limitations.
///
/// The `Client` trait has a general constructor method and you should use the `reqwest` re-export
/// to build you own customised client from the ground up.
///
/// Take a look at the reqwest documentation on how to build your own client
///
/// ## Example
/// ```rust
/// use std::time::Duration;
/// use lta::reqwest::ClientBuilder;
/// use lta::lta_client::LTAClient;
/// use lta::utils::commons::Client;
///
/// fn my_custom_client() -> LTAClient {
///     let client = ClientBuilder::new()
///         .gzip(true)
///         .connect_timeout(Some(Duration::new(420,0)))
///         .build()
///         .unwrap();
///
///     LTAClient::new(Some("api_key".to_string()), client)     
/// }
/// ```
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
