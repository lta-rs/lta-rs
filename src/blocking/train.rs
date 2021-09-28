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
    fn get_train_service_alert(client: &C, skip: Option<u32>) -> LTAResult<TrainServiceAlert>;
}

impl TrainRequests<LTAClient> for Train {
    fn get_train_service_alert(
        client: &LTAClient,
        skip: Option<u32>,
    ) -> LTAResult<TrainServiceAlert> {
        let res = build_req_with_skip::<TrainServiceAlertResp, _, _>(
            client,
            api_url!("/TrainServiceAlerts"),
            skip,
        );

        return if let Err(e) = res {
            match e {
                LTAError::UnhandledStatusCode(StatusCode::INTERNAL_SERVER_ERROR, body) => {
                    if body.contains("Rate limit") {
                        Err(LTAError::RateLimitReached)
                    } else {
                        Err(LTAError::UnhandledStatusCode(
                            StatusCode::INTERNAL_SERVER_ERROR,
                            body,
                        ))
                    }
                }
                _ => Err(e),
            }
        } else {
            res
        };
    }
}
