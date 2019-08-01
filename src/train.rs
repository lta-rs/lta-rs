use crate::lta_client::LTAClient;
use crate::utils::commons::{build_req, Result};

pub mod train_service_alert {
    use serde::{Deserialize, Serialize};

    use crate::utils::de::{dash_separated, from_int_to_mrt_status};

    pub const URL: &str = "http://datamall2.mytransport.sg/ltaodataservice/TrainServiceAlerts";

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
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

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub enum TrainStatus {
        Normal,
        Disrupted,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct AffectedSegment {
        pub line: MrtLine,

        pub direction: String,

        #[serde(deserialize_with = "dash_separated")]
        pub stations: Vec<String>,

        pub free_public_bus: String,

        pub free_mrt_shuttle: String,

        #[serde(rename = "MRTShuttleDirection")]
        pub mrt_shuttle_dir: String,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct TrainServiceAlertMessage {
        pub content: String,
        pub created_date: String,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct TrainServiceAlert {
        #[serde(deserialize_with = "from_int_to_mrt_status")]
        pub status: TrainStatus,

        pub affected_segments: Vec<AffectedSegment>,

        pub message: Vec<TrainServiceAlertMessage>,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct TrainServiceAlertResp {
        pub value: TrainServiceAlert,
    }
}

/// Returns detailed information on train service unavailability during scheduled
/// operating hours, such as affected line and stations etc.
///
/// **Update freq**: ad-hoc
/// ## Example
/// ```rust
/// use lta::prelude::*;
/// use lta::train::get_train_service_alert;
/// use lta::lta_client::LTAClient;
///
/// fn main() -> Result<()> {
///     let client = LTAClient::with_api_key("api_key");
///     let train_svc_alert: TrainServiceAlert = get_train_service_alert(&client)?;
///     println!("{:?}", train_svc_alert);
///     Ok(())
/// }
/// ```
pub fn get_train_service_alert(
    client: &LTAClient,
) -> Result<train_service_alert::TrainServiceAlert> {
    let resp: train_service_alert::TrainServiceAlertResp =
        build_req(client, train_service_alert::URL)?;

    Ok(resp.value)
}
