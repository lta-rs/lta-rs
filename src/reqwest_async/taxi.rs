use async_trait::async_trait;

use crate::{Taxi, TaxiRequests, LTAClient, reqwest_async::ReqwestAsync};

#[async_trait]
impl TaxiRequests<LTAClient<ReqwestAsync>> for Taxi {}
