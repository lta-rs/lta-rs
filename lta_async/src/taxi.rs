//! All APIs pertaining to taxis

use crate::lta_client::LTAClient;
use crate::{build_req_async_with_skip, AsyncClient};
use async_trait::async_trait;
use lta_models::taxi::taxi_avail::TaxiAvailResp;
use lta_models::taxi::taxi_stands::{TaxiStand, TaxiStandsResp};
use lta_models::taxi::{taxi_avail, taxi_stands};
use lta_utils_commons::{Coordinates, LTAResult};

/// Returns location coordinates of all Taxis that are currently available for
/// hire. Does not include "Hired" or "Busy" Taxis.
///
/// **Update freq**: 1min
pub async fn get_taxi_avail(client: &LTAClient, skip: Option<u32>) -> LTAResult<Vec<Coordinates>> {
    build_req_async_with_skip::<TaxiAvailResp, _>(client, taxi_avail::URL, skip).await
}

/// Returns detailed information of Taxi stands, such as location and whether is it barrier free.
///
/// **Update freq**: Monthly
pub async fn get_taxi_stands(client: &LTAClient, skip: Option<u32>) -> LTAResult<Vec<TaxiStand>> {
    build_req_async_with_skip::<TaxiStandsResp, _>(client, taxi_stands::URL, skip).await
}

#[async_trait]
pub trait TaxiRequests {
    type Client: AsyncClient;

    async fn taxi_avail(c: &Self::Client, skip: Option<u32>) -> LTAResult<Vec<Coordinates>>;

    async fn taxi_stands(c: &Self::Client, skip: Option<u32>) -> LTAResult<Vec<TaxiStand>>;
}
