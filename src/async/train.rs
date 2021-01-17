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
    async fn get_train_service_alert(client: &C, skip: Option<u32>)
        -> LTAResult<TrainServiceAlert>;
}

#[async_trait]
impl TrainRequests<LTAClient> for Train {
    async fn get_train_service_alert(
        client: &LTAClient,
        skip: Option<u32>,
    ) -> LTAResult<TrainServiceAlert> {
        build_req_with_skip::<TrainServiceAlertResp, _, _>(
            client,
            api_url!("/TrainServiceAlerts"),
            skip,
        )
        .await
    }
}
