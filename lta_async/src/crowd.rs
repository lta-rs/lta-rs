use crate::async_utils::build_req_async;
use crate::lta_client::LTAClient;
use lta_models::crowd::passenger_vol::{
    PassengerVolRawResp, VolType, URL_BY_BUS_STOPS, URL_BY_OD_BUS_STOPS, URL_BY_OD_TRAIN,
    URL_BY_TRAIN,
};
use lta_utils_commons::LTAResult;

/// Creates a new client for every call
/// **Update freq**: By 15th of every month, the passenger volume for previous month data
/// will be generated
///
/// Note: Link will expire after 5mins!
pub async fn get_passenger_vol_by(client: &LTAClient, vol_type: VolType) -> LTAResult<Vec<String>> {
    let url = match vol_type {
        VolType::BusStops => URL_BY_BUS_STOPS,
        VolType::OdBusStop => URL_BY_OD_BUS_STOPS,
        VolType::Train => URL_BY_TRAIN,
        VolType::OdTrain => URL_BY_OD_TRAIN,
    };
    build_req_async::<PassengerVolRawResp, _>(client, url).await
}
