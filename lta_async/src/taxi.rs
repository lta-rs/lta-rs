//! All APIs pertaining to taxis

use crate::async_utils::build_req_async;
use crate::lta_client::LTAClient;
use lta_models::taxi::taxi_avail;
use lta_models::taxi::taxi_avail::TaxiAvailResp;
use lta_utils_commons::{Coordinates, LTAResult};

/// Returns location coordinates of all Taxis that are currently available for
/// hire. Does not include "Hired" or "Busy" Taxis.
///
/// **Update freq**: 1min
pub async fn get_passenger_vol_by(client: &LTAClient) -> LTAResult<Vec<Coordinates>> {
    build_req_async::<TaxiAvailResp, _>(client, taxi_avail::URL).await
}
