use serde::{Deserialize, Serialize};

/// SBST -> SBS Transit
///
/// SMRT -> SMRT Corporation
///
/// TTS -> Tower Transit Singapore
///
/// GAS -> Go Ahead Singapore
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum Operator {
    SBST,
    SMRT,
    TTS,
    GAS,
}

/// SD -> Single Decker
///
/// DD -> Double Decker
///
/// BD -> Bendy
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum BusType {
    #[serde(rename = "SD")]
    SingleDecker,

    #[serde(rename = "DD")]
    DoubleDecker,

    #[serde(rename = "BD")]
    Bendy,
}

/// SEA -> Seats available
///
/// SDA -> Standing available
///
/// LSD -> Limited standing
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum BusLoad {
    #[serde(rename = "SEA")]
    SeatsAvailable,

    #[serde(rename = "SDA")]
    StandingAvailable,

    #[serde(rename = "LSD")]
    LimitedStanding,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum BusFeature {
    #[serde(rename = "WAB")]
    WheelChairAccessible,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum BusCategory {
    EXPRESS,

    FEEDER,

    INDUSTRIAL,

    TOWNLINK,

    TRUNK,

    #[serde(rename = "2-TIER FLAT FARE")]
    TWOTIERFLATFEE,

    FLATFEE,

    #[serde(rename = "NIGHT SERVICE")]
    NIGHTSERVICE,

    #[serde(rename = "CITY_LINK")]
    CITYLINK,

    #[serde(rename = "FLAT FARE $2.00")]
    FLATFARE2DOLLAR,
}
