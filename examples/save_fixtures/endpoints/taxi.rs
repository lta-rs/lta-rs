use lta::{Api, GetTaxiAvailabilityResponse, GetTaxiStandsResponse};

use crate::capture::{AttemptFailure, Capture, Captured, Ctx};
use crate::http::CaptureActionExt;

pub fn captures() -> Vec<Capture> {
    vec![taxi_availability(), taxi_stands()]
}

async fn taxi_availability_page(ctx: &Ctx, skip: u32) -> Result<Captured, AttemptFailure> {
    let (response, raw) = Api::new()
        .account_key(&ctx.account_key)
        .taxi()
        .get_availability()
        .skip(skip)
        .capture_with(&ctx.client)
        .await?;

    match response {
        GetTaxiAvailabilityResponse::Ok(coordinates) => Ok(Captured {
            records: coordinates.len(),
            body: raw.body,
        }),
        GetTaxiAvailabilityResponse::UnexpectedStatus(status, _) => {
            Err(AttemptFailure::unexpected(status, &raw.headers))
        }
    }
}

async fn taxi_stands_page(ctx: &Ctx, skip: u32) -> Result<Captured, AttemptFailure> {
    let (response, raw) = Api::new()
        .account_key(&ctx.account_key)
        .taxi()
        .get_stands()
        .skip(skip)
        .capture_with(&ctx.client)
        .await?;
    match response {
        GetTaxiStandsResponse::Ok(stands) => Ok(Captured {
            records: stands.len(),
            body: raw.body,
        }),
        GetTaxiStandsResponse::UnexpectedStatus(status, _) => {
            Err(AttemptFailure::unexpected(status, &raw.headers))
        }
    }
}

fn taxi_availability() -> Capture {
    Capture {
        id: "taxi_availability",
        dir: "taxi_availability",
        stem: "taxi_availability",
        paginated: true,
        fetch: |ctx, skip| Box::pin(taxi_availability_page(ctx, skip)),
    }
}

fn taxi_stands() -> Capture {
    Capture {
        id: "taxi_stands",
        dir: "taxi_stands",
        stem: "taxi_stands",
        paginated: true,
        fetch: |ctx, skip| Box::pin(taxi_stands_page(ctx, skip)),
    }
}
