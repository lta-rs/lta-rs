use crate::Client;
use lta_models::prelude::{BikeParking, BikeParkingResp};

use crate::{
    blocking::{traffic::TrafficRequests, ClientExt, LTAClient},
    reqwest_blocking::ReqwestBlocking,
    LTAResult, Traffic,
};
use concat_string::concat_string;

impl TrafficRequests<LTAClient<ReqwestBlocking>> for Traffic {
    fn get_bike_parking(
        client: &LTAClient<ReqwestBlocking>,
        lat: f64,
        long: f64,
        dist: impl Into<Option<f64>>,
    ) -> LTAResult<Vec<BikeParking>> {
        let unwrapped_dist = dist.into().unwrap_or(0.5);
        client.build_req_with_query::<BikeParkingResp, _, _>(
            &concat_string!(client.base_url(), "/BicycleParkingv2"),
            |rb| rb.query(&[("Lat", lat), ("Long", long), ("Dist", unwrapped_dist)]),
        )
    }
}
