//! All APIs pertaining to buses

use crate::async_utils::{build_req_async, build_req_async_with_query};
use crate::lta_client::LTAClient;
use lta_models::bus::{bus_arrival, bus_routes, bus_services, bus_stops};
use lta_utils_commons::LTAError;

/// Returns real-time Bus Arrival information of Bus Services at a queried Bus Stop,
/// including Est. Arrival Time, Est. Current Location, Est. Current Load.
///
/// Sometimes, it may return an empty Vec
///
/// If that happens, it means that there are no services at that timing.
///
/// **Update freq**: 1min
pub async fn get_arrival(
    client: &LTAClient,
    bus_stop_code: u32,
    service_no: Option<&str>,
) -> Result<bus_arrival::BusArrivalResp, LTAError> {
    build_req_async_with_query::<bus_arrival::RawBusArrivalResp, _, _>(
        client,
        bus_arrival::URL,
        move |rb| match service_no {
            Some(srv_no) => rb.query(&[
                ("BusStopCode", bus_stop_code.to_string()),
                ("ServiceNo", srv_no.to_string()),
            ]),
            None => rb.query(&[("BusStopCode", bus_stop_code.to_string())]),
        },
    )
    .await
}

/// Returns detailed service information for all buses currently in
/// operation, including: first stop, last stop, peak / offpeak frequency of
/// dispatch.
///
/// **Update freq**: Ad-Hoc
pub async fn get_bus_services(
    client: &LTAClient,
) -> Result<Vec<bus_services::BusService>, LTAError> {
    build_req_async::<bus_services::BusServiceResp, _>(client, bus_services::URL).await
}

/// Returns detailed route information for all services currently in operation,
/// including: all bus stops along each route, first/last bus timings for each stop
///
/// **Update freq**: Ad-Hoc
pub async fn get_bus_routes(client: &LTAClient) -> Result<Vec<bus_routes::BusRoute>, LTAError> {
    build_req_async::<bus_routes::BusRouteResp, _>(client, bus_services::URL).await
}

/// Returns detailed information for all bus stops currently being serviced by
/// buses, including: Bus Stop Code, location coordinates.
///
/// **Update freq**: Ad-Hoc
pub async fn get_bus_stops(client: &LTAClient) -> Result<Vec<bus_stops::BusStop>, LTAError> {
    build_req_async::<bus_stops::BusStopsResp, _>(client, bus_stops::URL).await
}
