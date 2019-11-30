use crate::lta_client::LTAClient;
use lta_utils_commons::{reqwest, serde, Client};

pub mod bus;
pub mod crowd;
pub mod lta_client;
pub mod taxi;
pub mod traffic;
pub mod train;

pub(crate) fn build_req<T, M>(client: &LTAClient, url: &str) -> reqwest::Result<M>
where
    for<'de> T: serde::Deserialize<'de> + Into<M>,
{
    let req_builder = client.get_req_builder(url);
    req_builder.send()?.json().map(|f: T| f.into())
}

pub(crate) fn build_req_with_query<T, M, F>(
    client: &LTAClient,
    url: &str,
    query: F,
) -> reqwest::Result<M>
where
    F: FnOnce(reqwest::RequestBuilder) -> reqwest::RequestBuilder,
    for<'de> T: serde::Deserialize<'de> + Into<M>,
{
    let req_builder = client.get_req_builder(url);
    query(req_builder).send()?.json().map(|f: T| f.into())
}
