use lta::{Api, GetEvChargingPointsBatchResponse, GetEvChargingPointsResponse, PostalCode};

use crate::capture::{AttemptFailure, Capture, Captured, Ctx};
use crate::http::CaptureActionExt;

pub fn captures() -> Vec<Capture> {
    vec![
        ev_charging_points_batch(),
        ev_charging_points_available(),
        ev_charging_points_occupied(),
        ev_charging_points_unavailable(),
        ev_charging_points_empty(),
        ev_charging_points_free(),
    ]
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

async fn ev_charging_points_page(ctx: &Ctx, postal_code: &str) -> Result<Captured, AttemptFailure> {
    let postal_code = PostalCode::try_from(postal_code).expect("valid postal code");
    let (response, raw) = Api::new()
        .account_key(&ctx.account_key)
        .ev()
        .get_charging_points(postal_code)
        .capture_with(&ctx.client)
        .await?;

    match response {
        GetEvChargingPointsResponse::Ok(value) => Ok(Captured {
            records: value.ev_locations_data.len(),
            body: raw.body,
        }),
        GetEvChargingPointsResponse::UnexpectedStatus(status, _) => {
            Err(AttemptFailure::unexpected(status, &raw.headers))
        }
    }
}

async fn ev_available_page(ctx: &Ctx, _skip: u32) -> Result<Captured, AttemptFailure> {
    ev_charging_points_page(ctx, "650346").await
}

async fn ev_occupied_page(ctx: &Ctx, _skip: u32) -> Result<Captured, AttemptFailure> {
    ev_charging_points_page(ctx, "219428").await
}

async fn ev_unavailable_page(ctx: &Ctx, _skip: u32) -> Result<Captured, AttemptFailure> {
    ev_charging_points_page(ctx, "760607").await
}

async fn ev_empty_page(ctx: &Ctx, _skip: u32) -> Result<Captured, AttemptFailure> {
    ev_charging_points_page(ctx, "000000").await
}

async fn ev_free_page(ctx: &Ctx, _skip: u32) -> Result<Captured, AttemptFailure> {
    ev_charging_points_page(ctx, "159361").await
}

fn ev_charging_points_available() -> Capture {
    Capture {
        id: "ev_charging_points_available",
        dir: "ev_charging_points",
        stem: "ev_charging_points",
        paginated: false,
        fetch: |ctx, skip| Box::pin(ev_available_page(ctx, skip)),
    }
}

fn ev_charging_points_occupied() -> Capture {
    Capture {
        id: "ev_charging_points_occupied",
        dir: "ev_charging_points",
        stem: "ev_charging_points",
        paginated: false,
        fetch: |ctx, skip| Box::pin(ev_occupied_page(ctx, skip)),
    }
}

fn ev_charging_points_unavailable() -> Capture {
    Capture {
        id: "ev_charging_points_unavailable",
        dir: "ev_charging_points",
        stem: "ev_charging_points",
        paginated: false,
        fetch: |ctx, skip| Box::pin(ev_unavailable_page(ctx, skip)),
    }
}

fn ev_charging_points_empty() -> Capture {
    Capture {
        id: "ev_charging_points_empty",
        dir: "ev_charging_points",
        stem: "ev_charging_points",
        paginated: false,
        fetch: |ctx, skip| Box::pin(ev_empty_page(ctx, skip)),
    }
}

fn ev_charging_points_free() -> Capture {
    Capture {
        id: "ev_charging_points_free",
        dir: "ev_charging_points",
        stem: "ev_charging_points",
        paginated: false,
        fetch: |ctx, skip| Box::pin(ev_free_page(ctx, skip)),
    }
}
