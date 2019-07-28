use reqwest::Error;
use tokio::prelude::Future;

use crate::crowd::passenger_vol::*;
use crate::crowd::*;
use crate::r#async::lta_client::LTAClient;
use crate::utils::commons::Client;

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

    let rb = client.get_req_builder(url);
    rb.send()
        .and_then(|mut f| f.json::<PassengerVolRawResp>())
        .map(|r| r.value.into_iter().map(|l| l.link).collect())
}
