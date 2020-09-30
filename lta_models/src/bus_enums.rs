//! Enums for buses and operators
//! Used for transforming stringly typed data from API to enums

use serde::{Deserialize, Serialize};

/// SBST -> SBS Transit
///
/// SMRT -> SMRT Corporation
///
/// TTS -> Tower Transit Singapore
///
/// GAS -> Go Ahead Singapore
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub enum Operator {
    SBST,
    SMRT,
    TTS,
    GAS,

    #[serde(other)]
    Unknown,
}

/// SD -> Single Decker
///
/// DD -> Double Decker
///
/// BD -> Bendy
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub enum BusType {
    #[serde(alias = "SD")]
    SingleDecker,

    #[serde(alias = "DD")]
    DoubleDecker,

    #[serde(alias = "BD")]
    Bendy,

    #[serde(other)]
    Unknown,
}

/// SEA -> Seats available
///
/// SDA -> Standing available
///
/// LSD -> Limited standing
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub enum BusLoad {
    #[serde(alias = "SEA")]
    SeatsAvailable,

    #[serde(alias = "SDA")]
    StandingAvailable,

    #[serde(alias = "LSD")]
    LimitedStanding,

    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub enum BusFeature {
    #[serde(alias = "WAB")]
    WheelChairAccessible,

    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum BusCategory {
    #[serde(alias = "EXPRESS")]
    Express,

    #[serde(alias = "FEEDER")]
    Feeder,

    #[serde(alias = "INDUSTRIAL")]
    Industrial,

    #[serde(alias = "TOWNLINK")]
    TownLink,

    #[serde(alias = "TRUNK")]
    Trunk,

    #[serde(alias = "2-TIER FLAT FARE")]
    TwoTierFlatFare,

    #[serde(alias = "FLATFEE")]
    FlatFee,

    #[serde(alias = "NIGHT SERVICE")]
    NightService,

    #[serde(alias = "CITY_LINK")]
    CityLink,

    #[serde(alias = "FLAT FARE $2.00")]
    FlatFareTwoDollar,

    #[serde(other)]
    Unknown,
}
