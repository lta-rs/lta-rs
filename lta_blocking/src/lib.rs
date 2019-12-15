//! Blocking API calls for lta-rs

use crate::lta_client::LTAClient;
use lta_utils_commons::{reqwest::blocking as rq_blocking, serde, Client, LTAResult};

pub mod bus;
pub mod crowd;
pub mod lta_client;
pub mod taxi;
pub mod traffic;
pub mod train;

pub(crate) fn build_req<T, M>(client: &LTAClient, url: &str) -> LTAResult<M>
where
    for<'de> T: serde::Deserialize<'de> + Into<M>,
{
    let rb = client.get_req_builder(url);
    rb.send()?.json().map(|f: T| f.into())
}

pub(crate) fn build_req_with_query<T, M, F>(
    client: &LTAClient,
    url: &str,
    query: F,
) -> LTAResult<M>
where
    F: FnOnce(rq_blocking::RequestBuilder) -> rq_blocking::RequestBuilder,
    for<'de> T: serde::Deserialize<'de> + Into<M>,
{
    let rb = client.get_req_builder(url);
    query(rb).send()?.json().map(|f: T| f.into())
}
