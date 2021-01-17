use crate::api_url;
use crate::blocking::{build_req_with_skip, Client, LTAClient};
use crate::models::train::prelude::*;
use crate::{LTAResult, Train};

pub trait TrainRequests<C: Client> {
    /// Returns detailed information on train service unavailability during scheduled
    /// operating hours, such as affected line and stations etc.
    ///
    /// **Update freq**: ad-hoc
    fn get_train_service_alert(client: &C, skip: Option<u32>) -> LTAResult<TrainServiceAlert>;
}

impl TrainRequests<LTAClient> for Train {
    fn get_train_service_alert(
        client: &LTAClient,
        skip: Option<u32>,
    ) -> LTAResult<TrainServiceAlert> {
        build_req_with_skip::<TrainServiceAlertResp, _, _>(
            client,
            api_url!("/TrainServiceAlerts"),
            skip,
        )
    }
}
