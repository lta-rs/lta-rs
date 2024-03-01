use crate::api_url;
use crate::models::train::prelude::*;
use crate::{Client, LTAResult};
use async_trait::async_trait;

use super::ClientExt;

#[async_trait]
pub trait TrainRequests<C: Client + ClientExt + Send + Sync> {
    /// Returns detailed information on train service unavailability during scheduled
    /// operating hours, such as affected line and stations etc.
    ///
    /// **Update freq**: ad-hoc
    async fn get_train_service_alert<S>(client: &C, skip: S) -> LTAResult<TrainServiceAlert>
    where
        S: Into<Option<u32>> + Send,
    {
        client
            .build_req_with_skip::<TrainServiceAlertResp, _>(
                api_url!("/TrainServiceAlerts"),
                skip.into(),
            )
            .await
    }
}
