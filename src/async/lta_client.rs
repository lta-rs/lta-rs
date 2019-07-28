use reqwest::r#async::Client as AsyncClient;
use reqwest::r#async::RequestBuilder as AsyncReqBuilder;

use crate::utils::commons::Client;

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
