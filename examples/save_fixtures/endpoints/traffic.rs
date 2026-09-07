use lta::{
    Api, GetBikeParkingResponse, GetFloodAlertsResponse, GetRoadOpeningsResponse,
    GetRoadWorksResponse, GetTrafficFlowResponse, GetTrafficImagesResponse,
    GetTrafficIncidentsResponse, GetTrafficSpeedBandsResponse, GetVariableMessageSignsResponse,
    Latitude, Longitude,
};

use crate::capture::{AttemptFailure, Capture, Captured, Ctx};
use crate::http::CaptureActionExt;

pub fn captures() -> Vec<Capture> {
    vec![
        traffic_incidents(),
        vms(),
        traffic_images(),
        traffic_speed_bands(),
        traffic_flow(),
        bike_parking(),
        road_works(),
        road_openings(),
        flood_alerts(),
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

async fn traffic_incidents_page(ctx: &Ctx, skip: u32) -> Result<Captured, AttemptFailure> {
    page!(
        ctx,
        Api::new()
            .account_key(&ctx.account_key)
            .traffic()
            .get_incidents()
            .skip(skip),
        GetTrafficIncidentsResponse
    )
}

async fn vms_page(ctx: &Ctx, skip: u32) -> Result<Captured, AttemptFailure> {
    page!(
        ctx,
        Api::new()
            .account_key(&ctx.account_key)
            .traffic()
            .get_variable_message_signs()
            .skip(skip),
        GetVariableMessageSignsResponse
    )
}

async fn traffic_images_page(ctx: &Ctx, skip: u32) -> Result<Captured, AttemptFailure> {
    page!(
        ctx,
        Api::new()
            .account_key(&ctx.account_key)
            .traffic()
            .get_images()
            .skip(skip),
        GetTrafficImagesResponse
    )
}

async fn traffic_speed_bands_page(ctx: &Ctx, skip: u32) -> Result<Captured, AttemptFailure> {
    page!(
        ctx,
        Api::new()
            .account_key(&ctx.account_key)
            .traffic()
            .get_speed_bands()
            .skip(skip),
        GetTrafficSpeedBandsResponse
    )
}

async fn traffic_flow_page(ctx: &Ctx, _skip: u32) -> Result<Captured, AttemptFailure> {
    page!(
        ctx,
        Api::new()
            .account_key(&ctx.account_key)
            .traffic()
            .get_flow(),
        GetTrafficFlowResponse
    )
}

async fn bike_parking_page(ctx: &Ctx, _skip: u32) -> Result<Captured, AttemptFailure> {
    let lat = Latitude::try_from(1.3521).expect("valid latitude");
    let long = Longitude::try_from(103.8198).expect("valid longitude");
    page!(
        ctx,
        Api::new()
            .account_key(&ctx.account_key)
            .traffic()
            .get_bike_parking(lat, long),
        GetBikeParkingResponse
    )
}

async fn road_works_page(ctx: &Ctx, skip: u32) -> Result<Captured, AttemptFailure> {
    page!(
        ctx,
        Api::new()
            .account_key(&ctx.account_key)
            .traffic()
            .get_road_works()
            .skip(skip),
        GetRoadWorksResponse
    )
}

async fn road_openings_page(ctx: &Ctx, skip: u32) -> Result<Captured, AttemptFailure> {
    page!(
        ctx,
        Api::new()
            .account_key(&ctx.account_key)
            .traffic()
            .get_road_openings()
            .skip(skip),
        GetRoadOpeningsResponse
    )
}

fn traffic_incidents() -> Capture {
    Capture {
        id: "traffic_incidents",
        dir: "traffic_incidents",
        stem: "traffic_incidents",
        paginated: true,
        fetch: |ctx, skip| Box::pin(traffic_incidents_page(ctx, skip)),
    }
}

/// Legacy fixture directory name: the tests glob `vms_emas`, not `variable_message_signs`.
fn vms() -> Capture {
    Capture {
        id: "vms",
        dir: "vms_emas",
        stem: "vms_emas",
        paginated: true,
        fetch: |ctx, skip| Box::pin(vms_page(ctx, skip)),
    }
}

fn traffic_images() -> Capture {
    Capture {
        id: "traffic_images",
        dir: "traffic_images",
        stem: "traffic_images",
        paginated: true,
        fetch: |ctx, skip| Box::pin(traffic_images_page(ctx, skip)),
    }
}

fn traffic_speed_bands() -> Capture {
    Capture {
        id: "traffic_speed_bands",
        dir: "traffic_speed_bands",
        stem: "traffic_speed_bands",
        paginated: true,
        fetch: |ctx, skip| Box::pin(traffic_speed_bands_page(ctx, skip)),
    }
}

fn traffic_flow() -> Capture {
    Capture {
        id: "traffic_flow",
        dir: "traffic_flow",
        stem: "traffic_flow",
        paginated: false,
        fetch: |ctx, skip| Box::pin(traffic_flow_page(ctx, skip)),
    }
}

fn bike_parking() -> Capture {
    Capture {
        id: "bike_parking",
        dir: "bike_parking",
        stem: "bike_parking",
        paginated: false,
        fetch: |ctx, skip| Box::pin(bike_parking_page(ctx, skip)),
    }
}

fn road_works() -> Capture {
    Capture {
        id: "road_works",
        dir: "road_works",
        stem: "road_works",
        paginated: true,
        fetch: |ctx, skip| Box::pin(road_works_page(ctx, skip)),
    }
}

fn road_openings() -> Capture {
    Capture {
        id: "road_openings",
        dir: "road_openings",
        stem: "road_openings",
        paginated: true,
        fetch: |ctx, skip| Box::pin(road_openings_page(ctx, skip)),
    }
}

async fn flood_alerts_page(ctx: &Ctx, _skip: u32) -> Result<Captured, AttemptFailure> {
    page!(
        ctx,
        Api::new()
            .account_key(&ctx.account_key)
            .traffic()
            .get_flood_alerts(),
        GetFloodAlertsResponse
    )
}

fn flood_alerts() -> Capture {
    Capture {
        id: "flood_alerts",
        dir: "flood_alerts",
        stem: "flood_alerts",
        paginated: false,
        fetch: |ctx, skip| Box::pin(flood_alerts_page(ctx, skip)),
    }
}
