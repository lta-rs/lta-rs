//! All API pertaining to traffic related data
use crate::lta_client::LTAClient;
use crate::utils::commons::{build_req, build_req_with_query, Result};

pub mod erp_rates {
    use core::fmt;
    use std::fmt::Formatter;
    use std::str::FromStr;

    use chrono::prelude::*;
    use serde::{Deserialize, Serialize};

    use crate::utils::de::slash_separated;
    use crate::utils::serde_date::{str_date, str_time_option};

    pub const URL: &str = "http://datamall2.mytransport.sg/ltaodataservice/ERPRates";

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub enum VehicleType {
        PassengerCars,
        Motorcycles,
        LightGoodsVehicles,
        HeavyGoodsVehicles,
        VeryHeavyGoodsVehicles,
        Taxis,
        BigBuses,
        None,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct VehicleError;

    impl fmt::Display for VehicleError {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "VehicleErr")
        }
    }

    impl FromStr for VehicleType {
        type Err = VehicleError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let res = match s {
                "Passenger Cars" => VehicleType::PassengerCars,
                "Motorcycles" => VehicleType::Motorcycles,
                "Light Goods Vehicles" => VehicleType::LightGoodsVehicles,
                "Heavy Goods Vehicles" => VehicleType::HeavyGoodsVehicles,
                "Very Heavy Goods Vehicles" => VehicleType::VeryHeavyGoodsVehicles,
                "Taxis" => VehicleType::Taxis,
                "Big Buses" => VehicleType::BigBuses,
                _ => VehicleType::None,
            };

            Ok(res)
        }
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub enum DayType {
        Weekdays,
        Saturday,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub enum ZoneId {
        AY1,
        AYC,
        AYT,
        BKE,
        BKZ,
        BMC,
        CBD,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct ErpRate {
        #[serde(deserialize_with = "slash_separated")]
        pub vehicle_type: Vec<VehicleType>,

        pub day_type: DayType,

        #[serde(with = "str_time_option")]
        pub start_time: Option<NaiveTime>,

        #[serde(with = "str_time_option")]
        pub end_time: Option<NaiveTime>,

        #[serde(rename = "ZoneID")]
        pub zone_id: ZoneId,

        #[serde(rename = "ChargeAmount")]
        pub charge_amt: f32,

        #[serde(with = "str_date")]
        pub effective_date: NaiveDate,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct ErpRatesResp {
        pub value: Vec<ErpRate>,
    }
}

/// Returns ERP rates of all vehicle types across all timings for each
/// zone.
///
/// **Update freq**: Ad-Hoc
/// ## Example
/// ```rust
/// use lta::prelude::*;
/// use lta::Result;
/// use lta::traffic::get_erp_rates;
/// use lta::lta_client::LTAClient;
///
/// fn main() -> Result<()> {
///     let api_key = std::env::var("API_KEY").unwrap();
///     let client = LTAClient::with_api_key(api_key);
///     let erp_rates: Vec<ErpRate> = get_erp_rates(&client)?;
///     println!("{:?}", erp_rates);
///     Ok(())
/// }
/// ```
pub fn get_erp_rates(client: &LTAClient) -> Result<Vec<erp_rates::ErpRate>> {
    build_req::<erp_rates::ErpRatesResp>(client, erp_rates::URL).map(|f| f.value)
}

pub mod carpark_avail {
    use serde::{Deserialize, Serialize};

    use crate::utils::commons::Coordinates;
    use crate::utils::de::from_str_to_coords;

    pub const URL: &str = "http://datamall2.mytransport.sg/ltaodataservice/CarParkAvailabilityv2";

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub enum LotType {
        C,
        L,
        Y,
        H,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub enum Agency {
        HDB,
        URA,
        LTA,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct Carpark {
        #[serde(rename = "CarParkID")]
        pub carpark_id: String,

        pub area: String,

        #[serde(rename = "Development")]
        pub dev: String,

        #[serde(rename = "Location", deserialize_with = "from_str_to_coords")]
        pub coords: Option<Coordinates>,

        #[serde(rename = "AvailableLots")]
        pub avail_lots: u32,

        pub lot_type: LotType,

        pub agency: Agency,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct CarparkAvailResp {
        pub value: Vec<Carpark>,
    }
}

/// Returns no. of available lots for HDB, LTA and URA carpark data.
/// The LTA carpark data consist of major shopping malls and developments within
/// Orchard, Marina, HarbourFront, Jurong Lake District.
/// (Note: list of LTA carpark data available on this API is subset of those listed on
/// One.Motoring and MyTransport Portals)
///
/// **Update freq**: 1 min
/// ## Example
/// ```rust
/// use lta::prelude::*;
/// use lta::Result;
/// use lta::traffic::get_carpark_avail;
/// use lta::lta_client::LTAClient;
///
/// fn main() -> Result<()> {
///     let api_key = std::env::var("API_KEY").unwrap();
///     let client = LTAClient::with_api_key(api_key);
///     let avail_carparks: Vec<Carpark> = get_carpark_avail(&client)?;
///     println!("{:?}", avail_carparks);
///     Ok(())
/// }
/// ```
pub fn get_carpark_avail(client: &LTAClient) -> Result<Vec<carpark_avail::Carpark>> {
    build_req::<carpark_avail::CarparkAvailResp>(client, carpark_avail::URL).map(|f| f.value)
}

pub mod est_travel_time {
    use serde::{Deserialize, Serialize};

    use crate::utils::de::from_int_to_highway;

    pub const URL: &str = "http://datamall2.mytransport.sg/ltaodataservice/EstTravelTimes";

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub enum Highway {
        PIE,
        AYE,
        NSC,
        ECP,
        CTE,
        TPE,
        KPE,
        SLE,
        BKE,
        KJE,
        MCE,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub enum HighwayDirection {
        EastToWest,
        WestToEast,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct EstTravelTime {
        pub name: Highway,

        #[serde(deserialize_with = "from_int_to_highway")]
        pub direction: HighwayDirection,

        #[serde(rename = "FarEndPoint")]
        pub far_end_pt: String,

        #[serde(rename = "StartPoint")]
        pub start_pt: String,

        #[serde(rename = "EndPoint")]
        pub end_pt: String,

        #[serde(rename = "EstTime")]
        pub est_travel_time: u32,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct EstTravelTimeResp {
        pub value: Vec<EstTravelTime>,
    }
}

/// Returns estimated travel times of expressways (in segments).
///
/// **Update freq**: 5min
/// ## Example
/// ```rust
/// use lta::prelude::*;
/// use lta::Result;
/// use lta::traffic::get_est_travel_time;
/// use lta::lta_client::LTAClient;
///
/// fn main() -> Result<()> {
///     let api_key = std::env::var("API_KEY").unwrap();
///     let client = LTAClient::with_api_key(api_key);
///     let est_travel_time: Vec<EstTravelTime> = get_est_travel_time(&client)?;
///     println!("{:?}", est_travel_time);
///     Ok(())
/// }
/// ```
pub fn get_est_travel_time(
    client: &LTAClient,
) -> reqwest::Result<Vec<est_travel_time::EstTravelTime>> {
    build_req::<est_travel_time::EstTravelTimeResp>(client, est_travel_time::URL).map(|f| f.value)
}

pub mod faulty_traffic_lights {
    use chrono::prelude::*;
    use serde::{Deserialize, Serialize};

    use crate::utils::serde_date::ymd_hms_option;

    pub const URL: &str = "http://datamall2.mytransport.sg/ltaodataservice/FaultyTrafficLights";

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub enum TechnicalAlarmType {
        Blackout = 4,
        FlashingYellow = 13,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct FaultyTrafficLight {
        #[serde(rename = "AlarmID")]
        pub alarm_id: String,

        #[serde(rename = "NodeID")]
        pub node_id: String,

        #[serde(rename = "Type")]
        pub technical_alarm_type: TechnicalAlarmType,

        #[serde(with = "ymd_hms_option")]
        pub start_date: Option<DateTime<FixedOffset>>,

        #[serde(with = "ymd_hms_option")]
        pub end_date: Option<DateTime<FixedOffset>>,

        pub message: String,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct FaultyTrafficLightResp {
        pub value: Vec<FaultyTrafficLight>,
    }
}

/// Returns alerts of traffic lights that are currently faulty, or currently
/// undergoing scheduled maintenance.
///
/// **Update freq**: 2min or whenever there are updates
/// ## Example
/// ```rust
/// use lta::prelude::*;
/// use lta::Result;
/// use lta::traffic::get_faulty_traffic_lights;
/// use lta::lta_client::LTAClient;
///
/// fn main() -> Result<()> {
///     let api_key = std::env::var("API_KEY").unwrap();
///     let client = LTAClient::with_api_key(api_key);
///     let faulty_traffic_lights: Vec<FaultyTrafficLight> = get_faulty_traffic_lights(&client)?;
///     println!("{:?}", faulty_traffic_lights);
///     Ok(())
/// }
/// ```
pub fn get_faulty_traffic_lights(
    client: &LTAClient,
) -> Result<Vec<faulty_traffic_lights::FaultyTrafficLight>> {
    build_req::<faulty_traffic_lights::FaultyTrafficLightResp>(client, faulty_traffic_lights::URL)
        .map(|f| f.value)
}

pub mod road {
    use chrono::prelude::*;
    use serde::{Deserialize, Serialize};

    use crate::utils::serde_date::str_date;

    pub const URL_ROAD_OPENING: &str =
        "http://datamall2.mytransport.sg/ltaodataservice/RoadOpenings";
    pub const URL_ROAD_WORKS: &str = "http://datamall2.mytransport.sg/ltaodataservice/RoadWorks";

    pub enum RoadDetailsType {
        RoadOpening,
        RoadWorks,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct RoadDetails {
        #[serde(rename = "EventID")]
        pub event_id: String,

        #[serde(with = "str_date")]
        pub start_date: NaiveDate,

        #[serde(with = "str_date")]
        pub end_date: NaiveDate,

        #[serde(rename = "SvcDept")]
        pub service_dept: String,

        pub road_name: String,

        pub other: String,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct RoadDetailsResp {
        pub value: Vec<RoadDetails>,
    }
}

/// Returns all planned road openings or road works depending on the `RoadDetailsType` supplied
///
/// **Update freq**: 24 hours – whenever there are updates
/// ## Example
/// ```rust
/// use lta::prelude::*;
/// use lta::Result;
/// use lta::traffic::get_road_details;
/// use lta::lta_client::LTAClient;
///
/// fn main() -> Result<()> {
///     let api_key = std::env::var("API_KEY").unwrap();
///     let client = LTAClient::with_api_key(api_key);
///     let road_details: Vec<RoadDetails> = get_road_details(&client, RoadDetailsType::RoadWorks)?;
///     println!("{:?}", road_details);
///     Ok(())
/// }
/// ```
pub fn get_road_details(
    client: &LTAClient,
    road_details_type: road::RoadDetailsType,
) -> Result<Vec<road::RoadDetails>> {
    let url = match road_details_type {
        road::RoadDetailsType::RoadOpening => road::URL_ROAD_OPENING,
        road::RoadDetailsType::RoadWorks => road::URL_ROAD_WORKS,
    };

    build_req::<road::RoadDetailsResp>(client, url).map(|f| f.value)
}

pub mod traffic_images {
    use serde::{Deserialize, Serialize};

    use crate::utils::de::from_str;

    pub const URL: &str = "http://datamall2.mytransport.sg/ltaodataservice/Traffic-Images";

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct TrafficImage {
        #[serde(rename = "CameraID", deserialize_with = "from_str")]
        pub camera_id: u32,

        #[serde(rename = "Latitude")]
        pub lat: f64,

        #[serde(rename = "Longitude")]
        pub long: f64,

        #[serde(rename = "ImageLink")]
        pub image_link: String,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct TrafficImageResp {
        pub value: Vec<TrafficImage>,
    }
}

/// Returns links to images of live traffic conditions along expressways and
/// Woodlands & Tuas Checkpoints.
///
/// **Update freq**: 1 to 5 minutes
/// ## Example
/// ```rust
/// use lta::prelude::*;
/// use lta::Result;
/// use lta::traffic::get_traffic_images;
/// use lta::lta_client::LTAClient;
///
/// fn main() -> Result<()> {
///     let api_key = std::env::var("API_KEY").unwrap();
///     let client = LTAClient::with_api_key(api_key);
///     let traffic_images: Vec<TrafficImage> = get_traffic_images(&client)?;
///     println!("{:?}", traffic_images);
///     Ok(())
/// }
/// ```
pub fn get_traffic_images(client: &LTAClient) -> Result<Vec<traffic_images::TrafficImage>> {
    build_req::<traffic_images::TrafficImageResp>(client, traffic_images::URL).map(|f| f.value)
}

pub mod traffic_incidents {
    use serde::{Deserialize, Serialize};

    pub const URL: &str = "http://datamall2.mytransport.sg/ltaodataservice/TrafficIncidents";

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub enum IncidentType {
        Accident,

        #[serde(rename = "Road Works")]
        RoadWorks,

        #[serde(rename = "Vehicle breakdown")]
        VehicleBreakdown,

        Weather,

        Obstacle,

        #[serde(rename = "Road Block")]
        RoadBlock,

        #[serde(rename = "Heavy Traffic")]
        HeavyTraffic,

        #[serde(rename = "Misc.")]
        Misc,

        Diversion,

        #[serde(rename = "Unattended Vehicle")]
        UnattendedVehicle,

        Roadwork,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct TrafficIncident {
        #[serde(rename = "Type")]
        pub incident_type: IncidentType,

        #[serde(rename = "Latitude")]
        pub lat: f64,

        #[serde(rename = "Longitude")]
        pub long: f64,

        #[serde(rename = "Message")]
        pub msg: String,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct TrafficIncidentResp {
        pub value: Vec<TrafficIncident>,
    }
}

/// Returns current traffic speeds on expressways and arterial roads,
/// expressed in speed bands.
///
/// **Update freq**: 5 minutes
/// ## Example
/// ```rust
/// use lta::prelude::*;
/// use lta::Result;
/// use lta::traffic::get_traffic_incidents;
/// use lta::lta_client::LTAClient;
///
/// fn main() -> Result<()> {
///     let api_key = std::env::var("API_KEY").unwrap();
///     let client = LTAClient::with_api_key(api_key);
///     let traffic_incidents: Vec<TrafficIncident> = get_traffic_incidents(&client)?;
///     println!("{:?}", traffic_incidents);
///     Ok(())
/// }
/// ```
pub fn get_traffic_incidents(
    client: &LTAClient,
) -> Result<Vec<traffic_incidents::TrafficIncident>> {
    build_req::<traffic_incidents::TrafficIncidentResp>(client, traffic_incidents::URL)
        .map(|f| f.value)
}

pub mod traffic_speed_bands {
    use serde::{Deserialize, Serialize};

    use crate::utils::commons::Location;
    use crate::utils::de::{from_str, from_str_loc_to_loc};

    pub const URL: &str = "http://datamall2.mytransport.sg/ltaodataservice/TrafficSpeedBandsv2";

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub enum RoadCategory {
        #[serde(rename = "A")]
        Expressway,

        #[serde(rename = "B")]
        MajorArterialRoads,

        #[serde(rename = "C")]
        ArterialRoads,

        #[serde(rename = "D")]
        MinorArterialRoads,

        #[serde(rename = "E")]
        SmallRoads,

        #[serde(rename = "F")]
        SlipRoads,

        #[serde(rename = "G")]
        NoCategoryInfoAvail,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct TrafficSpeedBand {
        #[serde(rename = "LinkID", deserialize_with = "from_str")]
        pub link_id: u64,

        pub road_name: String,

        pub road_category: RoadCategory,

        pub speed_band: u32,

        #[serde(rename = "MinimumSpeed", deserialize_with = "from_str")]
        pub min_speed: u32,

        #[serde(rename = "MaximumSpeed", deserialize_with = "from_str")]
        pub max_speed: u32,

        #[serde(rename = "Location", deserialize_with = "from_str_loc_to_loc")]
        pub coord_start_end: Option<Location>,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct TrafficSpeedBandResp {
        pub value: Vec<TrafficSpeedBand>,
    }
}

/// Returns current traffic speeds on expressways and arterial roads,
/// expressed in speed bands.
///
/// **Update freq**: 5 minutes
/// ## Example
/// ```rust
/// use lta::prelude::*;
/// use lta::Result;
/// use lta::traffic::get_traffic_speed_band;
/// use lta::lta_client::LTAClient;
///
/// fn main() -> Result<()> {
///     let api_key = std::env::var("API_KEY").unwrap();
///     let client = LTAClient::with_api_key(api_key);
///     let speed_bands: Vec<TrafficSpeedBand> = get_traffic_speed_band(&client)?;
///     println!("{:?}", speed_bands);
///     Ok(())
/// }
/// ```
pub fn get_traffic_speed_band(
    client: &LTAClient,
) -> Result<Vec<traffic_speed_bands::TrafficSpeedBand>> {
    build_req::<traffic_speed_bands::TrafficSpeedBandResp>(client, traffic_speed_bands::URL)
        .map(|f| f.value)
}

pub mod vms_emas {
    use serde::{Deserialize, Serialize};

    pub const URL: &str = "http://datamall2.mytransport.sg/ltaodataservice/VMS";

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct VMS {
        #[serde(rename = "EquipmentID")]
        pub equipment_id: String,

        #[serde(rename = "Latitude")]
        pub lat: f64,

        #[serde(rename = "Longitude")]
        pub long: f64,

        #[serde(rename = "Message")]
        pub msg: String,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct VMSResp {
        pub value: Vec<VMS>,
    }
}

/// Returns traffic advisories (via variable message services) concerning
/// current traffic conditions that are displayed on EMAS signboards
/// along expressways and arterial roads.
///
/// **Update freq**: 2 minutes
/// ## Example
/// ```rust
/// use lta::prelude::*;
/// use lta::Result;
/// use lta::traffic::get_vms_emas;
/// use lta::lta_client::LTAClient;
///
/// fn main() -> Result<()> {
///     let api_key = std::env::var("API_KEY").unwrap();
///     let client = LTAClient::with_api_key(api_key);
///     let vms_emas: Vec<VMS> = get_vms_emas(&client)?;
///     println!("{:?}", vms_emas);
///     Ok(())
/// }
/// ```
pub fn get_vms_emas(client: &LTAClient) -> Result<Vec<vms_emas::VMS>> {
    build_req::<vms_emas::VMSResp>(client, vms_emas::URL).map(|f| f.value)
}

pub mod bike_parking {
    use serde::{Deserialize, Serialize};

    use crate::utils::de::from_str_shelter_indicator_to_bool;

    pub const URL: &str = "http://datamall2.mytransport.sg/ltaodataservice/BicycleParkingv2";

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub enum RackType {
        Racks,
        #[serde(rename = "Yellow Box")]
        YellowBox,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct BikeParking {
        #[serde(rename = "Description")]
        pub desc: String,

        #[serde(rename = "Latitude")]
        pub lat: f64,

        #[serde(rename = "Longitude")]
        pub long: f64,

        pub rack_type: RackType,

        pub rack_count: u32,

        #[serde(deserialize_with = "from_str_shelter_indicator_to_bool")]
        pub shelter_indicator: bool,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct BikeParkingResp {
        pub value: Vec<BikeParking>,
    }
}

/// Returns bicycle parking locations within a radius
///
/// Dist is default to 0.5 even if you provide `None`
///
/// **Update freq**: Monthly
/// ## Example
/// ```rust
/// use lta::prelude::*;
/// use lta::Result;
/// use lta::traffic::get_bike_parking;
/// use lta::lta_client::LTAClient;
///
/// fn main() -> Result<()> {
///     let api_key = std::env::var("API_KEY").unwrap();
///     let client = LTAClient::with_api_key(api_key);
///     let bike_parking: Vec<BikeParking> = get_bike_parking(&client, 1.364897, 103.766094, None)?;
///     println!("{:?}", bike_parking);
///     Ok(())
/// }
/// ```
pub fn get_bike_parking(
    client: &LTAClient,
    lat: f64,
    long: f64,
    dist: Option<f64>,
) -> Result<Vec<bike_parking::BikeParking>> {
    let unwrapped_dist = dist.unwrap_or(0.5);
    build_req_with_query::<bike_parking::BikeParkingResp, _>(client, bike_parking::URL, |rb| {
        rb.query(&[("Lat", lat), ("Long", long), ("Dist", unwrapped_dist)])
    })
    .map(|f| f.value)
}
