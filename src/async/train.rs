use reqwest::Error;
use tokio::prelude::Future;

use crate::r#async::lta_client::LTAClient;
use crate::train::*;
use crate::utils::commons::Client;

pub fn get_train_service_alert(
    client: &LTAClient,
) -> impl Future<Item = Vec<train_service_alert::TrainServiceAlertResp>, Error = Error> {
    let rb = client.get_req_builder(train_service_alert::URL);
    rb.send()
        .and_then(|mut f| f.json::<train_service_alert::TrainServiceAlert>())
        .map(|r| r.value)
}
