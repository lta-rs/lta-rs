use crate::models::prelude::*;
use crate::models::utils::Coordinates;
use crate::r#async::build_req_with_skip;
use crate::r#async::LTAClient;
use crate::{api_url, Client, LTAResult, Taxi};
use async_trait::async_trait;

/// All APIs pertaining to taxis
#[async_trait]
pub trait TaxiRequests<C: Client> {
    /// Returns location coordinates of all Taxis that are currently available for
    /// hire. Does not include "Hired" or "Busy" Taxis.
    ///
    /// **Update freq**: 1min
    async fn get_taxi_avail<S>(client: &C, skip: S) -> LTAResult<Vec<Coordinates>>
    where
        S: Into<Option<u32>> + Send;

    /// Returns detailed information of Taxi stands, such as location and whether is it barrier free.
    ///
    /// **Update freq**: Monthly
    async fn get_taxi_stands<S>(client: &C, skip: S) -> LTAResult<Vec<TaxiStand>>
    where
        S: Into<Option<u32>> + Send;
}

#[async_trait]
impl TaxiRequests<LTAClient> for Taxi {
    async fn get_taxi_avail<S>(client: &LTAClient, skip: S) -> LTAResult<Vec<Coordinates>>
    where
        S: Into<Option<u32>> + Send,
    {
        build_req_with_skip::<TaxiAvailResp, _, _>(
            client,
            api_url!("/Taxi-Availability"),
            skip.into(),
        )
        .await
    }

    async fn get_taxi_stands<S>(client: &LTAClient, skip: S) -> LTAResult<Vec<TaxiStand>>
    where
        S: Into<Option<u32>> + Send,
    {
        build_req_with_skip::<TaxiStandsResp, _, _>(client, api_url!("/TaxiStands"), skip.into())
            .await
    }
}
