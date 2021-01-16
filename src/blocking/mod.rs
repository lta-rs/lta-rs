pub mod bus;
pub mod client;
pub mod crowd;
pub mod taxi;
pub mod traffic;
pub mod train;

use crate::{LTAError, LTAResult};

pub use client::LTAClient;
use reqwest::blocking;

pub mod prelude {
    pub use crate::blocking::{
        bus::BusRequests,
        crowd::CrowdRequests,
        taxi::TaxiRequests,
        traffic::TrafficRequests,
        train::TrainRequests,
    };
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Bus;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Crowd;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Taxi;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Traffic;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Train;

/// A `Client` to make requests with
/// The `Client` holds a connection pool internally, so it is advised that you create one and reuse it
pub trait Client: Sized {
    /// Any backend Client
    type InternalClient;

    /// Any type that can build requests
    type RB;

    /// General constructor for `Self`
    fn new<S: Into<String>>(api_key: S, client: Self::InternalClient) -> Self;

    /// This method not assign the `api_key` in struct if the provided key is empty or whitespaces
    /// Instead, assign `None`
    fn with_api_key<S: Into<String>>(api_key: S) -> LTAResult<Self>;

    /// Returns `Self::RB`
    fn req_builder(&self, url: &str) -> Self::RB;
}

pub(crate) fn build_req_with_skip<T, M, C>(client: &C, url: &str, skip: Option<u32>) -> LTAResult<M>
where
    C: Client<RB = blocking::RequestBuilder>,
    for<'de> T: serde::Deserialize<'de> + Into<M>,
{
    let skip = skip.unwrap_or(0);
    let rb = client.req_builder(url).query(&[("$skip", skip)]);
    rb.send()
        .map_err(LTAError::BackendError)?
        .json()
        .map(|f: T| f.into())
        .map_err(LTAError::BackendError)
}

pub(crate) fn build_req_with_query<T, M, F, C>(client: &C, url: &str, query: F) -> LTAResult<M>
where
    F: FnOnce(blocking::RequestBuilder) -> blocking::RequestBuilder,
    C: Client<RB = blocking::RequestBuilder>,
    for<'de> T: serde::Deserialize<'de> + Into<M>,
{
    let rb = client.req_builder(url);
    query(rb)
        .send()
        .map_err(LTAError::BackendError)?
        .json()
        .map(|f: T| f.into())
        .map_err(LTAError::BackendError)
}

#[cfg(test)]
mod tests {
    use lta_models::prelude::*;
    use std::env;
    use crate::blocking::*;
    use crate::LTAResult;
    use crate::blocking::prelude::*;

    macro_rules! gen_test {
        ($f: expr) => {{
            let client = get_client();
            let data = $f(&client, None);
            println!("{:?}", data);
            Ok(())
        }};
    }

    fn get_client() -> LTAClient {
        let api_key = env::var("API_KEY").expect("API_KEY does not exist!");
        let client = LTAClient::with_api_key(api_key).unwrap();
        client
    }

    #[test]
    fn get_bus_arrivals() -> LTAResult<()> {
        let client = get_client();
        let data = Bus::get_arrival(&client, 83139, None)?;
        println!("{:?}", data);
        Ok(())
    }

    #[test]
    fn get_bus_services() -> LTAResult<()> {
        gen_test!(Bus::get_bus_services)
    }

    #[test]
    fn get_bus_routes() -> LTAResult<()> {
        gen_test!(Bus::get_bus_routes)
    }

    #[test]
    fn get_bus_stops() -> LTAResult<()> {
        gen_test!(Bus::get_bus_stops)
    }

    #[test]
    fn get_passenger_vol() -> LTAResult<()> {
        let client = get_client();
        let data = Crowd::get_passenger_vol_by(&client, VolType::OdBusStop, None, None)?;
        println!("{:?}", data);
        Ok(())
    }

    #[test]
    fn get_taxi_avail() -> LTAResult<()> {
        gen_test!(Taxi::get_taxi_avail)
    }

    #[test]
    fn get_taxi_stands() -> LTAResult<()> {
        gen_test!(Taxi::get_taxi_stands)
    }

    #[test]
    fn get_erp_rates() -> LTAResult<()> {
        gen_test!(Traffic::get_erp_rates)
    }

    #[test]
    fn get_cp_avail() -> LTAResult<()> {
        gen_test!(Traffic::get_carpark_avail)
    }

    #[test]
    fn get_est_travel_time() -> LTAResult<()> {
        gen_test!(Traffic::get_est_travel_time)
    }

    #[test]
    fn get_faulty_traffic_lights() -> LTAResult<()> {
        gen_test!(Traffic::get_faulty_traffic_lights)
    }

    #[test]
    fn get_road_details() -> LTAResult<()> {
        let client = get_client();
        let data = Traffic::get_road_details(&client, RoadDetailsType::RoadWorks, None)?;
        println!("{:?}", data);
        Ok(())
    }

    #[test]
    fn get_traffic_images() -> LTAResult<()> {
        gen_test!(Traffic::get_traffic_images)
    }

    #[test]
    fn get_traffic_incidents() -> LTAResult<()> {
        gen_test!(Traffic::get_traffic_incidents)
    }

    #[test]
    fn get_traffic_speed_band() -> LTAResult<()> {
        gen_test!(Traffic::get_traffic_speed_band)
    }

    #[test]
    fn get_vms() -> LTAResult<()> {
        gen_test!(Traffic::get_vms_emas)
    }

    #[test]
    fn get_bike_parking() -> LTAResult<()> {
        let client = get_client();
        let data = Traffic::get_bike_parking(&client, 1.364897, 103.766094, Some(15.0))?;
        println!("{:?}", data);
        Ok(())
    }

    #[test]
    fn get_train_service_alerts() -> LTAResult<()> {
        gen_test!(Train::get_train_service_alert)
    }
}

