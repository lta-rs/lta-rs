use crate::models::train::prelude::*;
use crate::{Client, LTAResult};
use concat_string::concat_string;

use super::ClientExt;

pub trait TrainRequests<C: Client + ClientExt> {
    /// Returns detailed information on train service unavailability during scheduled
    /// operating hours, such as affected line and stations etc.
    ///
    /// **Update freq**: ad-hoc
    async fn get_train_service_alert<S>(client: &C, skip: S) -> LTAResult<TrainServiceAlert>
    where
        S: Into<Option<u32>>,
    {
        client
            .build_req_with_skip::<TrainServiceAlertResp, _>(
                &concat_string!(client.base_url(), "/TrainServiceAlerts"),
                skip.into(),
            )
            .await
    }
}
