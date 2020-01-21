//! All APIs pertaining to traffic

use crate::build_req_with_skip;
use crate::lta_client::LTAClient;
use lta_models::train::train_service_alert;
use lta_utils_commons::LTAResult;

/// Returns detailed information on train service unavailability during scheduled
/// operating hours, such as affected line and stations etc.
///
/// **Update freq**: ad-hoc
pub fn get_train_service_alert(
    client: &LTAClient,
    skip: Option<u32>,
) -> LTAResult<train_service_alert::TrainServiceAlert> {
    build_req_with_skip::<train_service_alert::TrainServiceAlertResp, _>(
        client,
        train_service_alert::URL,
        skip,
    )
}
