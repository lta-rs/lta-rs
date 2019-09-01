//! All API pertaining to buses
use crate::lta_client::LTAClient;
use crate::utils::commons::{build_req, build_res_with_query, Result};

pub mod bus_arrival {
    use chrono::prelude::*;
    use serde::{Deserialize, Serialize};

    use crate::bus_enums::{BusFeature, BusLoad, BusType, Operator};
    use crate::utils::de::from_str;

    pub const URL: &str = "http://datamall2.mytransport.sg/ltaodataservice/BusArrivalv2";

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct ArrivalBusService {
        pub service_no: String,
        pub operator: Operator,
        pub next_bus: Option<NextBus>,
        pub next_bus_2: Option<NextBus>,
        pub next_bus_3: Option<NextBus>,
    }

    impl ArrivalBusService {
        pub fn next_bus_as_arr(&self) -> [&Option<NextBus>; 3] {
            [&self.next_bus, &self.next_bus_2, &self.next_bus_3]
        }
    }

    /// Representation is similar to the one
    /// [here](https://www.mytransport.sg/content/dam/datamall/datasets/LTA_DataMall_API_User_Guide.pdf)
    /// in order to keep it consistent with the API itself in case anyone wants to
    /// reference the original docs
    #[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct NextBus {
        /// Original response returns a `String`
        ///
        /// String is then deserialized to `u32`
        ///
        /// Represents starting bus stop code
        #[serde(deserialize_with = "from_str")]
        pub origin_code: u32,

        /// Original response returns a `String`
        ///
        /// String is then deserialized to `u32`
        ///
        /// Represents ending bus stop code
        #[serde(deserialize_with = "from_str", rename = "DestinationCode")]
        pub dest_code: u32,

        /// Represents starting bus stop code
        ///
        /// Original response returns a `String`
        ///
        /// Example response: `2019-07-21T13:12:41+08:00`
        ///
        /// String is then deserialize to `Datetime<FixedOffset>`
        #[serde(rename = "EstimatedArrival")]
        pub est_arrival: DateTime<FixedOffset>,

        /// Original response returns a `String`
        ///
        /// String is then deserialized to `f64`
        ///
        /// Represents latitude of bus
        #[serde(deserialize_with = "from_str", rename = "Latitude")]
        pub lat: f64,

        /// Original response returns a `String`
        ///
        /// String is then deserialized to `f64`
        ///
        /// Represents longitude of bus
        #[serde(deserialize_with = "from_str", rename = "Longitude")]
        pub long: f64,

        /// Original response returns a `String`
        ///
        /// String is then deserialized to `u32`
        ///
        /// Represents number of times the bus visited
        #[serde(deserialize_with = "from_str", rename = "VisitNumber")]
        pub visit_no: u32,

        /// Original response returns a `String`
        ///
        /// String is then deserialized to `BusLoad` enum
        ///
        /// Represents the load the bus has
        pub load: BusLoad,

        /// Original response returns a `String`
        ///
        /// String is then deserialized to `Option<BusFeature>`
        ///
        /// Represents features the bus has
        pub feature: Option<BusFeature>,

        /// Original response returns a `String`
        ///
        /// String is then deserialized to `BusType` enum
        ///
        /// Represents the bus type
        #[serde(rename = "Type")]
        pub bus_type: BusType,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct BusArrivalResp {
        #[serde(deserialize_with = "from_str")]
        pub bus_stop_code: u32,
        pub services: Vec<ArrivalBusService>,
    }
}

/// Returns real-time Bus Arrival information of Bus Services at a queried Bus Stop,
/// including
/// - Estimated Arrival Time
/// - Estimated Current Location
/// - Estimated Current Load.
///
/// Sometimes, it may return an empty Vec
///
/// If that happens, it means that there are no services at that timing.
///
/// **Update freq**: 1min
///
/// ## Example
/// ```rust
/// use lta::prelude::*;
/// use lta::Result;
/// use lta::lta_client::LTAClient;
/// use lta::bus::get_arrival;
///
/// fn main() -> Result<()> {
///     let api_key = std::env::var("API_KEY").unwrap();
///     let client = LTAClient::with_api_key(api_key);
///     let arrivals: BusArrivalResp = get_arrival(&client, 83139, Some("15"))?;
///     println!("{:?}", arrivals);
///     Ok(())
/// }
/// ```
pub fn get_arrival(
    client: &LTAClient,
    bus_stop_code: u32,
    service_no: Option<&str>,
) -> Result<bus_arrival::BusArrivalResp> {
    let resp: bus_arrival::BusArrivalResp = match service_no {
        Some(srv_no) => build_res_with_query(client, bus_arrival::URL, |rb| {
            rb.query(&[
                ("BusStopCode", bus_stop_code.to_string()),
                ("ServiceNo", srv_no.to_string()),
            ])
        })?,
        None => build_res_with_query(client, bus_arrival::URL, |rb| {
            rb.query(&[("BusStopCode", bus_stop_code.to_string())])
        })?,
    };

    Ok(resp)
}

pub mod bus_services {
    use serde::{Deserialize, Serialize};

    use crate::bus_enums::{BusCategory, Operator};
    use crate::utils::de::{from_str, from_str_to_bus_freq};

    pub const URL: &str = "http://datamall2.mytransport.sg/ltaodataservice/BusServices";

    /// Both min and max are in terms of minutes
    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub struct BusFreq {
        pub min: Option<u32>,
        pub max: Option<u32>,
    }

    impl BusFreq {
        pub fn new(min: u32, max: u32) -> Self {
            BusFreq {
                min: Some(min),
                max: Some(max),
            }
        }

        pub fn default() -> Self {
            BusFreq::new(0, 0)
        }

        pub fn no_max(min: u32) -> Self {
            BusFreq {
                min: Some(min),
                max: None,
            }
        }

        pub fn no_timing() -> Self {
            BusFreq {
                min: None,
                max: None,
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct BusService {
        pub service_no: String,

        pub operator: Operator,

        #[serde(rename = "Direction")]
        pub no_direction: u32,

        pub category: BusCategory,

        #[serde(deserialize_with = "from_str")]
        pub origin_code: u32,

        #[serde(deserialize_with = "from_str", rename = "DestinationCode")]
        pub dest_code: u32,

        #[serde(rename = "AM_Peak_Freq", deserialize_with = "from_str_to_bus_freq")]
        pub am_peak_freq: BusFreq,

        #[serde(rename = "AM_Offpeak_Freq", deserialize_with = "from_str_to_bus_freq")]
        pub am_offpeak_freq: BusFreq,

        #[serde(rename = "PM_Peak_Freq", deserialize_with = "from_str_to_bus_freq")]
        pub pm_peak_freq: BusFreq,

        #[serde(rename = "PM_Offpeak_Freq", deserialize_with = "from_str_to_bus_freq")]
        pub pm_offpeak_freq: BusFreq,

        pub loop_desc: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct BusServiceResp {
        pub value: Vec<BusService>,
    }
}

/// Returns detailed service information for all buses currently in
/// operation, including: first stop, last stop, peak / offpeak frequency of
/// dispatch.
///
/// **Update freq**: Ad-Hoc
///
/// ## Example
/// ```rust
/// use lta::prelude::*;
/// use lta::bus::get_bus_services;
/// use lta::lta_client::LTAClient;
/// use lta::Result;
///
/// fn main() -> Result<()> {
///     let api_key = std::env::var("API_KEY").unwrap();
///     let client = LTAClient::with_api_key(api_key);
///     let bus_services: Vec<BusService> = get_bus_services(&client)?;
///     println!("{:?}", bus_services);
///     Ok(())
/// }
/// ```
pub fn get_bus_services(client: &LTAClient) -> Result<Vec<bus_services::BusService>> {
    let resp: bus_services::BusServiceResp = build_req(client, bus_services::URL)?;
    Ok(resp.value)
}

pub mod bus_routes {
    use chrono::prelude::*;
    use serde::{Deserialize, Serialize};

    use crate::bus_enums::Operator;
    use crate::utils::de::{from_str, from_str_to_time};
    use crate::utils::ser::from_time_to_str;

    pub const URL: &str = "http://datamall2.mytransport.sg/ltaodataservice/BusRoutes";

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct BusRoute {
        pub service_no: String,

        pub operator: Operator,

        pub direction: u32,

        #[serde(rename = "StopSequence")]
        pub stop_seq: u32,

        #[serde(deserialize_with = "from_str")]
        pub bus_stop_code: u32,

        #[serde(rename = "Distance")]
        pub dist: f64,

        #[serde(
            rename = "WD_FirstBus",
            deserialize_with = "from_str_to_time",
            serialize_with = "from_time_to_str"
        )]
        pub wd_first: Option<NaiveTime>,

        #[serde(
            rename = "WD_LastBus",
            deserialize_with = "from_str_to_time",
            serialize_with = "from_time_to_str"
        )]
        pub wd_last: Option<NaiveTime>,

        #[serde(
            rename = "SAT_FirstBus",
            deserialize_with = "from_str_to_time",
            serialize_with = "from_time_to_str"
        )]
        pub sat_first: Option<NaiveTime>,

        #[serde(
            rename = "SAT_LastBus",
            deserialize_with = "from_str_to_time",
            serialize_with = "from_time_to_str"
        )]
        pub sat_last: Option<NaiveTime>,

        #[serde(
            rename = "SUN_FirstBus",
            deserialize_with = "from_str_to_time",
            serialize_with = "from_time_to_str"
        )]
        pub sun_first: Option<NaiveTime>,

        #[serde(
            rename = "SUN_LastBus",
            deserialize_with = "from_str_to_time",
            serialize_with = "from_time_to_str"
        )]
        pub sun_last: Option<NaiveTime>,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct BusRouteResp {
        pub value: Vec<BusRoute>,
    }
}

/// Returns detailed route information for all services currently in operation,
/// including: all bus stops along each route, first/last bus timings for each stop
///
/// **Update freq**: Ad-Hoc
///
/// ## Example
/// ```rust
/// use lta::prelude::*;
/// use lta::Result;
/// use lta::bus::get_bus_routes;
/// use lta::lta_client::LTAClient;
///
/// fn main() -> Result<()> {
///     let api_key = std::env::var("API_KEY").unwrap();
///     let client = LTAClient::with_api_key(api_key);
///     let bus_routes: Vec<BusRoute> = get_bus_routes(&client)?;
///     println!("{:?}", bus_routes);
///     Ok(())
/// }
/// ```
pub fn get_bus_routes(client: &LTAClient) -> Result<Vec<bus_routes::BusRoute>> {
    let resp: bus_routes::BusRouteResp = build_req(client, bus_routes::URL)?;
    Ok(resp.value)
}

pub mod bus_stops {
    use serde::{Deserialize, Serialize};

    use crate::utils::commons::Coordinates;
    use crate::utils::de::from_str;

    pub const URL: &str = "http://datamall2.mytransport.sg/ltaodataservice/BusStops";

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct BusStop {
        #[serde(deserialize_with = "from_str")]
        pub bus_stop_code: u32,

        pub road_name: String,

        #[serde(rename = "Description")]
        pub desc: String,

        #[serde(rename = "Latitude")]
        pub lat: f64,

        #[serde(rename = "Longitude")]
        pub long: f64,
    }

    impl BusStop {
        pub fn coordinates(&self) -> Coordinates {
            Coordinates::new(self.lat, self.long)
        }
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct BusStopsResp {
        pub value: Vec<BusStop>,
    }
}

/// Returns detailed information for all bus stops currently being serviced by
/// buses, including: Bus Stop Code, location coordinates.
///
/// **Update freq**: Ad-Hoc
///
/// ## Example
/// ```rust
/// use lta::prelude::*;
/// use lta::Result;
/// use lta::bus::get_bus_stops;
/// use lta::lta_client::LTAClient;
///
/// fn main() -> Result<()> {
///     let api_key = std::env::var("API_KEY").unwrap();
///     let client = LTAClient::with_api_key(api_key);
///     let bus_stops: Vec<BusStop> = get_bus_stops(&client)?;
///     println!("{:?}", bus_stops);
///     Ok(())
/// }
/// ```
pub fn get_bus_stops(client: &LTAClient) -> Result<Vec<bus_stops::BusStop>> {
    let resp: bus_stops::BusStopsResp = build_req(client, bus_stops::URL)?;
    Ok(resp.value)
}
