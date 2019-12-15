//! Async API calls for lta-rs. Currently uses async/await
//!

pub mod bus;
pub mod crowd;
pub mod lta_client;
pub mod taxi;
pub mod traffic;
pub mod train;

use crate::lta_client::LTAClient;
use lta_utils_commons::{reqwest, serde, Client, LTAError};
use reqwest::RequestBuilder;

pub(crate) async fn build_req_async<T, M>(client: &LTAClient, url: &str) -> Result<M, LTAError>
where
    for<'de> T: serde::Deserialize<'de> + Into<M>,
{
    let rb = client.get_req_builder(url);
    rb.send().await?.json::<T>().await.map(|f| f.into())
}

pub(crate) async fn build_req_async_with_query<T, M, F>(
    client: &LTAClient,
    url: &str,
    query: F,
) -> Result<M, LTAError>
where
    F: FnOnce(RequestBuilder) -> RequestBuilder,
    for<'de> T: serde::Deserialize<'de> + Into<M>,
{
    let rb = client.get_req_builder(url);
    query(rb)
        .send()
        .await?
        .json::<T>()
        .await
        .map(|f: T| f.into())
}
