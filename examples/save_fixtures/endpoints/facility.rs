use lta::{Api, GetFacilitiesMaintenanceResponse};

use crate::capture::{AttemptFailure, Capture, Captured, Ctx};
use crate::http::CaptureActionExt;

pub fn captures() -> Vec<Capture> {
    vec![facilities_maintenance()]
}

async fn facilities_maintenance_page(ctx: &Ctx, skip: u32) -> Result<Captured, AttemptFailure> {
    let (response, raw) = Api::new()
        .account_key(&ctx.account_key)
        .facility()
        .get_facilities_maintenance()
        .skip(skip)
        .capture_with(&ctx.client)
        .await?;
    match response {
        GetFacilitiesMaintenanceResponse::Ok(records) => Ok(Captured {
            records: records.len(),
            body: raw.body,
        }),
        GetFacilitiesMaintenanceResponse::UnexpectedStatus(status, _) => {
            Err(AttemptFailure::unexpected(status, &raw.headers))
        }
    }
}

fn facilities_maintenance() -> Capture {
    Capture {
        id: "facilities_maintenance",
        dir: "facilities_maintenance",
        stem: "facilities_maintenance",
        paginated: true,
        fetch: Box::new(|ctx, skip| Box::pin(facilities_maintenance_page(ctx, skip))),
    }
}
