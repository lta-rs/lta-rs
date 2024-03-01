use crate::models::bus::prelude::*;
use crate::{Client, LTAResult};
use async_trait::async_trait;

use super::ClientExt;

/// All API pertaining to buses
#[async_trait]
pub trait BusRequests<C: Client + ClientExt + Send + Sync> {
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
        S: Into<Option<&'a str>> + Send;

    /// Returns detailed service information for all buses currently in
    /// operation, including: first stop, last stop, peak / offpeak frequency of
    /// dispatch.
    ///
    /// **Update freq**: Ad-Hoc
    async fn get_bus_services<S>(client: &C, skip: S) -> LTAResult<Vec<BusService>>
    where
        S: Into<Option<u32>> + Send,
    {
        client
            .build_req_with_skip::<BusServiceResp, _>(api_url!("/BusServices"), skip.into())
            .await
    }
    /// Returns detailed route information for all services currently in operation,
    /// including: all bus stops along each route, first/last bus timings for each stop
    ///
    /// **Update freq**: Ad-Hoc
    async fn get_bus_routes<S>(client: &C, skip: S) -> LTAResult<Vec<BusRoute>>
    where
        S: Into<Option<u32>> + Send,
    {
        client
            .build_req_with_skip::<BusRouteResp, _>(api_url!("/BusRoutes"), skip.into())
            .await
    }

    /// Returns detailed information for all bus stops currently being serviced by
    /// buses, including: Bus Stop Code, location coordinates.
    ///
    /// **Update freq**: Ad-Hoc
    async fn get_bus_stops<S>(client: &C, skip: S) -> LTAResult<Vec<BusStop>>
    where
        S: Into<Option<u32>> + Send,
    {
        client
            .build_req_with_skip::<BusStopsResp, _>(api_url!("/BusStops"), skip.into())
            .await
    }
}
