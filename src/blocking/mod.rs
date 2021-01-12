pub mod bus;
pub mod client;

use crate::{LTAError, LTAResult};

pub use client::LTAClient;
use reqwest::blocking;

/// A `Client` to make requests with
/// The `Client` holds a connection pool internally, so it is advised that you create one and reuse it
pub trait Client: Sized {
    type InternalClient;
    type RB;

    /// General constructor for `Self`
    fn new<S: Into<String>>(api_key: S, client: Self::InternalClient) -> Self;
    /// This method not assign the `api_key` in struct if the provided key is empty or whitespaces
    /// Instead, assign `None`
    fn with_api_key<S: Into<String>>(api_key: S) -> LTAResult<Self>;
    fn req_builder(&self, url: &str) -> Self::RB;
}

pub(crate) fn build_req_with_skip<T, M, C>(client: &C, url: &str, skip: Option<u32>) -> LTAResult<M>
where
    C: Client<RB = blocking::RequestBuilder>,
    for<'de> T: serde::Deserialize<'de> + Into<M>,
{
    let skip = skip.unwrap_or(0);
    let rb = client.req_builder(url).query(&[("$skip", skip)]);
    rb.send()
        .map_err(LTAError::BackendError)?
        .json()
        .map(|f: T| f.into())
        .map_err(LTAError::BackendError)
}

pub(crate) fn build_req_with_query<T, M, F, C>(client: &C, url: &str, query: F) -> LTAResult<M>
where
    F: FnOnce(blocking::RequestBuilder) -> blocking::RequestBuilder,
    C: Client<RB = blocking::RequestBuilder>,
    for<'de> T: serde::Deserialize<'de> + Into<M>,
{
    let rb = client.req_builder(url);
    query(rb)
        .send()
        .map_err(LTAError::BackendError)?
        .json()
        .map(|f: T| f.into())
        .map_err(LTAError::BackendError)
}
