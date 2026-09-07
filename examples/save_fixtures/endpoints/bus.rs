use lta::{Api, BusStopCode, GetBusArrivalResponse, GetBusStopsResponse};

use crate::capture::{AttemptFailure, Capture, Captured, Ctx};
use crate::http::CaptureActionExt;

pub fn captures() -> Vec<Capture> {
    vec![bus_arrival(), bus_stops()]
}

async fn bus_arrival_page(ctx: &Ctx, _skip: u32) -> Result<Captured, AttemptFailure> {
    let bus_stop_code = BusStopCode::try_from("01012").expect("valid bus stop code");
    let (response, raw) = Api::new()
        .account_key(&ctx.account_key)
        .bus()
        .get_arrival(bus_stop_code)
        .capture_with(&ctx.client)
        .await?;

    match response {
        GetBusArrivalResponse::Ok(_) => Ok(Captured {
            records: 1,
            body: raw.body,
        }),
        GetBusArrivalResponse::UnexpectedStatus(status, _) => {
            Err(AttemptFailure::unexpected(status, &raw.headers))
        }
    }
}

async fn bus_stops_page(ctx: &Ctx, skip: u32) -> Result<Captured, AttemptFailure> {
    let (response, raw) = Api::new()
        .account_key(&ctx.account_key)
        .bus()
        .get_stops()
        .skip(skip)
        .capture_with(&ctx.client)
        .await?;

    match response {
        GetBusStopsResponse::Ok(stops) => Ok(Captured {
            records: stops.len(),
            body: raw.body,
        }),
        GetBusStopsResponse::UnexpectedStatus(status, _) => {
            Err(AttemptFailure::unexpected(status, &raw.headers))
        }
    }
}

fn bus_arrival() -> Capture {
    Capture {
        id: "bus_arrival",
        dir: "bus_arrival",
        stem: "bus_arrival",
        paginated: false,
        fetch: |ctx, skip| Box::pin(bus_arrival_page(ctx, skip)),
    }
}

fn bus_stops() -> Capture {
    Capture {
        id: "bus_stops",
        dir: "bus_stops",
        stem: "bus_stops",
        paginated: true,
        fetch: |ctx, skip| Box::pin(bus_stops_page(ctx, skip)),
    }
}
