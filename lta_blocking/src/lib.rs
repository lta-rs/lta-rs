//! Blocking API calls for lta-rs

use crate::lta_client::LTAClient;
use lta_utils_commons::{reqwest::blocking, serde, Client, LTAResult};

pub mod bus;
pub mod crowd;
pub mod lta_client;
pub mod taxi;
pub mod traffic;
pub mod train;

pub(crate) fn build_req_with_skip<T, M>(
    client: &LTAClient,
    url: &str,
    skip: Option<u32>,
) -> LTAResult<M>
where
    for<'de> T: serde::Deserialize<'de> + Into<M>,
{
    let skip = skip.unwrap_or(0);
    let rb = client.get_req_builder(url).query(&[("$skip", skip)]);
    rb.send()?.json().map(|f: T| f.into())
}

pub(crate) fn build_req_with_query<T, M, F>(client: &LTAClient, url: &str, query: F) -> LTAResult<M>
where
    F: FnOnce(blocking::RequestBuilder) -> blocking::RequestBuilder,
    for<'de> T: serde::Deserialize<'de> + Into<M>,
{
    let rb = client.get_req_builder(url);
    query(rb).send()?.json().map(|f: T| f.into())
}

#[cfg(test)]
mod tests {
    use crate::{bus, crowd, lta_client::LTAClient, taxi, traffic, train};
    use lta_models::prelude::*;
    use lta_utils_commons::{Client, LTAResult};
    use std::env;

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
        let client = LTAClient::with_api_key(api_key);
        client
    }

    #[test]
    fn get_bus_arrivals() -> LTAResult<()> {
        let client = get_client();
        let data = bus::get_arrival(&client, 83139, None)?;
        println!("{:?}", data);
        Ok(())
    }

    #[test]
    fn get_bus_services() -> LTAResult<()> {
        gen_test!(bus::get_bus_services)
    }

    #[test]
    fn get_bus_routes() -> LTAResult<()> {
        gen_test!(bus::get_bus_routes)
    }

    #[test]
    fn get_bus_stops() -> LTAResult<()> {
        gen_test!(bus::get_bus_stops)
    }

    #[test]
    fn get_passenger_vol() -> LTAResult<()> {
        let client = get_client();
        let data = crowd::get_passenger_vol_by(&client, VolType::OdBusStop, None, None)?;
        println!("{:?}", data);
        Ok(())
    }

    #[test]
    fn get_taxi_avail() -> LTAResult<()> {
        gen_test!(taxi::get_taxi_avail)
    }

    #[test]
    fn get_taxi_stands() -> LTAResult<()> {
        gen_test!(taxi::get_taxi_stands)
    }

    #[test]
    fn get_erp_rates() -> LTAResult<()> {
        gen_test!(traffic::get_erp_rates)
    }

    #[test]
    fn get_cp_avail() -> LTAResult<()> {
        gen_test!(traffic::get_carpark_avail)
    }

    #[test]
    fn get_est_travel_time() -> LTAResult<()> {
        gen_test!(traffic::get_est_travel_time)
    }

    #[test]
    fn get_faulty_traffic_lights() -> LTAResult<()> {
        gen_test!(traffic::get_faulty_traffic_lights)
    }

    #[test]
    fn get_road_details() -> LTAResult<()> {
        let client = get_client();
        let data = traffic::get_road_details(&client, RoadDetailsType::RoadWorks, None)?;
        println!("{:?}", data);
        Ok(())
    }

    #[test]
    fn get_traffic_images() -> LTAResult<()> {
        gen_test!(traffic::get_traffic_images)
    }

    #[test]
    fn get_traffic_incidents() -> LTAResult<()> {
        gen_test!(traffic::get_traffic_incidents)
    }

    #[test]
    fn get_traffic_speed_band() -> LTAResult<()> {
        gen_test!(traffic::get_traffic_speed_band)
    }

    #[test]
    fn get_vms() -> LTAResult<()> {
        gen_test!(traffic::get_vms_emas)
    }

    #[test]
    fn get_bike_parking() -> LTAResult<()> {
        let client = get_client();
        let data = traffic::get_bike_parking(&client, 1.364897, 103.766094, Some(15.0))?;
        println!("{:?}", data);
        Ok(())
    }

    #[test]
    fn get_train_service_alerts() -> LTAResult<()> {
        gen_test!(train::get_train_service_alert)
    }
}
