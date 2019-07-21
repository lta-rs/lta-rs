//! # lta
//! lta-rs is a lta datamall client library written in pure safe rust. lta-rs is used to interact with the lta-datamall
//!
//! ## Design Decisions
//! - Made sure that Rust structs are as close to the original response as possible to make sure that people can reference the original docs if there are any issues
//! - Simple and no additional baggage. Only the client is included. E.g If anyone wants to add concurrency, they have to do it their own
//!
//! ## Usage
//! Put this in you `Cargo.toml`
//! ```toml
//! [dependencies]
//! lta = "0.2.0"
//! ```
//! Initialise API key
//! ```rust
//! extern crate lta;
//! use lta::lta_client::LTAClient;
//! fn main() {
//!     let client = LTAClient::new().with_api_key("Your API KEY".to_string());
//! }
//! ```

extern crate chrono;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate reqwest;
extern crate serde;

pub mod bus;
pub mod bus_enums;
pub mod crowd;
pub mod lta_client;
pub mod taxi;
pub mod traffic;
pub mod train;
pub mod utils;

#[cfg(test)]
mod tests {
    use std::env;
    use std::fmt::Debug;

    use crate::crowd::passenger_vol::VolType;
    use crate::lta_client::*;
    use crate::{bus, crowd, taxi, traffic, train};

    fn run_test_and_print<F, T>(f: F)
    where
        F: Fn(&LTAClient) -> reqwest::Result<T>,
        T: Debug,
    {
        let api_key = env::var("API_KEY").unwrap();
        let client = LTAClient::new().with_api_key(api_key);
        let res = f(&client);
        match res {
            Ok(r) => println!("{:?}", r),
            Err(e) => println!("{:?}", e),
        }
    }

    #[test]
    fn get_arrivals() {
        run_test_and_print(|c| bus::get_arrival(c, 83139, "15"))
    }

    #[test]
    fn get_bus_services() {
        run_test_and_print(|c| bus::get_bus_services(c));
    }

    #[test]
    fn get_bus_routes() {
        run_test_and_print(|c| bus::get_bus_routes(c));
    }

    #[test]
    fn get_bus_stops() {
        run_test_and_print(|c| bus::get_bus_stops(c));
    }

    #[test]
    fn get_passenger_vol() {
        run_test_and_print(|c| crowd::get_passenger_vol_by(c, VolType::OdTrain));
    }

    #[test]
    fn get_taxi_avail() {
        run_test_and_print(|c| taxi::get_taxi_avail(c));
    }

    #[test]
    fn get_erp_rates() {
        run_test_and_print(|c| traffic::get_erp_rates(c));
    }

    #[test]
    fn get_cp_avail() {
        run_test_and_print(|c| traffic::get_carpark_avail(c));
    }

    #[test]
    fn get_est_travel_time() {
        run_test_and_print(|c| traffic::get_est_travel_time(c));
    }

    #[test]
    fn get_faulty_traffic_lights() {
        run_test_and_print(|c| traffic::get_faulty_traffic_lights(c));
    }

    #[test]
    fn get_road_details() {
        use traffic::road::RoadDetailsType::RoadWorks;
        run_test_and_print(|c| traffic::get_road_details(c, RoadWorks));
    }

    #[test]
    fn get_traffic_images() {
        run_test_and_print(|c| traffic::get_traffic_images(c));
    }

    #[test]
    fn get_traffic_incidents() {
        run_test_and_print(|c| traffic::get_traffic_incidents(c));
    }

    #[test]
    fn get_traffic_speed_band() {
        run_test_and_print(|c| traffic::get_traffic_speed_band(c));
    }

    #[test]
    fn get_vms() {
        run_test_and_print(|c| traffic::get_vms_emas(c));
    }

    #[test]
    fn get_bike_parking() {
        run_test_and_print(|c| traffic::get_bike_parking(c, 1.364897, 103.766094, 0.5));
    }

    #[test]
    fn get_train_service_alerts() {
        run_test_and_print(|c| train::get_train_service_alert(c));
    }
}
