//! Crowd structs and data structures

pub mod prelude {
    pub use crate::crowd::passenger_vol::{Link, PassengerVolRawResp, VolType};
}

pub mod passenger_vol {
    use serde::{Deserialize, Serialize};

    pub const URL_BY_BUS_STOPS: &str = "http://datamall2.mytransport.sg/ltaodataservice/PV/Bus";

    pub const URL_BY_OD_BUS_STOPS: &str =
        "http://datamall2.mytransport.sg/ltaodataservice/PV/ODBus";

    pub const URL_BY_TRAIN: &str = "http://datamall2.mytransport.sg/ltaodataservice/PV/Train";

    pub const URL_BY_OD_TRAIN: &str = "http://datamall2.mytransport.sg/ltaodataservice/PV/ODTrain";

    pub const FORMAT: &str = "%Y%m";

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

        #[serde(other)]
        Unknown,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct Link {
        #[serde(alias = "Link")]
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
