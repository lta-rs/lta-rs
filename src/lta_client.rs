use reqwest::{Client, RequestBuilder};

#[derive(Debug, Clone)]
pub struct LTAClient {
    api_key: Option<String>,
    client: Client,
}

impl LTAClient {
    pub fn new(api_key: Option<String>, client: Client) -> Self {
        LTAClient { api_key, client }
    }

    pub fn with_api_key<S>(api_key: S) -> Self
    where
        S: Into<String>,
    {
        let api_key = api_key.into();

        let api_opt = if !api_key.is_empty() {
            Some(api_key)
        } else {
            None
        };

        LTAClient {
            api_key: api_opt,
            client: Client::new(),
        }
    }

    pub fn get_req_builder(&self, url: &str) -> RequestBuilder {
        match &self.api_key {
            Some(s) => self.client.get(url).header("AccountKey", s.as_str()),
            None => panic!("API key not init!"),
        }
    }
}
