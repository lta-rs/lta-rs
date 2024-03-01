use async_trait::async_trait;

use crate::{reqwest_async::ReqwestAsync, LTAClient, Train, TrainRequests};

#[async_trait]
impl TrainRequests<LTAClient<ReqwestAsync>> for Train {}
