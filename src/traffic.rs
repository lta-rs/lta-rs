use crate::lta_client::LTAClient;
use crate::utils::commons::{build_req, build_res_with_query};

pub mod erp_rates {
    use core::fmt;
    use std::fmt::Formatter;
    use std::str::FromStr;

    use chrono::prelude::*;
    use serde::{Deserialize, Serialize};

    use crate::utils::de::{from_str_to_date, from_str_to_time, slash_separated};
    use crate::utils::ser::{from_date_to_str, from_time_to_str};

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

        #[serde(
            deserialize_with = "from_str_to_time",
            serialize_with = "from_time_to_str"
        )]
        pub start_time: Option<NaiveTime>,

        #[serde(
            deserialize_with = "from_str_to_time",
            serialize_with = "from_time_to_str"
        )]
        pub end_time: Option<NaiveTime>,

        #[serde(rename = "ZoneID")]
        pub zone_id: ZoneId,

        #[serde(rename = "ChargeAmount")]
        pub charge_amt: f32,

        #[serde(
            deserialize_with = "from_str_to_date",
            serialize_with = "from_date_to_str"
        )]
        pub effective_date: Date<FixedOffset>,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct ErpRatesResp {
        pub value: Vec<ErpRate>,
    }
}

/// Returns ERP rates of all vehicle types across all timings for each
/// zone.
///
/// Update freq: Ad-Hoc
pub fn get_erp_rates(client: &LTAClient) -> reqwest::Result<Vec<erp_rates::ErpRate>> {
    let resp: erp_rates::ErpRatesResp = build_req(client, erp_rates::URL)?;
    Ok(resp.value)
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
/// Update freq: 1 min
pub fn get_carpark_avail(client: &LTAClient) -> reqwest::Result<Vec<carpark_avail::Carpark>> {
    let resp: carpark_avail::CarparkAvailResp = build_req(client, carpark_avail::URL)?;
    Ok(resp.value)
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
/// Update freq: 5min
pub fn get_est_travel_time(
    client: &LTAClient,
) -> reqwest::Result<Vec<est_travel_time::EstTravelTime>> {
    let resp: est_travel_time::EstTravelTimeResp = build_req(client, est_travel_time::URL)?;
    Ok(resp.value)
}

pub mod faulty_traffic_lights {
    use chrono::prelude::*;
    use serde::{Deserialize, Serialize};

    use crate::utils::de::from_str_to_datetime;
    use crate::utils::ser::from_datetime_to_str;

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

        #[serde(
            deserialize_with = "from_str_to_datetime",
            serialize_with = "from_datetime_to_str"
        )]
        pub start_date: Option<DateTime<FixedOffset>>,

        #[serde(
            deserialize_with = "from_str_to_datetime",
            serialize_with = "from_datetime_to_str"
        )]
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
/// Update freq: 2min or whenever there are updates
pub fn get_faulty_traffic_lights(
    client: &LTAClient,
) -> reqwest::Result<Vec<faulty_traffic_lights::FaultyTrafficLight>> {
    let resp: faulty_traffic_lights::FaultyTrafficLightResp =
        build_req(client, faulty_traffic_lights::URL)?;
    Ok(resp.value)
}

pub mod road {
    use chrono::prelude::*;
    use serde::{Deserialize, Serialize};

    use crate::utils::de::from_str_to_date;
    use crate::utils::ser::from_date_to_str;

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

        #[serde(
            deserialize_with = "from_str_to_date",
            serialize_with = "from_date_to_str"
        )]
        pub start_date: Date<FixedOffset>,

        #[serde(
            deserialize_with = "from_str_to_date",
            serialize_with = "from_date_to_str"
        )]
        pub end_date: Date<FixedOffset>,

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

/// Returns all planned road openings
///
/// Update freq: 24 hours – whenever there are updates
pub fn get_road_details(
    client: &LTAClient,
    road_details_type: road::RoadDetailsType,
) -> reqwest::Result<Vec<road::RoadDetails>> {
    let url = match road_details_type {
        road::RoadDetailsType::RoadOpening => road::URL_ROAD_OPENING,
        road::RoadDetailsType::RoadWorks => road::URL_ROAD_WORKS,
    };

    let resp: road::RoadDetailsResp = build_req(client, url)?;

    Ok(resp.value)
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
/// Update freq: 1 to 5 minutes
pub fn get_traffic_images(
    client: &LTAClient,
) -> reqwest::Result<Vec<traffic_images::TrafficImage>> {
    let resp: traffic_images::TrafficImageResp = build_req(client, traffic_images::URL)?;
    Ok(resp.value)
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
/// Update freq: 5 minutes
pub fn get_traffic_incidents(
    client: &LTAClient,
) -> reqwest::Result<Vec<traffic_incidents::TrafficIncident>> {
    let resp: traffic_incidents::TrafficIncidentResp = build_req(client, traffic_incidents::URL)?;
    Ok(resp.value)
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
/// Update freq: 5 minutes
pub fn get_traffic_speed_band(
    client: &LTAClient,
) -> reqwest::Result<Vec<traffic_speed_bands::TrafficSpeedBand>> {
    let resp: traffic_speed_bands::TrafficSpeedBandResp =
        build_req(client, traffic_speed_bands::URL)?;
    Ok(resp.value)
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
/// Update freq: 2 minutes
pub fn get_vms_emas(client: &LTAClient) -> reqwest::Result<Vec<vms_emas::VMS>> {
    let resp: vms_emas::VMSResp = build_req(client, vms_emas::URL)?;
    Ok(resp.value)
}

pub mod bike_parking {
    use serde::{Deserialize, Serialize};

    pub const URL: &str = "http://datamall2.mytransport.sg/ltaodataservice/BicycleParkingv2";

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct BikeParking {
        #[serde(rename = "Description")]
        pub desc: String,

        #[serde(rename = "Latitude")]
        pub lat: f64,

        #[serde(rename = "Longitude")]
        pub long: f64,

        pub rack_type: String,

        pub rack_count: u32,

        pub shelter_indicator: String,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct BikeParkingResp {
        pub value: Vec<BikeParking>,
    }
}

/// Returns bicycle parking locations within a radius. The default radius is
/// set as 0.5km
///
/// Update freq: Monthly
pub fn get_bike_parking(
    client: &LTAClient,
    lat: f64,
    long: f64,
    dist: f64,
) -> reqwest::Result<Vec<bike_parking::BikeParking>> {
    let resp: bike_parking::BikeParkingResp =
        build_res_with_query(client, bike_parking::URL, |rb| {
            rb.query(&[("Lat", lat), ("Long", long), ("Dist", dist)])
        })?;

    Ok(resp.value)
}
