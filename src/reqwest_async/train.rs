use crate::{reqwest_async::ReqwestAsync, LTAClient, Train, TrainRequests};

impl TrainRequests<LTAClient<ReqwestAsync>> for Train {}
