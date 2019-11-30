use crate::async_utils::build_req_async;
use crate::lta_client::LTAClient;
use lta_models::train::train_service_alert::{TrainServiceAlert, TrainServiceAlertResp, URL};
use lta_utils_commons::LTAResult;

/// Returns detailed information on train service unavailability during scheduled
/// operating hours, such as affected line and stations etc.
///
/// **Update freq**: ad-hoc
pub async fn get_train_service_alert(client: &LTAClient) -> LTAResult<TrainServiceAlert> {
    build_req_async::<TrainServiceAlertResp, _>(client, URL).await
}
