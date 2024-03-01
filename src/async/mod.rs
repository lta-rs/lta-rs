pub mod bus;
pub mod client;
pub mod crowd;
pub mod facility;
pub mod geo;
pub mod taxi;
pub mod traffic;
pub mod train;

use async_trait::async_trait;

use crate::{Client, LTAResult};

pub use crate::r#async::client::LTAClient;

pub mod prelude {
    pub use crate::r#async::{
        bus::BusRequests, crowd::CrowdRequests, facility::FacilityRequests, geo::GeoRequests,
        taxi::TaxiRequests, traffic::TrafficRequests, train::TrainRequests,
    };
}

#[async_trait]
pub trait ClientExt: Client {
    async fn build_req_with_skip<T, T2>(&self, url: &str, skip: Option<u32>) -> LTAResult<T2>
    where
        for<'de> T: serde::Deserialize<'de> + Into<T2>;

    async fn build_req_with_query<T, T2, F>(&self, url: &str, query: F) -> LTAResult<T2>
    where
        F: Send + FnOnce(Self::RB) -> Self::RB,
        for<'de> T: serde::Deserialize<'de> + Into<T2>;
}
