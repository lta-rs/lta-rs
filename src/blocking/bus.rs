use crate::blocking::ClientExt;
use crate::models::bus::prelude::*;
use crate::LTAResult;
use crate::{api_url, Client};

/// All API pertaining to buses
pub trait BusRequests<C: Client + ClientExt> {
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
    fn get_arrival<'a>(
        client: &C,
        bus_stop_code: u32,
        service_no: impl Into<Option<&'a str>>,
    ) -> LTAResult<BusArrivalResp>;

    /// Returns detailed service information for all buses currently in
    /// operation, including: first stop, last stop, peak / offpeak frequency of
    /// dispatch.
    ///
    /// **Update freq**: Ad-Hoc
    fn get_bus_services(client: &C, skip: impl Into<Option<u32>>) -> LTAResult<Vec<BusService>> {
        client.build_req_with_skip::<BusServiceResp, _>(api_url!("/BusServices"), skip.into())
    }

    /// Returns detailed route information for all services currently in operation,
    /// including: all bus stops along each route, first/last bus timings for each stop
    ///
    /// **Update freq**: Ad-Hoc
    fn get_bus_routes(client: &C, skip: impl Into<Option<u32>>) -> LTAResult<Vec<BusRoute>> {
        client.build_req_with_skip::<BusRouteResp, _>(api_url!("/BusRoutes"), skip.into())
    }

    /// Returns detailed information for all bus stops currently being serviced by
    /// buses, including: Bus Stop Code, location coordinates.
    ///
    /// **Update freq**: Ad-Hoc
    fn get_bus_stops(client: &C, skip: impl Into<Option<u32>>) -> LTAResult<Vec<BusStop>> {
        client.build_req_with_skip::<BusStopsResp, _>(api_url!("/BusStops"), skip.into())
    }
}


