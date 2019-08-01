//! ![Github](https://img.shields.io/badge/-lta--rs-blueviolet.svg?style=flat-square&link=https://github.com/BudiNverse/lta-rs)
//! ![GitHub](https://img.shields.io/github/license/BudiNverse/lta-rs?style=flat-square)
//! ![GitHub](https://img.shields.io/crates/v/lta?style=flat-square&link=https://crates.io/crates/lta)
//! ![Github](https://img.shields.io/travis/com/BudiNverse/lta-rs?style=flat-square&link=https://travis-ci.com/BudiNverse/lta-rs)
//! ![Github](https://img.shields.io/badge/rust-1.3.6-blueviolet.svg?style=flat-square)
//! ![Github](https://img.shields.io/crates/d/lta?style=flat-square)
//!
//!
//! # lta
//! lta-rs is a lta datamall client library written in pure safe rust. lta-rs is used to interact with the lta-datamall
//!
//! ## Design Decisions
//! - Made sure that Rust structs are as close to the original response as possible to make sure that people can reference the original docs if there are any issues
//! - Simple and no additional baggage. Only the client is included. E.g If anyone wants to add concurrency, they have to do it on their own
//! - Predictable API usage
//!
//! ## Usage
//! Put this in you `Cargo.toml`
//! ```toml
//! [dependencies]
//! lta = "0.3.0-async-preview-1"
//! ```
//!
//! Initialise API key
//! ```rust
//! extern crate lta;
//! use lta::prelude::*;
//! use lta::traffic::get_erp_rates;
//! use lta::lta_client::LTAClient;
//!
//! fn main() -> Result<()>{
//!     let client = LTAClient::with_api_key("api_key");
//!     let erp_rates: Vec<ErpRate> = get_erp_rates(&client)?;
//!     println!("{:?}", erp_rates);
//!     Ok(())
//! }
//! ```

extern crate chrono;
extern crate futures;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate reqwest;
extern crate serde;
extern crate tokio;

pub mod r#async;
pub mod bus;
pub mod bus_enums;
pub mod crowd;
pub mod lta_client;
pub mod taxi;
pub mod traffic;
pub mod train;
pub mod utils;

pub mod prelude {
    pub use crate::bus::{
        bus_arrival::{ArrivalBusService, BusArrivalResp, NextBus},
        bus_routes::BusRoute,
        bus_services::{BusFreq, BusService},
        bus_stops::BusStop,
    };
    pub use crate::bus_enums::*;
    pub use crate::crowd::passenger_vol::VolType;
    pub use crate::taxi::taxi_avail::Coordinates as TaxiCoordinates;
    pub use crate::traffic::{
        bike_parking::BikeParking,
        carpark_avail::Carpark,
        erp_rates::ErpRate,
        est_travel_time::{EstTravelTime, Highway, HighwayDirection},
        faulty_traffic_lights::{FaultyTrafficLight, TechnicalAlarmType},
        road::{RoadDetails, RoadDetailsType},
        traffic_images::TrafficImage,
        traffic_incidents::{IncidentType, TrafficIncident},
        traffic_speed_bands::{RoadCategory, TrafficSpeedBand},
        vms_emas::VMS,
    };
    pub use crate::train::train_service_alert::TrainServiceAlert;
    pub use crate::utils::commons::{Client, Coordinates, Error, Location, Result};
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fmt::Debug;

    use tokio::prelude::Future;

    use crate::bus::bus_arrival::BusArrivalResp;
    use crate::crowd::passenger_vol::VolType;
    use crate::lta_client::*;
    use crate::r#async::lta_client::LTAClient as AsyncLTAClient;
    use crate::traffic::erp_rates::ErpRate;
    use crate::utils::commons::{Client, Result};
    use crate::{bus, crowd, taxi, traffic, train};

    fn run_test_and_print<F, T>(f: F)
    where
        F: Fn(&LTAClient) -> Result<T>,
        T: Debug,
    {
        let api_key = env::var("API_KEY").unwrap();
        let client = LTAClient::with_api_key(api_key);
        let res = f(&client);
        match res {
            Ok(r) => println!("{:?}", r),
            Err(e) => println!("{:?}", e),
        }
    }

    fn async_example(client: &AsyncLTAClient) -> impl Future<Item = (), Error = ()> {
        use crate::r#async::{bus::get_arrival, traffic::get_erp_rates};
        // these are imported because my IDE is complaining of missing stuff
        // and it wont show any autocomplete
        // as of now, until this is fixed, this import will remain here
        // just for the sake of autocomplete and red lines
        use futures::{FutureExt, TryFutureExt};

        type Req = (Vec<ErpRate>, BusArrivalResp);
        let fut = get_erp_rates(client);
        let fut2 = get_arrival(client, 83139, "15");

        fut.join(fut2)
            .map(|(a, b): Req| {
                println!("{:?}", a);
                println!("{:?}", b);
                std::process::exit(0);
            })
            .map_err(|e| println!("Request failed \n ${:?}", e))
    }

    #[test]
    fn run_async() {
        let api_key = env::var("API_KEY").unwrap();
        let client = &AsyncLTAClient::with_api_key(api_key);
        let fut = async_example(client);
        tokio::run(fut);
    }

    #[test]
    fn get_arrivals() {
        let api_key = env::var("API_KEY").unwrap();
        let client = LTAClient::with_api_key(api_key);
        let res = bus::get_arrival(&client, 83139, "15").unwrap();
        let arr = res.services.get(0);

        match arr {
            Some(r) => println!("{:?}", r.next_bus_as_arr()),
            None => println!("No arrivals"),
        }

        println!("{:?}", &res);
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
        run_test_and_print(|c| crowd::get_passenger_vol_by(c, VolType::OdBusStop));
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
