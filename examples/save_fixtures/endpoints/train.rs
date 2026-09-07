use lta::{
    Api, GetGtfsRealTimeTrainServiceAlertsResponse, GetGtfsRealtimeTrainTripUpdatesResponse,
    GetGtfsScheduleTrainResponse,
};

use crate::capture::{AttemptFailure, Capture, Captured, Ctx};
use crate::http::CaptureActionExt;

pub fn captures() -> Vec<Capture> {
    vec![
        gtfs_schedule_train(),
        gtfs_real_time_train_service_alerts(),
        gtfs_realtime_train_trip_updates(),
    ]
}

/// Requests a page and returns the decoded record count next to the untouched wire body.
macro_rules! page {
    ($ctx:expr, $chain:expr, $response:ident) => {{
        let (response, raw) = $chain.capture_with(&$ctx.client).await?;
        match response {
            $response::Ok(records) => Ok(Captured {
                records: records.len(),
                body: raw.body,
            }),
            $response::UnexpectedStatus(status, _) => {
                Err(AttemptFailure::unexpected(status, &raw.headers))
            }
        }
    }};
}

async fn gtfs_schedule_train_page(ctx: &Ctx, _skip: u32) -> Result<Captured, AttemptFailure> {
    page!(
        ctx,
        Api::new()
            .account_key(&ctx.account_key)
            .train()
            .get_gtfs_schedule(),
        GetGtfsScheduleTrainResponse
    )
}

fn gtfs_schedule_train() -> Capture {
    Capture {
        id: "gtfs_schedule_train",
        dir: "gtfs_schedule_train",
        stem: "gtfs_schedule_train",
        paginated: false,
        fetch: |ctx, skip| Box::pin(gtfs_schedule_train_page(ctx, skip)),
    }
}

async fn gtfs_real_time_train_service_alerts_page(
    ctx: &Ctx,
    _skip: u32,
) -> Result<Captured, AttemptFailure> {
    page!(
        ctx,
        Api::new()
            .account_key(&ctx.account_key)
            .train()
            .get_gtfs_real_time_service_alerts(),
        GetGtfsRealTimeTrainServiceAlertsResponse
    )
}

fn gtfs_real_time_train_service_alerts() -> Capture {
    Capture {
        id: "gtfs_real_time_train_service_alerts",
        dir: "gtfs_real_time_train_service_alerts",
        stem: "gtfs_real_time_train_service_alerts",
        paginated: false,
        fetch: |ctx, skip| Box::pin(gtfs_real_time_train_service_alerts_page(ctx, skip)),
    }
}

async fn gtfs_realtime_train_trip_updates_page(
    ctx: &Ctx,
    _skip: u32,
) -> Result<Captured, AttemptFailure> {
    page!(
        ctx,
        Api::new()
            .account_key(&ctx.account_key)
            .train()
            .get_gtfs_realtime_trip_updates(),
        GetGtfsRealtimeTrainTripUpdatesResponse
    )
}

fn gtfs_realtime_train_trip_updates() -> Capture {
    Capture {
        id: "gtfs_realtime_train_trip_updates",
        dir: "gtfs_realtime_train_trip_updates",
        stem: "gtfs_realtime_train_trip_updates",
        paginated: false,
        fetch: |ctx, skip| Box::pin(gtfs_realtime_train_trip_updates_page(ctx, skip)),
    }
}
