use crate::{blocking::LTAClient, Client, LTAError, LTAResult};
use ureq::{Agent, Request};

impl Client for LTAClient<Agent> {
    type InternalClient = Agent;
    type RB = Request;

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

        let client = Agent::new();

        Ok(LTAClient {
            api_key,
            client,
            base_url,
        })
    }

    fn req_builder(&self, url: &str) -> Self::RB {
        self.client
            .get(url)
            .set("AccountKey", self.api_key.as_str())
    }

    fn base_url(&self) -> &str {
        &self.base_url
    }
}
