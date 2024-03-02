use crate::models::prelude::*;
use crate::models::utils::Coordinates;
use crate::{Client, LTAResult};
use concat_string::concat_string;

use super::ClientExt;

/// All APIs pertaining to taxis
pub trait TaxiRequests<C: Client + ClientExt> {
    /// Returns location coordinates of all Taxis that are currently available for
    /// hire. Does not include "Hired" or "Busy" Taxis.
    ///
    /// **Update freq**: 1min
    fn get_taxi_avail(client: &C, skip: impl Into<Option<u32>>) -> LTAResult<Vec<Coordinates>> {
        client.build_req_with_skip::<TaxiAvailResp, _>(
            &concat_string!(client.base_url(), "/Taxi-Availability"),
            skip.into(),
        )
    }

    /// Returns detailed information of Taxi stands, such as location and whether is it barrier free.
    ///
    /// **Update freq**: Monthly
    fn get_taxi_stands(client: &C, skip: impl Into<Option<u32>>) -> LTAResult<Vec<TaxiStand>> {
        client.build_req_with_skip::<TaxiStandsResp, _>(
            &concat_string!(client.base_url(), "/TaxiStands"),
            skip.into(),
        )
    }
}
