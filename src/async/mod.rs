pub mod bus;
pub mod client;

use crate::{Client, LTAError, LTAResult};

pub(crate) async fn build_req_with_skip<T, M, C>(
    client: &C,
    url: &str,
    skip: Option<u32>,
) -> LTAResult<M>
where
    C: Client<RB = reqwest::RequestBuilder>,
    for<'de> T: serde::Deserialize<'de> + Into<M>,
{
    let skip = skip.unwrap_or(0);
    let rb = client.req_builder(url).query(&[("$skip", skip)]);
    rb.send()
        .await
        .map_err(LTAError::BackendError)?
        .json()
        .await
        .map(|f: T| f.into())
        .map_err(LTAError::BackendError)
}

pub(crate) async fn build_req_with_query<T, M, F, C>(
    client: &C,
    url: &str,
    query: F,
) -> LTAResult<M>
where
    F: FnOnce(reqwest::RequestBuilder) -> reqwest::RequestBuilder,
    C: Client<RB = reqwest::RequestBuilder>,
    for<'de> T: serde::Deserialize<'de> + Into<M>,
{
    let rb = client.req_builder(url);
    query(rb)
        .send()
        .await
        .map_err(LTAError::BackendError)?
        .json()
        .await
        .map(|f: T| f.into())
        .map_err(LTAError::BackendError)
}
