use crate::client_config::CLIENT_CONFIG;
use crate::utils::commons::build_res;

pub mod train_service_alert {
    use serde::Deserialize;

    use crate::utils::de::{dash_separated, from_int_to_mrt_status};

    pub const URL: &'static str =
        "http://datamall2.mytransport.sg/ltaodataservice/TrainServiceAlerts";

    #[derive(Debug, Clone, PartialEq, Deserialize)]
    pub enum MrtLine {
        CCL,
        CEL,
        CGL,
        DTL,
        EWL,
        NEL,
        NSL,
        PEL,
        PWL,
        SEL,
        SWL,
        BPL,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize)]
    pub enum TrainStatus {
        Normal,
        Disrupted,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct AffectedSegment {
        pub line: MrtLine,

        pub direction: String,

        #[serde(deserialize_with = "dash_separated")]
        pub stations: Vec<String>,

        pub free_public_bus: String,

        pub free_mrt_shuttle: String,

        #[serde(rename = "MRTShuttleDirection")]
        pub mrt_shuttle_direction: String,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct TrainServiceAlertMessage {
        pub content: String,
        pub created_date: String,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct TrainServiceAlert {
        #[serde(deserialize_with = "from_int_to_mrt_status")]
        pub status: TrainStatus,

        pub affected_segments: Vec<AffectedSegment>,

        pub message: Vec<TrainServiceAlertMessage>,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize)]
    pub struct TrainServiceAlertResp {
        pub value: TrainServiceAlert,
    }
}

/// Returns detailed information on train service unavailability during scheduled
/// operating hours, such as affected line and stations etc.
///
/// Update Freq: ad-hoc
pub fn get_train_service_alert() -> reqwest::Result<train_service_alert::TrainServiceAlert> {
    let resp: train_service_alert::TrainServiceAlertResp = build_res(train_service_alert::URL)?;

    Ok(resp.value)
}
