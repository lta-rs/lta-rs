use lta_utils_commons::{reqwest::blocking as rq_blocking, Client};


#[derive(Debug, Clone)]
pub struct LTAClient {
    api_key: Option<String>,
    client: rq_blocking::Client,
}

impl Client<rq_blocking::Client, rq_blocking::RequestBuilder> for LTAClient {
    fn new(api_key: Option<String>, client: rq_blocking::Client) -> LTAClient {
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

        let client = rq_blocking::Client::new();

        LTAClient {
            api_key: api_opt,
            client,
        }
    }

    fn get_req_builder(&self, url: &str) -> rq_blocking::RequestBuilder {
        let api_key = self.api_key.as_ref().expect("Empty API KEY!");
        self.client.get(url).header("AccountKey", api_key.as_str())
    }
}
