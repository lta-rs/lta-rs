use crate::api_url;
use crate::blocking::{build_req_with_skip, LTAClient};
use crate::models::train::prelude::*;
use crate::reqwest::StatusCode;
use crate::{Client, LTAError, LTAResult, Train};

pub trait TrainRequests<C: Client> {
    /// Returns detailed information on train service unavailability during scheduled
    /// operating hours, such as affected line and stations etc.
    ///
    /// **Update freq**: ad-hoc
    fn get_train_service_alert<S>(client: &C, skip: S) -> LTAResult<TrainServiceAlert>
    where
        S: Into<Option<u32>>;
}

impl TrainRequests<LTAClient> for Train {
    fn get_train_service_alert<S>(client: &LTAClient, skip: S) -> LTAResult<TrainServiceAlert>
    where
        S: Into<Option<u32>>,
    {
        build_req_with_skip::<TrainServiceAlertResp, _, _>(
            client,
            api_url!("/TrainServiceAlerts"),
            skip.into(),
        )
    }
}
