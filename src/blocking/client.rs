//! Client for interacting with LTA API
use crate::blocking::Client;
use crate::{LTAError, LTAResult};
use reqwest::blocking::Client as RqClient;
use reqwest::blocking::RequestBuilder;

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
/// use lta::reqwest::blocking::ClientBuilder;
/// use lta::blocking::client::LTAClient;
/// use lta::Client;
/// use std::time::Duration;
///
///
/// fn my_custom_client() -> LTAClient {
///     let client = ClientBuilder::new()
///         .no_gzip()
///         .connect_timeout(Duration::new(420, 0))
///         .build()
///         .unwrap();
///
///     LTAClient::new(String::from("API_KEY"), client)
/// }
/// ```
#[derive(Debug, Clone)]
pub struct LTAClient {
    api_key: String,
    client: RqClient,
}

impl Client for LTAClient {
    type InternalClient = RqClient;
    type RB = RequestBuilder;

    fn new<S: Into<String>>(api_key: S, client: Self::InternalClient) -> LTAClient {
        let api_key = api_key.into();
        LTAClient { api_key, client }
    }

    fn with_api_key<S: Into<String>>(api_key: S) -> LTAResult<Self> {
        let api_key = api_key.into();

        if api_key.is_empty() {
            return Err(LTAError::InvalidAPIKey);
        }
        let client = RqClient::new();
        Ok(LTAClient { api_key, client })
    }

    fn req_builder(&self, url: &str) -> Self::RB {
        self.client
            .get(url)
            .header("AccountKey", self.api_key.as_str())
    }
}
