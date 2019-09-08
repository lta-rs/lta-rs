//! <p align="center">
//!     <img width="333" height="117" src="https://raw.githubusercontent.com/BudiNverse/lta-rs/master/logo.png">
//! </p>
//! <p align="center">
//!     <a href="https://github.com/BudiNverse/lta-rs">
//!         <img src="https://img.shields.io/badge/-lta--rs-blueviolet.svg?style=flat-square"/>
//!     </a>
//!     <a href="https://github.com/BudiNverse/lta-rs">
//!         <img src="https://img.shields.io/github/license/BudiNverse/lta-rs?style=flat-square"/>
//!     </a>
//!     <a href="https://crates.io/crates/lta">
//!         <img src="https://img.shields.io/crates/v/lta?style=flat-square"/>
//!     </a>
//!     <a href="https://travis-ci.com/BudiNverse/lta-rs">
//!         <img src="https://img.shields.io/travis/com/BudiNverse/lta-rs?style=flat-square"/>
//!     </a>
//!     <a href="https://github.com/BudiNverse/lta-rs">
//!         <img src="https://img.shields.io/badge/rust-1.3.6-blueviolet.svg?style=flat-square"/>
//!     </a>
//!     <a href="https://github.com/BudiNverse/lta-rs">
//!         <img src="https://img.shields.io/crates/d/lta?style=flat-square"/>
//!     </a>
//! </p>
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
//! lta = "0.3.0-async-preview-4"
//! ```
//!
//! Initialise API key
//! ```rust
//! extern crate lta;
//! use lta::prelude::*;
//! use lta::traffic::get_erp_rates;
//! use lta::lta_client::LTAClient;
//! use lta::Result;
//!
//! fn main() -> Result<()>{
//!     let api_key = std::env::var("API_KEY").unwrap();
//!     let client = LTAClient::with_api_key(api_key);
//!     let erp_rates: Vec<ErpRate> = get_erp_rates(&client)?;
//!     println!("{:?}", erp_rates);
//!     Ok(())
//! }
//! ```

pub extern crate chrono;
extern crate futures;
#[macro_use]
extern crate lazy_static;
extern crate regex;
pub extern crate reqwest;
extern crate serde;

pub use crate::utils::commons::Error;
pub use crate::utils::commons::Result;

pub mod r#async;
pub mod bus;
pub mod bus_enums;
pub mod crowd;
pub mod lta_client;
pub mod taxi;
pub mod traffic;
pub mod train;
pub mod utils;

/// Necessary imports to use lts-rs. Prefer this over glob imports
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
    pub use crate::utils::commons::{Client, Coordinates, Location};
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fmt::Debug;

    use crate::traffic::get_carpark_avail;
    use crate::Result;
    use crate::{
        bus, crowd,
        lta_client::LTAClient,
        r#async::{lta_client::LTAClient as AsyncLTAClient, prelude::*},
        taxi, traffic, train,
    };

    fn run_test_and_print<F, T>(f: F)
    where
        F: Fn(&LTAClient) -> Result<T>,
        T: Debug,
    {
        let api_key = env::var("API_KEY").unwrap();
        let client = LTAClient::with_api_key(api_key);
        let res = f(&client).unwrap();
        println!("{:?}", res);
    }

    fn async_example(client: &AsyncLTAClient) -> impl Future<Item = (), Error = ()> {
        use crate::r#async::{bus::get_arrival, traffic::get_faulty_traffic_lights};

        type Req = (Vec<FaultyTrafficLight>, BusArrivalResp);
        let fut = get_faulty_traffic_lights(client);
        let fut2 = get_arrival(client, 83139, Some("15"));

        fut.join(fut2)
            .map(|(a, b): Req| {
                println!("{:?}", a);
                println!("{:?}", b);
                std::process::exit(0);
            })
            .map_err(|e| panic!("Request failed \n ${:?}", e))
    }

    #[test]
    fn run_async() {
        extern crate tokio;

        let api_key = env::var("API_KEY").unwrap();
        let client = &AsyncLTAClient::with_api_key(api_key);
        let fut = async_example(client);
        tokio::run(fut);
    }

    #[test]
    fn concurrent() {
        use std::sync::Arc;
        use std::thread::spawn;

        let api_key = env::var("API_KEY").unwrap();
        let c1 = Arc::new(LTAClient::with_api_key(api_key));
        let c2 = c1.clone();

        let child = spawn(move || {
            let res = get_carpark_avail(&c1).unwrap();
            println!("{:?}", res)
        });

        let vms = traffic::get_vms_emas(&c2).unwrap();
        println!("{:?}", vms);

        child.join().unwrap();
    }

    #[test]
    fn get_arrival_empty_next_bus() {
        let json = r#"{
    "odata.metadata": "http://datamall2.mytransport.sg/ltaodataservice/$metadata#BusArrivalv2/@Element",
        "BusStopCode": "83139",
        "Services": [
            {
                "ServiceNo": "15",
                "Operator": "GAS",
                "NextBus": {
                "OriginCode": "77009",
                "DestinationCode": "77009",
                "EstimatedArrival": "2019-08-24T23:12:44+08:00",
                "Latitude": "1.3155073333333334",
                "Longitude": "103.90588666666666",
                "VisitNumber": "1",
                "Load": "SEA",
                "Feature": "WAB",
                "Type": "SD"
            },
                "NextBus2": {
                "OriginCode": "77009",
                "DestinationCode": "77009",
                "EstimatedArrival": "2019-08-24T23:32:28+08:00",
                "Latitude": "1.3451273333333333",
                "Longitude": "103.9366695",
                "VisitNumber": "1",
                "Load": "SEA",
                "Feature": "WAB",
                "Type": "SD"
            },
                "NextBus3": {
                "OriginCode": "77009",
                "DestinationCode": "77009",
                "EstimatedArrival": "2019-08-24T23:45:49+08:00",
                "Latitude": "1.3593521666666666",
                "Longitude": "103.94218583333333",
                "VisitNumber": "1",
                "Load": "SEA",
                "Feature": "WAB",
                "Type": "SD"
            }
            },
            {
                "ServiceNo": "150",
                "Operator": "SBST",
                "NextBus": {
                "OriginCode": "82009",
                "DestinationCode": "82009",
                "EstimatedArrival": "2019-08-24T23:25:13+08:00",
                "Latitude": "0",
                "Longitude": "0",
                "VisitNumber": "1",
                "Load": "SEA",
                "Feature": "WAB",
                "Type": "SD"
            },
                "NextBus2": {
                "OriginCode": "82009",
                "DestinationCode": "82009",
                "EstimatedArrival": "2019-08-24T23:40:13+08:00",
                "Latitude": "0",
                "Longitude": "0",
                "VisitNumber": "1",
                "Load": "SEA",
                "Feature": "WAB",
                "Type": "SD"
            },
                "NextBus3": {
                "OriginCode": "",
                "DestinationCode": "",
                "EstimatedArrival": "",
                "Latitude": "",
                "Longitude": "",
                "VisitNumber": "",
                "Load": "",
                "Feature": "",
                "Type": ""
            }
            },
            {
                "ServiceNo": "155",
                "Operator": "SBST",
                "NextBus": {
                "OriginCode": "52009",
                "DestinationCode": "84009",
                "EstimatedArrival": "2019-08-24T23:11:35+08:00",
                "Latitude": "1.3155108333333334",
                "Longitude": "103.9059175",
                "VisitNumber": "1",
                "Load": "SEA",
                "Feature": "WAB",
                "Type": "SD"
            },
                "NextBus2": {
                "OriginCode": "52009",
                "DestinationCode": "84009",
                "EstimatedArrival": "2019-08-24T23:28:17+08:00",
                "Latitude": "1.326041",
                "Longitude": "103.883962",
                "VisitNumber": "1",
                "Load": "SEA",
                "Feature": "WAB",
                "Type": "SD"
            },
                "NextBus3": {
                "OriginCode": "52009",
                "DestinationCode": "84009",
                "EstimatedArrival": "2019-08-24T23:39:42+08:00",
                "Latitude": "1.3378748333333332",
                "Longitude": "103.87604566666667",
                "VisitNumber": "1",
                "Load": "SEA",
                "Feature": "WAB",
                "Type": "SD"
            }
            }
        ]
    }"#;

        println!(
            "{:?}",
            serde_json::from_str::<BusArrivalResp>(json).unwrap()
        );
    }

    #[test]
    fn get_arrivals() {
        run_test_and_print(|c| bus::get_arrival(c, 83139, None))
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
        use traffic::road::RoadDetailsType;
        run_test_and_print(|c| traffic::get_road_details(c, RoadDetailsType::RoadWorks));
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
        run_test_and_print(|c| traffic::get_bike_parking(c, 1.364897, 103.766094, None));
    }

    #[test]
    fn get_train_service_alerts() {
        run_test_and_print(|c| train::get_train_service_alert(c));
    }
}
