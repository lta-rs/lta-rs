use crate::{Client, blocking::LTAClient, LTAResult, LTAError};
use reqwest::blocking::{Client as ReqwestBlocking, RequestBuilder};

impl Client for LTAClient<ReqwestBlocking> {
    type InternalClient = ReqwestBlocking;
    type RB = RequestBuilder;

    fn new(api_key: impl Into<String>, client: Self::InternalClient) -> Self {
        let api_key = api_key.into();
        LTAClient { api_key, client }
    }

    fn with_api_key(api_key: impl Into<String>) -> LTAResult<Self> {
        let api_key = api_key.into();

        if api_key.is_empty() {
            return Err(LTAError::InvalidAPIKey);
        }

        let client = ReqwestBlocking::new();
        Ok(LTAClient { api_key, client })
    }

    fn req_builder(&self, url: &str) -> Self::RB {
        self.client
            .get(url)
            .header("AccountKey", self.api_key.as_str())
    }
}
