//! All API pertaining to transportation crowd
//!
use futures::Future;
use reqwest::Error;

use crate::crowd::passenger_vol::*;
use crate::r#async::lta_client::LTAClient;
use crate::utils::commons::build_req_async;

/// Creates a new client for every call
/// **Update freq**: By 15th of every month, the passenger volume for previous month data
/// will be generated
///
/// Note: Link will expire after 5mins!
pub fn get_passenger_vol_by(
    client: &LTAClient,
    vol_type: VolType,
) -> impl Future<Item = Vec<String>, Error = Error> {
    let url = match vol_type {
        VolType::BusStops => URL_BY_BUS_STOPS,
        VolType::OdBusStop => URL_BY_OD_BUS_STOPS,
        VolType::Train => URL_BY_TRAIN,
        VolType::OdTrain => URL_BY_OD_TRAIN,
    };
    build_req_async::<PassengerVolRawResp, _>(client, url)
}
