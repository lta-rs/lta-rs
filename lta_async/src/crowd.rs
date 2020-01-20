//! All APIs pertaining to transportation crowd

use crate::lta_client::LTAClient;
use crate::{build_req_async_with_query, build_req_async_with_skip};
use lta_models::crowd::passenger_vol::{
    PassengerVolRawResp, VolType, FORMAT, URL_BY_BUS_STOPS, URL_BY_OD_BUS_STOPS, URL_BY_OD_TRAIN,
    URL_BY_TRAIN,
};
use lta_utils_commons::{chrono::NaiveDate, LTAResult};

/// Creates a new client for every call
/// **Update freq**: By 15th of every month, the passenger volume for previous month data
/// will be generated
///
/// Note: Link will expire after 5mins!
pub async fn get_passenger_vol_by(
    client: &LTAClient,
    vol_type: VolType,
    date: Option<NaiveDate>,
    skip: Option<u32>,
) -> LTAResult<Vec<String>> {
    let fmt_date = date.map(|f| f.format(FORMAT).to_string());

    let url = match vol_type {
        VolType::BusStops => URL_BY_BUS_STOPS,
        VolType::OdBusStop => URL_BY_OD_BUS_STOPS,
        VolType::Train => URL_BY_TRAIN,
        VolType::OdTrain => URL_BY_OD_TRAIN,
    };

    match fmt_date {
        Some(nd) => {
            build_req_async_with_query::<PassengerVolRawResp, _, _>(client, url, |rb| {
                rb.query(&[("Date", nd)])
            })
            .await
        }
        None => build_req_async_with_skip::<PassengerVolRawResp, _>(client, url, skip).await,
    }
}
