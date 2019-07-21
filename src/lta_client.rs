use reqwest::{Client, RequestBuilder};

#[derive(Debug, Clone)]
pub struct LTAClient {
    api_key: Option<String>,
    client: Client,
}

impl LTAClient {
    pub fn new() -> Self {
        LTAClient {
            api_key: None,
            client: Client::new(),
        }
    }

    pub fn with_api_key<S>(mut self, api_key: S) -> Self
    where
        S: Into<String>,
    {
        let api_str = api_key.into();

        if !api_str.is_empty() {
            self.api_key = Some(api_str);
        }

        self
    }

    pub fn get_req_builder(&self, url: &str) -> RequestBuilder {
        match &self.api_key {
            Some(s) => self.client.get(url).header("AccountKey", s.as_str()),
            None => panic!("API key not init!"),
        }
    }
}
