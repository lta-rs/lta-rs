//! All API pertaining to taxis
use crate::lta_client::LTAClient;
use crate::utils::commons::{build_req, Result};

pub mod taxi_avail {
    use serde::{Deserialize, Serialize};

    pub const URL: &str = "http://datamall2.mytransport.sg/ltaodataservice/Taxi-Availability";

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct Coordinates {
        #[serde(rename = "Longitude")]
        pub long: f64,

        #[serde(rename = "Latitude")]
        pub lat: f64,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct TaxiAvailResp {
        pub value: Vec<Coordinates>,
    }

    impl Into<Vec<Coordinates>> for TaxiAvailResp {
        fn into(self) -> Vec<Coordinates> {
            self.value
        }
    }
}

/// Returns location coordinates of all Taxis that are currently available for
/// hire. Does not include "Hired" or "Busy" Taxis.
///
/// **Update freq**: 1min
/// ## Example
/// ```rust
/// use lta::prelude::*;
/// use lta::Result;
/// use lta::taxi::get_taxi_avail;
/// use lta::lta_client::LTAClient;
///
/// fn main() -> Result<()> {
///     let api_key = std::env::var("API_KEY").unwrap();
///     let client = LTAClient::with_api_key(api_key);
///     let taxi_avail: Vec<TaxiCoordinates> = get_taxi_avail(&client)?;
///     println!("{:?}", taxi_avail);
///     Ok(())
/// }
/// ```
pub fn get_taxi_avail(client: &LTAClient) -> Result<Vec<taxi_avail::Coordinates>> {
    build_req::<taxi_avail::TaxiAvailResp>(client, taxi_avail::URL).map(|f| f.value)
}
