use super::ClientExt;
use crate::models::prelude::*;
use crate::models::utils::Coordinates;
use crate::{Client, LTAResult};
use concat_string::concat_string;

/// All APIs pertaining to taxis
pub trait TaxiRequests<C: Client + ClientExt> {
    /// Returns location coordinates of all Taxis that are currently available for
    /// hire. Does not include "Hired" or "Busy" Taxis.
    ///
    /// **Update freq**: 1min
    async fn get_taxi_avail<S>(client: &C, skip: S) -> LTAResult<Vec<Coordinates>>
    where
        S: Into<Option<u32>>,
    {
        client
            .build_req_with_skip::<TaxiAvailResp, _>(
                &concat_string!(client.base_url(), "/Taxi-Availability"),
                skip.into(),
            )
            .await
    }

    /// Returns detailed information of Taxi stands, such as location and whether is it barrier free.
    ///
    /// **Update freq**: Monthly
    async fn get_taxi_stands<S>(client: &C, skip: S) -> LTAResult<Vec<TaxiStand>>
    where
        S: Into<Option<u32>>,
    {
        client
            .build_req_with_skip::<TaxiStandsResp, _>(
                &concat_string!(client.base_url(), "/TaxiStands"),
                skip.into(),
            )
            .await
    }
}
