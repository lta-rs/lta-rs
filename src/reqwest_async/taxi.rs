use crate::{reqwest_async::ReqwestAsync, LTAClient, Taxi, TaxiRequests};

impl TaxiRequests<LTAClient<ReqwestAsync>> for Taxi {}
