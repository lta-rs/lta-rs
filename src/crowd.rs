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

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct PassengerVolRawResp {
        pub value: Vec<Link>,
    }
}

/// **Update freq**: By 15th of every month, the passenger volume for previous month data
/// will be generated
///
/// Note: Link will expire after 5mins!
/// ## Example
/// ```rust
/// use lta::prelude::*;  
/// use lta::lta_client::LTAClient;
/// use lta::crowd::{ get_passenger_vol_by, passenger_vol::VolType };
///
/// fn main() -> Result<()> {
///     let client = LTAClient::with_api_key("api_key");
///     let train_crowd: Vec<String> = get_passenger_vol_by(&client, VolType::Train)?;
///     println!("{:?}", train_crowd);
///     Ok(())
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

    let resp: passenger_vol::PassengerVolRawResp = build_req(client, url)?;
    let as_str = resp.value.into_iter().map(|f| f.link).collect();

    Ok(as_str)
}
