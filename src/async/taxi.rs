//! All API pertaining to taxis
use futures::Future;
use reqwest::Error;

use crate::r#async::lta_client::LTAClient;
use crate::taxi::taxi_avail::{Coordinates, TaxiAvailResp};
use crate::taxi::*;
use crate::utils::commons::build_req_async;

/// Returns location coordinates of all Taxis that are currently available for
/// hire. Does not include "Hired" or "Busy" Taxis.
///
/// **Update freq**: 1min
pub fn get_passenger_vol_by(
    client: &LTAClient,
) -> impl Future<Item = Vec<Coordinates>, Error = Error> {
    build_req_async::<TaxiAvailResp, _>(client, taxi_avail::URL)
}
