//! All APIs pertaining to transportation crowd

use crate::lta_client::LTAClient;
use crate::{build_req, build_req_with_query};
use lta_models::crowd::passenger_vol;
use lta_models::crowd::passenger_vol::VolType;
use lta_utils_commons::{chrono::NaiveDate, LTAResult};

pub fn get_passenger_vol_by(
    client: &LTAClient,
    vol_type: passenger_vol::VolType,
    date: Option<NaiveDate>,
) -> LTAResult<Vec<String>> {
    let fmt_date = date.map(|f| f.format(passenger_vol::FORMAT).to_string());

    let url = match vol_type {
        VolType::BusStops => passenger_vol::URL_BY_BUS_STOPS,
        VolType::OdBusStop => passenger_vol::URL_BY_OD_BUS_STOPS,
        VolType::Train => passenger_vol::URL_BY_TRAIN,
        VolType::OdTrain => passenger_vol::URL_BY_OD_TRAIN,
    };

    match fmt_date {
        Some(nd) => {
            build_req_with_query::<passenger_vol::PassengerVolRawResp, _, _>(client, url, |rb| {
                rb.query(&[("Date", nd)])
            })
        }
        None => build_req::<passenger_vol::PassengerVolRawResp, _>(client, url),
    }
}
