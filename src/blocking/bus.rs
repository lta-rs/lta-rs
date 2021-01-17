use crate::blocking::{build_req_with_query, build_req_with_skip, Client, LTAClient};
use crate::models::bus::prelude::*;
use crate::LTAResult;
use crate::{api_url, Bus};

/// All API pertaining to buses
pub trait BusRequests<C: Client> {
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
    fn get_arrival(
        client: &C,
        bus_stop_code: u32,
        service_no: Option<&str>,
    ) -> LTAResult<BusArrivalResp>;

    /// Returns detailed service information for all buses currently in
    /// operation, including: first stop, last stop, peak / offpeak frequency of
    /// dispatch.
    ///
    /// **Update freq**: Ad-Hoc
    fn get_bus_services(client: &C, skip: Option<u32>) -> LTAResult<Vec<BusService>>;

    /// Returns detailed route information for all services currently in operation,
    /// including: all bus stops along each route, first/last bus timings for each stop
    ///
    /// **Update freq**: Ad-Hoc
    fn get_bus_routes(client: &C, skip: Option<u32>) -> LTAResult<Vec<BusRoute>>;

    /// Returns detailed information for all bus stops currently being serviced by
    /// buses, including: Bus Stop Code, location coordinates.
    ///
    /// **Update freq**: Ad-Hoc
    fn get_bus_stops(client: &C, skip: Option<u32>) -> LTAResult<Vec<BusStop>>;
}

impl BusRequests<LTAClient> for Bus {
    fn get_arrival(
        client: &LTAClient,
        bus_stop_code: u32,
        service_no: Option<&str>,
    ) -> LTAResult<BusArrivalResp> {
        let url = api_url!("/BusArrivalv2");
        match service_no {
            Some(srv_no) => build_req_with_query::<RawBusArrivalResp, _, _, _>(client, url, |rb| {
                rb.query(&[
                    ("BusStopCode", bus_stop_code.to_string().as_str()),
                    ("ServiceNo", srv_no),
                ])
            }),
            None => build_req_with_query::<RawBusArrivalResp, _, _, _>(client, url, |rb| {
                rb.query(&[("BusStopCode", bus_stop_code.to_string())])
            }),
        }
    }

    fn get_bus_services(client: &LTAClient, skip: Option<u32>) -> LTAResult<Vec<BusService>> {
        build_req_with_skip::<BusServiceResp, _, _>(client, api_url!("/BusServices"), skip)
    }

    fn get_bus_routes(client: &LTAClient, skip: Option<u32>) -> LTAResult<Vec<BusRoute>> {
        build_req_with_skip::<BusRouteResp, _, _>(client, api_url!("/BusRoutes"), skip)
    }

    fn get_bus_stops(client: &LTAClient, skip: Option<u32>) -> LTAResult<Vec<BusStop>> {
        build_req_with_skip::<BusStopsResp, _, _>(client, api_url!("/BusStops"), skip)
    }
}
