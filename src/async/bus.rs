use crate::models::bus::prelude::*;
use crate::{Client, LTAResult};
use concat_string::concat_string;

use super::ClientExt;

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
    async fn get_arrival<'a, S>(
        client: &C,
        bus_stop_code: u32,
        service_no: S,
    ) -> LTAResult<BusArrivalResp>
    where
        S: Into<Option<&'a str>>;

    /// Returns detailed service information for all buses currently in
    /// operation, including: first stop, last stop, peak / offpeak frequency of
    /// dispatch.
    ///
    /// **Update freq**: Ad-Hoc
    async fn get_bus_services<S>(client: &C, skip: S) -> LTAResult<Vec<BusService>>
    where
        S: Into<Option<u32>>,
    {
        let url = concat_string!(client.base_url(), "/BusServices");
        client
            .build_req_with_skip::<BusServiceResp, _>(url.as_str(), skip.into())
            .await
    }

    /// Returns detailed route information for all services currently in operation,
    /// including: all bus stops along each route, first/last bus timings for each stop
    ///
    /// **Update freq**: Ad-Hoc
    async fn get_bus_routes<S>(client: &C, skip: S) -> LTAResult<Vec<BusRoute>>
    where
        S: Into<Option<u32>>,
    {
        let url = concat_string!(client.base_url(), "/BusRoutes");
        client
            .build_req_with_skip::<BusRouteResp, _>(url.as_str(), skip.into())
            .await
    }

    /// Returns detailed information for all bus stops currently being serviced by
    /// buses, including: Bus Stop Code, location coordinates.
    ///
    /// **Update freq**: Ad-Hoc
    async fn get_bus_stops<S>(client: &C, skip: S) -> LTAResult<Vec<BusStop>>
    where
        S: Into<Option<u32>>,
    {
        let url = concat_string!(client.base_url(), "/BusStops");
        client
            .build_req_with_skip::<BusStopsResp, _>(url.as_str(), skip.into())
            .await
    }
}
