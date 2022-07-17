use crate::blocking::{build_req_with_skip, LTAClient};
use crate::models::prelude::*;
use crate::models::utils::Coordinates;
use crate::{api_url, Client, LTAResult, Taxi};

/// All APIs pertaining to taxis
pub trait TaxiRequests<C: Client> {
    /// Returns location coordinates of all Taxis that are currently available for
    /// hire. Does not include "Hired" or "Busy" Taxis.
    ///
    /// **Update freq**: 1min
    fn get_taxi_avail<S>(client: &C, skip: S) -> LTAResult<Vec<Coordinates>>
    where
        S: Into<Option<u32>>;

    /// Returns detailed information of Taxi stands, such as location and whether is it barrier free.
    ///
    /// **Update freq**: Monthly
    fn get_taxi_stands<S>(client: &C, skip: S) -> LTAResult<Vec<TaxiStand>>
    where
        S: Into<Option<u32>>;
}

impl TaxiRequests<LTAClient> for Taxi {
    fn get_taxi_avail<S>(client: &LTAClient, skip: S) -> LTAResult<Vec<Coordinates>>
    where
        S: Into<Option<u32>>,
    {
        build_req_with_skip::<TaxiAvailResp, _, _>(
            client,
            api_url!("/Taxi-Availability"),
            skip.into(),
        )
    }

    fn get_taxi_stands<S>(client: &LTAClient, skip: S) -> LTAResult<Vec<TaxiStand>>
    where
        S: Into<Option<u32>>,
    {
        build_req_with_skip::<TaxiStandsResp, _, _>(client, api_url!("/TaxiStands"), skip.into())
    }
}
