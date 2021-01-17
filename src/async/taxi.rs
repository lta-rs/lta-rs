use crate::r#async::{build_req_with_skip};
use crate::models::prelude::*;
use crate::models::utils::Coordinates;
use crate::{api_url, LTAResult, Taxi, Client};
use crate::r#async::LTAClient;
use async_trait::async_trait;

/// All APIs pertaining to taxis
#[async_trait]
pub trait TaxiRequests<C: Client> {
    /// Returns location coordinates of all Taxis that are currently available for
    /// hire. Does not include "Hired" or "Busy" Taxis.
    ///
    /// **Update freq**: 1min
    async fn get_taxi_avail(client: &C, skip: Option<u32>) -> LTAResult<Vec<Coordinates>>;

    /// Returns detailed information of Taxi stands, such as location and whether is it barrier free.
    ///
    /// **Update freq**: Monthly
    async fn get_taxi_stands(client: &C, skip: Option<u32>) -> LTAResult<Vec<TaxiStand>>;
}

#[async_trait]
impl TaxiRequests<LTAClient> for Taxi {
    async fn get_taxi_avail(client: &LTAClient, skip: Option<u32>) -> LTAResult<Vec<Coordinates>> {
        build_req_with_skip::<TaxiAvailResp, _, _>(client, api_url!("/Taxi-Availability"), skip).await
    }

    async fn get_taxi_stands(client: &LTAClient, skip: Option<u32>) -> LTAResult<Vec<TaxiStand>> {
        build_req_with_skip::<TaxiStandsResp, _, _>(client, api_url!("/TaxiStands"), skip).await
    }
}

