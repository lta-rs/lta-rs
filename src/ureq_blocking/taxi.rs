use ureq::Agent;

use crate::{
    blocking::{taxi::TaxiRequests, LTAClient},
    Taxi,
};

impl TaxiRequests<LTAClient<Agent>> for Taxi {}
