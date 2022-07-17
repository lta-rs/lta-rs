use crate::api_url;
use crate::models::train::prelude::*;
use crate::{Client, LTAResult};

use super::ClientExt;

pub trait TrainRequests<C: Client + ClientExt> {
    /// Returns detailed information on train service unavailability during scheduled
    /// operating hours, such as affected line and stations etc.
    ///
    /// **Update freq**: ad-hoc
    fn get_train_service_alert(
        client: &C,
        skip: impl Into<Option<u32>>,
    ) -> LTAResult<TrainServiceAlert> {
        client.build_req_with_skip::<TrainServiceAlertResp, _>(
            api_url!("/TrainServiceAlerts"),
            skip.into(),
        )
    }
}
