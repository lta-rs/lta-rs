pub use train_service_alert::{
    AffectedSegment, MrtLine, TrainServiceAlert, TrainServiceAlertMessage, TrainServiceAlertResp,
    TrainStatus,
};

pub mod train_service_alert {
    use serde::{Deserialize, Serialize};
    use serde_repr::*;

    use lta_utils_commons::de::dash_separated;

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

    #[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
    #[repr(u32)]
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
