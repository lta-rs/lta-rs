use crate::models::prelude::*;
use crate::models::utils::Coordinates;
use crate::{api_url, Client, LTAResult};
use async_trait::async_trait;

use super::ClientExt;

/// All APIs pertaining to taxis
#[async_trait]
pub trait TaxiRequests<C: Client + ClientExt + Send + Sync> {
    /// Returns location coordinates of all Taxis that are currently available for
    /// hire. Does not include "Hired" or "Busy" Taxis.
    ///
    /// **Update freq**: 1min
    async fn get_taxi_avail<S>(client: &C, skip: S) -> LTAResult<Vec<Coordinates>>
    where
        S: Into<Option<u32>> + Send,
    {
        client
            .build_req_with_skip::<TaxiAvailResp, _>(api_url!("/Taxi-Availability"), skip.into())
            .await
    }

    /// Returns detailed information of Taxi stands, such as location and whether is it barrier free.
    ///
    /// **Update freq**: Monthly
    async fn get_taxi_stands<S>(client: &C, skip: S) -> LTAResult<Vec<TaxiStand>>
    where
        S: Into<Option<u32>> + Send,
    {
        client
            .build_req_with_skip::<TaxiStandsResp, _>(api_url!("/TaxiStands"), skip.into())
            .await
    }
}