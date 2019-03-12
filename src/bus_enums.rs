use serde::Deserialize;

/// SBST -> SBS Transit
///
/// SMRT -> SMRT Corporation
///
/// TTS -> Tower Transit Singapore
///
/// GAS -> Go Ahead Singapore
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum BusType {
    SD,
    DD,
    BD,
}

/// SEA -> Seats available
///
/// SDA -> Standing available
///
/// LSD -> Limited standing
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum BusLoad {
    SEA,
    SDA,
    LSD,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum BusFeature {
    WAB
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub enum BusCategory {
    EXPRESS,

    FEEDER,

    INDUSTRIAL,

    TOWNLINK,

    TRUNK,

    TWOTIERFLATFEE,

    FLATFEE,

    #[serde(rename = "NIGHT SERVICE")]
    NIGHTSERVICE,

    #[serde(rename = "CITY_LINK")]
    CITYLINK,

    #[serde(rename = "FLAT FARE $2.00")]
    FLATFARE2DOLLAR,
}
