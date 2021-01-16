use crate::blocking::{build_req_with_skip, Client, LTAClient, Taxi};
use crate::models::prelude::*;
use crate::models::utils::Coordinates;
use crate::{LTAResult, api_url};

/// All APIs pertaining to taxis
pub trait TaxiRequests<C: Client> {
    /// Returns location coordinates of all Taxis that are currently available for
    /// hire. Does not include "Hired" or "Busy" Taxis.
    ///
    /// **Update freq**: 1min
    fn get_taxi_avail(client: &C, skip: Option<u32>) -> LTAResult<Vec<Coordinates>>;

    /// Returns detailed information of Taxi stands, such as location and whether is it barrier free.
    ///
    /// **Update freq**: Monthly
    fn get_taxi_stands(client: &C, skip: Option<u32>) -> LTAResult<Vec<TaxiStand>>;
}

impl TaxiRequests<LTAClient> for Taxi {
    fn get_taxi_avail(client: &LTAClient, skip: Option<u32>) -> LTAResult<Vec<Coordinates>> {
        build_req_with_skip::<TaxiAvailResp, _, _>(client, api_url!("/Taxi-Availability"), skip)
    }

    fn get_taxi_stands(client: &LTAClient, skip: Option<u32>) -> LTAResult<Vec<TaxiStand>> {
        build_req_with_skip::<TaxiStandsResp, _, _>(client, api_url!("/TaxiStands"), skip)
    }
}
