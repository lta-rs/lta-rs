use concat_string::concat_string;
use lta_models::prelude::{BikeParking, BikeParkingResp};
use ureq::Agent;

use crate::{
    blocking::{traffic::TrafficRequests, ClientExt, LTAClient},
    Client, LTAResult, Traffic,
};

impl TrafficRequests<LTAClient<Agent>> for Traffic {
    fn get_bike_parking(
        client: &LTAClient<Agent>,
        lat: f64,
        long: f64,
        dist: impl Into<Option<f64>>,
    ) -> LTAResult<Vec<BikeParking>> {
        let unwrapped_dist = dist.into().unwrap_or(0.5);
        client.build_req_with_query::<BikeParkingResp, _, _>(
            &concat_string!(client.base_url(), "/BicycleParkingv2"),
            |rb| {
                rb.query("Lat", lat.to_string().as_str())
                    .query("Long", long.to_string().as_str())
                    .query("Dist", unwrapped_dist.to_string().as_ref())
            },
        )
    }
}
