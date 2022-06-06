use crate::api_url;
use crate::models::train::prelude::*;
use crate::r#async::{build_req_with_skip, LTAClient};
use crate::{Client, LTAResult, Train};
use async_trait::async_trait;

#[async_trait]
pub trait TrainRequests<C: Client> {
    /// Returns detailed information on train service unavailability during scheduled
    /// operating hours, such as affected line and stations etc.
    ///
    /// **Update freq**: ad-hoc
    async fn get_train_service_alert<S>(client: &C, skip: S) -> LTAResult<TrainServiceAlert>
    where
        S: Into<Option<u32>> + Send;
}

#[async_trait]
impl TrainRequests<LTAClient> for Train {
    async fn get_train_service_alert<S>(client: &LTAClient, skip: S) -> LTAResult<TrainServiceAlert>
    where
        S: Into<Option<u32>> + Send,
    {
        build_req_with_skip::<TrainServiceAlertResp, _, _>(
            client,
            api_url!("/TrainServiceAlerts"),
            skip.into(),
        )
        .await
    }
}
