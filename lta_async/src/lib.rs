//! Async API calls for lta-rs. Currently uses async/await
//!

pub mod bus;
pub mod crowd;
pub mod lta_client;
pub mod taxi;
pub mod traffic;
pub mod train;

mod async_utils {
    use crate::lta_client::LTAClient;
    use futures::{compat::Future01CompatExt, Future};
    use futures01::Future as F01;
    use lta_utils_commons::{reqwest, serde, Client, LTAError};
    use reqwest::r#async::RequestBuilder;
    use std::fmt::Debug;

    pub(crate) fn build_req_async<T, M>(
        client: &LTAClient,
        url: &str,
    ) -> impl Future<Output = Result<M, LTAError>>
    where
        for<'de> T: serde::Deserialize<'de> + Into<M> + Debug,
    {
        let rb = client.get_req_builder(url);
        rb.send()
            .and_then(|mut f| f.json::<T>())
            .map(|f: T| f.into())
            .compat()
    }

    pub(crate) fn build_req_async_with_query<T, M, F>(
        client: &LTAClient,
        url: &str,
        query: F,
    ) -> impl Future<Output = Result<M, LTAError>>
    where
        F: FnOnce(RequestBuilder) -> RequestBuilder,
        for<'de> T: serde::Deserialize<'de> + Into<M> + Debug,
    {
        let rb = client.get_req_builder(url);
        query(rb)
            .send()
            .and_then(|mut f| f.json::<T>())
            .map(|f: T| f.into())
            .compat()
    }
}
