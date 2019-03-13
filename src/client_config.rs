use std::sync::Mutex;

#[derive(Debug, Clone)]
pub struct ClientConfig {
    api_key: Option<String>,
    client: reqwest::Client,
}

lazy_static! {
    pub static ref CLIENT_CONFIG: Mutex<ClientConfig> = Mutex::new(ClientConfig::new());
}

impl ClientConfig {
    pub fn new() -> Self {
        ClientConfig {
            api_key: None,
            client: reqwest::Client::new(),
        }
    }

    pub fn with_api_key(&mut self, api_key: &str) {
        self.api_key = Some(api_key.to_string());
    }

    pub fn get_req_builder(&self, url: &str) -> reqwest::RequestBuilder {
        let api_key = self.api_key.clone().unwrap();
        self.client
            .get(url)
            .header("AccountKey", api_key.as_str())
    }
}