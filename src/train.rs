//! All API pertaining to train related data

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
        Normal = 1,
        Disrupted = 2,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all(deserialize = "PascalCase"))]
    pub struct AffectedSegment {
        pub line: MrtLine,

        pub direction: String,

        #[serde(deserialize_with = "dash_separated")]
        pub stations: Vec<String>,

        pub free_public_bus: String,

        pub free_mrt_shuttle: String,

        #[serde(alias = "MRTShuttleDirection")]
        pub mrt_shuttle_dir: String,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all(deserialize = "PascalCase"))]
    pub struct TrainServiceAlertMessage {
        pub content: String,
        pub created_date: String,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all(deserialize = "PascalCase"))]
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

    impl Into<TrainServiceAlert> for TrainServiceAlertResp {
        fn into(self) -> TrainServiceAlert {
            self.value
        }
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
/// fn main() -> lta::Result<()> {
///     let api_key = std::env::var("API_KEY").unwrap();
///     let client = LTAClient::with_api_key(api_key);
///     let train_svc_alert: TrainServiceAlert = get_train_service_alert(&client)?;
///     println!("{:?}", train_svc_alert);
///     Ok(())
/// }
/// ```
pub fn get_train_service_alert(
    client: &LTAClient,
) -> Result<train_service_alert::TrainServiceAlert> {
    build_req::<train_service_alert::TrainServiceAlertResp, _>(client, train_service_alert::URL)
}
