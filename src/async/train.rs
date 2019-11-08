//! All API pertaining to train related data
use crate::Error as LTAError;
use futures::Future;

use crate::r#async::lta_client::LTAClient;
use crate::train::train_service_alert::{TrainServiceAlert, TrainServiceAlertResp, URL};
use crate::utils::commons::build_req_async;

/// Returns detailed information on train service unavailability during scheduled
/// operating hours, such as affected line and stations etc.
///
/// **Update freq**: ad-hoc
pub fn get_train_service_alert(
    client: &LTAClient,
) -> impl Future<Item = TrainServiceAlert, Error = LTAError> {
    build_req_async::<TrainServiceAlertResp, _>(client, URL)
}
