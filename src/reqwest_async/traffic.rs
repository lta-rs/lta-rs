use crate::r#async::ClientExt;
use crate::Client;
use crate::{reqwest_async::ReqwestAsync, LTAClient, LTAResult, Traffic, TrafficRequests};
use concat_string::concat_string;
use lta_models::prelude::{BikeParking, BikeParkingResp};

impl TrafficRequests<LTAClient<ReqwestAsync>> for Traffic {
    async fn get_bike_parking<D>(
        client: &LTAClient<ReqwestAsync>,
        lat: f64,
        long: f64,
        dist: D,
    ) -> LTAResult<Vec<BikeParking>>
    where
        D: Into<Option<f64>> + Send,
    {
        let unwrapped_dist = dist.into().unwrap_or(0.5);
        let url = concat_string!(client.base_url(), "/BicycleParkingv2");

        client
            .build_req_with_query::<BikeParkingResp, _, _>(url.as_str(), |rb| {
                rb.query(&[("Lat", lat), ("Long", long), ("Dist", unwrapped_dist)])
            })
            .await
    }
}
