//! All APIs pertaining to taxis

use crate::build_req_with_skip;
use crate::lta_client::LTAClient;
use lta_models::taxi::taxi_avail::TaxiAvailResp;
use lta_models::taxi::taxi_stands::{TaxiStand, TaxiStandsResp};
use lta_models::taxi::{taxi_avail, taxi_stands};
use lta_utils_commons::{Coordinates, LTAResult};

/// Returns location coordinates of all Taxis that are currently available for
/// hire. Does not include "Hired" or "Busy" Taxis.
///
/// **Update freq**: 1min
pub fn get_taxi_avail(client: &LTAClient, skip: Option<u32>) -> LTAResult<Vec<Coordinates>> {
    build_req_with_skip::<TaxiAvailResp, _>(client, taxi_avail::URL, skip)
}

/// Returns detailed information of Taxi stands, such as location and whether is it barrier free.
///
/// **Update freq**: Monthly
pub fn get_taxi_stands(client: &LTAClient, skip: Option<u32>) -> LTAResult<Vec<TaxiStand>> {
    build_req_with_skip::<TaxiStandsResp, _>(client, taxi_stands::URL, skip)
}
