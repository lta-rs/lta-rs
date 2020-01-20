//! All API pertaining to buses

use crate::lta_client::LTAClient;
use crate::{build_req_with_query, build_req_with_skip};
use lta_models::bus::{bus_arrival, bus_routes, bus_services, bus_stops};
use lta_utils_commons::LTAResult;

/// Returns real-time Bus Arrival information of Bus Services at a queried Bus Stop,
/// including
/// - Estimated Arrival Time
/// - Estimated Current Location
/// - Estimated Current Load.
///
/// Sometimes, it may return an empty Vec
///
/// If that happens, it means that there are no services at that timing.
///
/// **Update freq**: 1min
///
pub fn get_arrival(
    client: &LTAClient,
    bus_stop_code: u32,
    service_no: Option<&str>,
) -> LTAResult<bus_arrival::BusArrivalResp> {
    match service_no {
        Some(srv_no) => build_req_with_query::<bus_arrival::RawBusArrivalResp, _, _>(
            client,
            bus_arrival::URL,
            |rb| {
                rb.query(&[
                    ("BusStopCode", bus_stop_code.to_string()),
                    ("ServiceNo", srv_no.to_string()),
                ])
            },
        ),
        None => build_req_with_query::<bus_arrival::RawBusArrivalResp, _, _>(
            client,
            bus_arrival::URL,
            |rb| rb.query(&[("BusStopCode", bus_stop_code.to_string())]),
        ),
    }
}

/// Returns detailed service information for all buses currently in
/// operation, including: first stop, last stop, peak / offpeak frequency of
/// dispatch.
///
/// **Update freq**: Ad-Hoc
///
pub fn get_bus_services(
    client: &LTAClient,
    skip: Option<u32>,
) -> LTAResult<Vec<bus_services::BusService>> {
    build_req_with_skip::<bus_services::BusServiceResp, _>(client, bus_services::URL, skip)
}

/// Returns detailed route information for all services currently in operation,
/// including: all bus stops along each route, first/last bus timings for each stop
///
/// **Update freq**: Ad-Hoc
///
pub fn get_bus_routes(
    client: &LTAClient,
    skip: Option<u32>,
) -> LTAResult<Vec<bus_routes::BusRoute>> {
    build_req_with_skip::<bus_routes::BusRouteResp, _>(client, bus_routes::URL, skip)
}

/// Returns detailed information for all bus stops currently being serviced by
/// buses, including: Bus Stop Code, location coordinates.
///
/// **Update freq**: Ad-Hoc
///
pub fn get_bus_stops(client: &LTAClient, skip: Option<u32>) -> LTAResult<Vec<bus_stops::BusStop>> {
    build_req_with_skip::<bus_stops::BusStopsResp, _>(client, bus_stops::URL, skip)
}
