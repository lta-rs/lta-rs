use lta::{Api, GetEvChargingPointsBatchResponse};

use crate::capture::{AttemptFailure, Capture, Captured, Ctx};
use crate::http::CaptureActionExt;

pub fn captures() -> Vec<Capture> {
    vec![ev_charging_points_batch()]
}

async fn ev_charging_points_batch_page(ctx: &Ctx, _skip: u32) -> Result<Captured, AttemptFailure> {
    let (response, raw) = Api::new()
        .account_key(&ctx.account_key)
        .ev()
        .get_charging_points_batch()
        .capture_with(&ctx.client)
        .await?;

    match response {
        GetEvChargingPointsBatchResponse::Ok(links) => Ok(Captured {
            records: links.len(),
            body: raw.body,
        }),
        GetEvChargingPointsBatchResponse::UnexpectedStatus(status, _) => {
            Err(AttemptFailure::unexpected(status, &raw.headers))
        }
    }
}

fn ev_charging_points_batch() -> Capture {
    Capture {
        id: "ev_charging_points_batch",
        dir: "ev_charging_points_batch",
        stem: "ev_charging_points_batch",
        paginated: false,
        fetch: |ctx, skip| Box::pin(ev_charging_points_batch_page(ctx, skip)),
    }
}
