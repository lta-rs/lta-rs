use crate::{blocking::LTAClient, Client, LTAError, LTAResult};
use reqwest::blocking::{Client as ReqwestBlocking, RequestBuilder};

impl Client for LTAClient<ReqwestBlocking> {
    type InternalClient = ReqwestBlocking;
    type RB = RequestBuilder;

    fn new(
        api_key: impl Into<String>,
        client: Self::InternalClient,
        base_url: impl Into<String>,
    ) -> Self {
        let api_key = api_key.into();
        let base_url = base_url.into();

        LTAClient {
            api_key,
            client,
            base_url,
        }
    }

    fn with_api_key(api_key: impl Into<String>, base_url: impl Into<String>) -> LTAResult<Self> {
        let api_key = api_key.into();
        let base_url = base_url.into();

        if api_key.is_empty() {
            return Err(LTAError::InvalidAPIKey);
        }

        let client = ReqwestBlocking::new();

        Ok(LTAClient {
            api_key,
            client,
            base_url,
        })
    }

    fn req_builder(&self, url: &str) -> Self::RB {
        self.client
            .get(url)
            .header("AccountKey", self.api_key.as_str())
    }

    fn base_url(&self) -> &str {
        &self.base_url
    }
}
