use crate::client_config::CLIENT_CONFIG;
use crate::utils::commons::{build_res, build_res_with_query};

pub mod bus_arrival {
    use serde::Deserialize;

    use crate::bus_enums::{BusFeature, BusLoad, BusType, Operator};
    use crate::utils::de::from_str;

    pub const URL: &'static str = "http://datamall2.mytransport.sg/ltaodataservice/BusArrivalv2";

    #[derive(Debug, Clone, PartialEq, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct ArrivalBusService {
        pub service_no: String,
        pub operator: Operator,
        pub next_bus: NextBus,
        pub next_bus_2: NextBus,
        pub next_bus_3: NextBus,
    }

    impl ArrivalBusService {
        fn next_bus_as_vec(&self) -> Vec<&NextBus> {
            vec![&self.next_bus, &self.next_bus_2, &self.next_bus_3]
        }
    }

    /// Representation is similar to the one on
    /// https://www.mytransport.sg/content/dam/datamall/datasets/LTA_DataMall_API_User_Guide.pdf
    /// in order to keep it consistent with the API itself in case anyone wants to
    /// reference the original docs
    #[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct NextBus {
        /// Original response returns a [`String`]
        ///
        /// String is then deserialized to [`u32`]
        ///
        /// Represents starting bus stop code
        #[serde(deserialize_with = "from_str")]
        pub origin_code: u32,

        /// Original response returns a string
        ///
        /// String is then deserialized to u32
        ///
        /// Represents ending bus stop code
        #[serde(deserialize_with = "from_str", rename = "DestinationCode")]
        pub dest_code: u32,

        /// Represents starting bus stop code
        #[serde(rename = "EstimatedArrival")]
        pub est_arrival: String,

        /// Original response returns a string
        ///
        /// String is then deserialized to f64
        ///
        /// Represents latitude of bus
        #[serde(deserialize_with = "from_str", rename = "Latitude")]
        pub lat: f64,

        /// Original response returns a string
        ///
        /// String is then deserialized to f64
        ///
        /// Represents longitude of bus
        #[serde(deserialize_with = "from_str", rename = "Longitude")]
        pub long: f64,

        /// Original response returns a string
        ///
        /// String is then deserialized to u32
        ///
        /// Represents number of times the bus visited
        #[serde(deserialize_with = "from_str", rename = "VisitNumber")]
        pub visit_no: u32,

        /// Original response returns a string
        ///
        /// String is then deserialized to BusLoad enum
        ///
        /// Represents the load the bus has
        pub load: BusLoad,

        /// Original response returns a string
        ///
        /// String is then deserialized to Option<BusFeature>
        ///
        /// Represents features the bus has
        pub feature: Option<BusFeature>,

        /// Original response returns a string
        ///
        /// String is then deserialized to BusType enum
        ///
        /// Represents the bus type
        #[serde(rename = "Type")]
        pub bus_type: BusType,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct BusArrivalResp {
        #[serde(deserialize_with = "from_str")]
        pub bus_stop_code: u32,
        pub services: Vec<ArrivalBusService>,
    }
}


///
/// Returns real-time Bus Arrival information of Bus Services at a queried Bus Stop,
/// including Est. Arrival Time, Est. Current Location, Est. Current Load.
///
/// Sometimes, it may return an empty Vec
///
/// If that happens, it means that there are no services at that timing.
///
/// Update Freq: 1min
pub fn get_arrival(bus_stop_code: u32, service_no: &str) -> reqwest::Result<bus_arrival::BusArrivalResp> {
    let resp: bus_arrival::BusArrivalResp =
        build_res_with_query(bus_arrival::URL, |req_builder| {
            req_builder.query(&[
                ("BusStopCode", bus_stop_code.to_string()),
                ("ServiceNo", service_no.to_string())
            ])
        })?;
    Ok(resp)
}

pub mod bus_services {
    use serde::Deserialize;

    use crate::bus_enums::{BusCategory, Operator};
    use crate::utils::de::{from_str, from_str_to_bus_freq};

    pub const URL: &'static str = "http://datamall2.mytransport.sg/ltaodataservice/BusServices";

    #[derive(Debug, Clone, PartialEq)]
    pub struct BusFreq {
        pub min: Option<u32>,
        pub max: Option<u32>,
    }

    impl BusFreq {
        pub fn new(min: u32, max: u32) -> Self {
            BusFreq { min: Some(min), max: Some(max) }
        }

        pub fn default() -> Self {
            BusFreq::new(0, 0)
        }

        pub fn no_max(min: u32) -> Self {
            BusFreq { min: Some(min), max: None }
        }

        pub fn no_timing() -> Self {
            BusFreq { min: None, max: None }
        }
    }

    #[derive(Debug, Clone, PartialEq, Deserialize)]
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

    #[derive(Debug, Clone, PartialEq, Deserialize)]
    pub struct BusServiceResp {
        pub value: Vec<BusService>
    }
}


///
/// Returns detailed service information for all buses currently in
/// operation, including: first stop, last stop, peak / offpeak frequency of
/// dispatch.
///
/// Update freq: Ad-Hoc
pub fn get_bus_services() -> reqwest::Result<Vec<bus_services::BusService>> {
    let resp: bus_services::BusServiceResp = build_res(bus_services::URL)?;
    Ok(resp.value)
}

pub mod bus_routes {
    use serde::Deserialize;

    use crate::bus_enums::Operator;
    use crate::utils::de::from_str;

    pub const URL: &'static str = "http://datamall2.mytransport.sg/ltaodataservice/BusRoutes";

    #[derive(Debug, Clone, PartialEq, Deserialize)]
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

        #[serde(rename = "WD_FirstBus")]
        pub wd_first: String,

        #[serde(rename = "WD_LastBus")]
        pub wd_last: String,

        #[serde(rename = "SAT_FirstBus")]
        pub sat_first: String,

        #[serde(rename = "SAT_LastBus")]
        pub sat_last: String,

        #[serde(rename = "SUN_FirstBus")]
        pub sun_first: String,

        #[serde(rename = "SUN_LastBus")]
        pub sun_last: String,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize)]
    pub struct BusRouteResp {
        pub value: Vec<BusRoute>
    }
}


///
/// Returns detailed route information for all services currently in operation,
/// including: all bus stops along each route, first/last bus timings for each stop
///
/// Update freq: Ad-Hoc
pub fn get_bus_routes() -> reqwest::Result<Vec<bus_routes::BusRoute>> {
    let resp: bus_routes::BusRouteResp = build_res(bus_routes::URL)?;
    Ok(resp.value)
}

pub mod bus_stops {
    use serde::Deserialize;

    use crate::utils::de::from_str;

    pub const URL: &'static str = "http://datamall2.mytransport.sg/ltaodataservice/BusStops";

    #[derive(Debug, Clone, PartialEq, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct BusStop {
        #[serde(deserialize_with = "from_str")]
        pub bus_stop_code: u32,

        pub road_name: String,

        #[serde(rename = "Description")]
        pub desc: String,

        #[serde(rename = "Latitude")]
        pub lat: f64,

        #[serde(rename = "Lo1ngitude")]
        pub long: f64,
    }


    #[derive(Debug, Clone, PartialEq, Deserialize)]
    pub struct BusStopsResp {
        pub value: Vec<BusStop>
    }
}


///
/// Returns detailed information for all bus stops currently being serviced by
/// buses, including: Bus Stop Code, location coordinates.
///
/// Update freq: Ad-Hoc
pub fn get_bus_stops() -> reqwest::Result<Vec<bus_stops::BusStop>> {
    let resp: bus_stops::BusStopsResp = build_res(bus_stops::URL)?;
    Ok(resp.value)
}

