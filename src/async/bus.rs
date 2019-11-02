//! All API pertaining to buses
use futures::Future;
use reqwest::Error;

use crate::bus::*;
use crate::r#async::lta_client::LTAClient;
use crate::utils::commons::{build_req_async, build_req_async_with_query};

/// Returns real-time Bus Arrival information of Bus Services at a queried Bus Stop,
/// including Est. Arrival Time, Est. Current Location, Est. Current Load.
///
/// Sometimes, it may return an empty Vec
///
/// If that happens, it means that there are no services at that timing.
///
/// **Update freq**: 1min
pub fn get_arrival<'a>(
    client: &LTAClient,
    bus_stop_code: u32,
    service_no: Option<&'a str>,
) -> impl Future<Item = bus_arrival::BusArrivalResp, Error = Error> + 'a {
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
}

/// Returns detailed service information for all buses currently in
/// operation, including: first stop, last stop, peak / offpeak frequency of
/// dispatch.
///
/// **Update freq**: Ad-Hoc
pub fn get_bus_services(
    client: &LTAClient,
) -> impl Future<Item = Vec<bus_services::BusService>, Error = Error> {
    build_req_async::<bus_services::BusServiceResp, _>(client, bus_services::URL)
}

/// Returns detailed route information for all services currently in operation,
/// including: all bus stops along each route, first/last bus timings for each stop
///
/// **Update freq**: Ad-Hoc
pub fn get_bus_routes(
    client: &LTAClient,
) -> impl Future<Item = Vec<bus_routes::BusRoute>, Error = Error> {
    build_req_async::<bus_routes::BusRouteResp, _>(client, bus_services::URL)
}

/// Returns detailed information for all bus stops currently being serviced by
/// buses, including: Bus Stop Code, location coordinates.
///
/// **Update freq**: Ad-Hoc
pub fn get_bus_stops(
    client: &LTAClient,
) -> impl Future<Item = Vec<bus_stops::BusStop>, Error = Error> {
    build_req_async::<bus_stops::BusStopsResp, _>(client, bus_stops::URL)
}
