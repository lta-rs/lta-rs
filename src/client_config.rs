use std::sync::Mutex;

#[derive(Debug, Clone)]
pub struct ClientConfig {
    api_key: String,
    client: reqwest::Client,
}

lazy_static! {
    pub static ref CLIENT_CONFIG: Mutex<ClientConfig> = Mutex::new(ClientConfig::new());
}

impl ClientConfig {
    pub fn new() -> Self {
        ClientConfig {
            api_key: "".to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub fn with_api_key(&mut self, api_key: &str) {
        self.api_key = api_key.to_string();
    }

    pub fn get_req_builder(&self, url: &str) -> reqwest::RequestBuilder {
        self.client
            .get(url)
            .header("AccountKey", self.api_key.as_str())
    }
}