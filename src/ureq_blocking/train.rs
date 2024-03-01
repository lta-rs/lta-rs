use ureq::Agent;

use crate::{
    blocking::{train::TrainRequests, LTAClient},
    Train,
};

impl TrainRequests<LTAClient<Agent>> for Train {}
