use crate::client_config::CLIENT_CONFIG;
use crate::utils::commons::build_res;

pub mod passenger_vol {
    use serde::Deserialize;

    pub const URL_BY_BUS_STOPS: &'static str =
        "http://datamall2.mytransport.sg/ltaodataservice/PV/Bus";

    pub const URL_BY_OD_BUS_STOPS: &'static str =
        "http://datamall2.mytransport.sg/ltaodataservice/PV/ODBus";

    pub const URL_BY_TRAIN: &'static str =
        "http://datamall2.mytransport.sg/ltaodataservice/PV/Train";

    pub const URL_BY_OD_TRAIN: &'static str =
        "http://datamall2.mytransport.sg/ltaodataservice/PV/ODTrain";

    #[derive(Debug, Clone, PartialEq)]
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

    #[derive(Debug, Clone, PartialEq, Deserialize)]
    pub struct Link {
        #[serde(rename = "Link")]
        pub link: String
    }

    #[derive(Debug, Clone, PartialEq, Deserialize)]
    pub struct PassengerVolRawResp {
        pub value: Vec<Link>
    }
}


/// Creates a new client for every call
/// Update freq: By 15th of every month, the passenger volume for previous month data
/// will be generated
///
/// Note: Link will expire after 5mins!
pub fn get_passenger_vol_by(vol_type: passenger_vol::VolType) -> reqwest::Result<Vec<String>> {
    use crate::crowd::passenger_vol::VolType;

    let url = match vol_type {
        VolType::BusStops => passenger_vol::URL_BY_BUS_STOPS,
        VolType::OdBusStop => passenger_vol::URL_BY_OD_BUS_STOPS,
        VolType::Train => passenger_vol::URL_BY_TRAIN,
        VolType::OdTrain => passenger_vol::URL_BY_OD_TRAIN,
    };

    let resp: passenger_vol::PassengerVolRawResp = build_res(url)?;
    let as_str = resp.value.into_iter().map(|f| f.link).collect();

    Ok(as_str)
}