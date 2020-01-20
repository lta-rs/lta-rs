//! Blocking API calls for lta-rs

use crate::lta_client::LTAClient;
use lta_utils_commons::{reqwest::blocking, serde, Client, LTAResult};

pub mod bus;
pub mod crowd;
pub mod lta_client;
pub mod taxi;
pub mod traffic;
pub mod train;

pub(crate) fn build_req_with_skip<T, M>(
    client: &LTAClient,
    url: &str,
    skip: Option<u32>,
) -> LTAResult<M>
where
    for<'de> T: serde::Deserialize<'de> + Into<M>,
{
    let skip = skip.unwrap_or(0);
    let rb = client.get_req_builder(url).query(&[("$skip", skip)]);
    rb.send()?.json().map(|f: T| f.into())
}

pub(crate) fn build_req_with_query<T, M, F>(client: &LTAClient, url: &str, query: F) -> LTAResult<M>
where
    F: FnOnce(blocking::RequestBuilder) -> blocking::RequestBuilder,
    for<'de> T: serde::Deserialize<'de> + Into<M>,
{
    let rb = client.get_req_builder(url);
    query(rb).send()?.json().map(|f: T| f.into())
}
