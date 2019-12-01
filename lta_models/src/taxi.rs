pub mod prelude {
    pub use { lta_utils_commons::Coordinates, crate::taxi::taxi_avail::TaxiAvailResp};
}



pub mod taxi_avail {
    use lta_utils_commons::Coordinates;
    use serde::{Deserialize, Serialize};

    pub const URL: &str = "http://datamall2.mytransport.sg/ltaodataservice/Taxi-Availability";

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct InternalCoordinates {
        #[serde(alias = "Longitude")]
        pub long: f64,

        #[serde(alias = "Latitude")]
        pub lat: f64,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct TaxiAvailResp {
        pub value: Vec<InternalCoordinates>,
    }

    impl Into<Vec<Coordinates>> for TaxiAvailResp {
        fn into(self) -> Vec<Coordinates> {
            self.value.into_iter().map(|f| f.into()).collect()
        }
    }

    impl Into<Coordinates> for InternalCoordinates {
        fn into(self) -> Coordinates {
            Coordinates {
                lat: self.lat,
                long: self.long,
            }
        }
    }
}
