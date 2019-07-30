use reqwest::Error;
use tokio::prelude::Future;

use crate::r#async::lta_client::LTAClient;
use crate::taxi::taxi_avail::{Coordinates, TaxiAvailResp};
use crate::taxi::*;
use crate::utils::commons::Client;

/// Returns location coordinates of all Taxis that are currently available for
/// hire. Does not include "Hired" or "Busy" Taxis.
///
/// Update freq: 1min
pub fn get_passenger_vol_by(
    client: &LTAClient,
) -> impl Future<Item = Vec<Coordinates>, Error = Error> {
    let rb = client.get_req_builder(taxi_avail::URL);
    rb.send()
        .and_then(|mut f| f.json::<TaxiAvailResp>())
        .map(|r| r.value)
}
