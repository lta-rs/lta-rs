use crate::{
    blocking::{train::TrainRequests, LTAClient},
    reqwest_blocking::ReqwestBlocking,
    Train,
};

impl TrainRequests<LTAClient<ReqwestBlocking>> for Train {}
