//! All API pertaining to transportation crowd

use crate::lta_client::LTAClient;
use crate::utils::commons::{build_req, Result};

pub mod passenger_vol {
    use serde::{Deserialize, Serialize};

    pub const URL_BY_BUS_STOPS: &str = "http://datamall2.mytransport.sg/ltaodataservice/PV/Bus";

    pub const URL_BY_OD_BUS_STOPS: &str =
        "http://datamall2.mytransport.sg/ltaodataservice/PV/ODBus";

    pub const URL_BY_TRAIN: &str = "http://datamall2.mytransport.sg/ltaodataservice/PV/Train";

    pub const URL_BY_OD_TRAIN: &str = "http://datamall2.mytransport.sg/ltaodataservice/PV/ODTrain";

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub enum VolType {
        /// Returns tap in and tap out passenger volume by weekdays and
        /// weekends for individual bus stop
        BusStops,

        /// Returns number of trips by weekdays and weekends from origin to
        /// destination bus stops
        OdBusStop,

        /// Returns number of trips by weekdays and weekends from origin to
        /// destination train stations
        Train,

        /// Returns tap in and tap out passenger volume by weekdays and
        /// weekends for individual train station
        OdTrain,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct Link {
        #[serde(rename = "Link")]
        pub link: String,
    }

    impl Into<String> for Link {
        fn into(self) -> String {
            self.link
        }
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct PassengerVolRawResp {
        pub value: Vec<Link>,
    }

    impl Into<Vec<String>> for PassengerVolRawResp {
        fn into(self) -> Vec<String> {
            self.value.into_iter().map(|f| f.link).collect()
        }
    }
}

/// **Update freq**: By 15th of every month, the passenger volume for previous month data
/// will be generated
///
/// Note: Link will expire after 5mins!
/// ## Example
/// ```rust
/// use lta::prelude::*;
/// use lta::Result;
/// use lta::lta_client::LTAClient;
/// use lta::crowd::{ get_passenger_vol_by, passenger_vol::VolType };
///
/// fn main()  {
///     let api_key = std::env::var("API_KEY").unwrap();
///     let client = LTAClient::with_api_key(api_key);
///     let train_crowd: Result<Vec<String>> = get_passenger_vol_by(&client, VolType::Train);
///     match train_crowd {
///         Ok(tc) => println!("{:?}", tc),
///         Err(e) => {
///             if e.is_serialization() {
///                 println!("INTERNAL SERVER ERROR")
///             }
///         }
///     }
/// }
/// ```
pub fn get_passenger_vol_by(
    client: &LTAClient,
    vol_type: passenger_vol::VolType,
) -> Result<Vec<String>> {
    use crate::crowd::passenger_vol::VolType;

    let url = match vol_type {
        VolType::BusStops => passenger_vol::URL_BY_BUS_STOPS,
        VolType::OdBusStop => passenger_vol::URL_BY_OD_BUS_STOPS,
        VolType::Train => passenger_vol::URL_BY_TRAIN,
        VolType::OdTrain => passenger_vol::URL_BY_OD_TRAIN,
    };

    build_req::<passenger_vol::PassengerVolRawResp, _>(client, url)
}
