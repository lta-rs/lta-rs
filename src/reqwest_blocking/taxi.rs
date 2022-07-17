use crate::{
    blocking::{taxi::TaxiRequests, LTAClient},
    reqwest_blocking::ReqwestBlocking,
    Taxi,
};

impl TaxiRequests<LTAClient<ReqwestBlocking>> for Taxi {}
