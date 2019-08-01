use reqwest::r#async::{Client as AsyncClient, RequestBuilder as AsyncReqBuilder};

use crate::utils::commons::Client;

/// An async version of the normal `LTAClient`
/// A `Client` to make requests with
/// The `Client` holds a connection pool internally, so it is advised that you create one and reuse it
#[derive(Debug, Clone)]
pub struct LTAClient {
    api_key: Option<String>,
    client: AsyncClient,
}

impl Client<AsyncClient, AsyncReqBuilder> for LTAClient {
    type Output = LTAClient;

    fn new(api_key: Option<String>, client: AsyncClient) -> LTAClient {
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

        let client = AsyncClient::new();

        LTAClient {
            api_key: api_opt,
            client,
        }
    }

    fn get_req_builder(&self, url: &str) -> AsyncReqBuilder {
        match &self.api_key {
            Some(s) => self.client.get(url).header("AccountKey", s.as_str()),
            None => panic!("API key not init!"),
        }
    }
}
