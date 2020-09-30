//! <p align="center">
//!     <img width="333" height="117" src="https://raw.githubusercontent.com/BudiNverse/lta-rs/master/logo.png">
//! </p>
//! <p align="center">
//!     <a href="https://github.com/BudiNverse/lta-rs">
//!         <img src="https://img.shields.io/badge/-lta--rs-blueviolet.svg"/>
//!     </a>
//!     <a href="https://github.com/BudiNverse/lta-rs">
//!         <img src="https://img.shields.io/github/license/BudiNverse/lta-rs"/>
//!     </a>
//!     <a href="https://crates.io/crates/lta">
//!         <img src="https://img.shields.io/crates/v/lta"/>
//!     </a>
//!     <a href="https://dev.azure.com/budisyahiddin/lta-rs/_build?definitionId=6">
//!         <img src="https://dev.azure.com/budisyahiddin/lta-rs/_apis/build/status/BudiNverse.lta-rs?branchName=master&jobName=Job&configuration=Job%20stable"/>
//!     </a>
//!     <a href="https://github.com/BudiNverse/lta-rs">
//!         <img src="https://img.shields.io/badge/rust-1.3.9-blueviolet.svg"/>
//!     </a>
//!     <a href="https://github.com/BudiNverse/lta-rs">
//!         <img src="https://img.shields.io/crates/d/lta"/>
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
//! Features available: `async`, `blocking`. If you only need blocking requests, choose blocking vice versa.
//! ```toml
//! [dependencies]
//! lta = { version = "0.4.0", features = ["async"] }
//! ```
//!
//! Initialise API key
//! ```rust
//! use lta::{
//!     utils::{Client, LTAResult},
//!     models::traffic::erp_rates::ErpRate,
//!     blocking::{
//!         traffic::get_erp_rates,
//!         lta_client::LTAClient
//!     }
//! };
//!
//! fn main() -> LTAResult<()> {
//!     let api_key = std::env::var("API_KEY").unwrap();
//!     let client = LTAClient::with_api_key(api_key);
//!     let erp_rates: Vec<ErpRate> = get_erp_rates(&client, None)?;
//!     println!("{:?}", erp_rates);
//!     Ok(())
//! }
//! ```

#[cfg(feature = "blocking")]
pub use lta_blocking as blocking;

#[cfg(feature = "async")]
pub use lta_async as r#async;

pub use lta_models as models;
pub use lta_utils_commons as utils;
pub use utils::chrono;
pub use utils::reqwest;

/// Necessary imports to use lts-rs.
pub mod prelude {
    pub use crate::utils::{Client, LTAResult};
}

#[cfg(test)]
mod tests {
    use crate::blocking::lta_client::LTAClient;
    use crate::utils::Client;
    use std::env;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    #[ignore]
    #[rustfmt::skip]
    fn dump_json() -> Result<(), Box<dyn std::error::Error>>{
        use crate::models;

        let api_key = env::var("API_KEY").expect("`API_KEY` not present as env var!");
        let client = LTAClient::with_api_key(api_key);
        let urls_with_query = [
            (lta_models::bus::bus_arrival::URL, &[("BusStopCode", "83139"), ("", ""), ("", "")], "bus_arrival.json"),
            (lta_models::traffic::bike_parking::URL, &[("Lat", "1.364897"), ("Long", "103.766094"), ("Dist", "15.0")], "bike_parking.json"),
        ];

        let urls = [
            (models::bus::bus_routes::URL, "bus_route.json"),
            (models::bus::bus_services::URL, "bus_services.json"),
            (models::bus::bus_stops::URL, "bus_stops.json"),
            (models::taxi::taxi_avail::URL, "taxi_avail.json"),
            (models::traffic::carpark_avail::URL, "carpark_avail.json"),
            (models::traffic::erp_rates::URL, "erp_rates.json"),
            (models::traffic::est_travel_time::URL, "est_travel_time.json"),
            (models::traffic::faulty_traffic_lights::URL, "faulty_traffic_lights.json"),
            (models::train::train_service_alert::URL, "train_service_alert.json"),
            (models::crowd::passenger_vol::URL_BY_BUS_STOPS, "passenger_vol_bus_stops.json"),
            (models::crowd::passenger_vol::URL_BY_OD_BUS_STOPS, "passenger_vol_od_bus_stops.json"),
            (models::crowd::passenger_vol::URL_BY_OD_TRAIN, "passenger_vol_od_train.json"),
            (models::crowd::passenger_vol::URL_BY_TRAIN, "passenger_vol_train.json"),
            (models::taxi::taxi_stands::URL, "taxi_stands.json")
        ];
        let mut results: Vec<(String, &str)> = Vec::with_capacity(15);

        for url in urls.iter() {
            let rb = client.get_req_builder(url.0);
            let data = rb
                .send()
                .map(|res| res.text().unwrap())?;

            println!("{}", &data);
            results.push((data, url.1))
        }

        for url in urls_with_query.iter() {
            let rb = client.get_req_builder(url.0);
            let data = rb
                .query(url.1)
                .send()
                .map(|res| res.text().unwrap())?;

            println!("{}", &data);
            results.push((data, url.2))
        }
        results.into_iter().for_each(|f| {
            let mut file = File::create(format!("./dumped_data/{}", f.1)).unwrap();
            file.write_all(f.0.as_bytes()).unwrap();
        });

        Ok(())
    }
}
