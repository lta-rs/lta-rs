use crate::blocking::{build_req_with_query, build_req_with_skip, LTAClient};
use crate::models::bus::prelude::*;
use crate::LTAResult;
use crate::{api_url, Bus, Client};
use crate::blocking::ClientExt;

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
    fn get_arrival<'a, S>(
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
    fn get_bus_services<S>(client: &C, skip: S) -> LTAResult<Vec<BusService>>
    where
        S: Into<Option<u32>>;

    /// Returns detailed route information for all services currently in operation,
    /// including: all bus stops along each route, first/last bus timings for each stop
    ///
    /// **Update freq**: Ad-Hoc
    fn get_bus_routes<S>(client: &C, skip: S) -> LTAResult<Vec<BusRoute>>
    where
        S: Into<Option<u32>>;

    /// Returns detailed information for all bus stops currently being serviced by
    /// buses, including: Bus Stop Code, location coordinates.
    ///
    /// **Update freq**: Ad-Hoc
    fn get_bus_stops<S>(client: &C, skip: S) -> LTAResult<Vec<BusStop>>
    where
        S: Into<Option<u32>>;
}

impl BusRequests<LTAClient> for Bus {
    fn get_arrival<'a, S>(
        client: &LTAClient,
        bus_stop_code: u32,
        service_no: S,
    ) -> LTAResult<BusArrivalResp>
    where
        S: Into<Option<&'a str>>,
    {
        let url = api_url!("/BusArrivalv2");
        match service_no.into() {
            Some(srv_no) => build_req_with_query::<RawBusArrivalResp, _, _, _>(client, url, |rb| {
                let srv_no = srv_no.as_ref();
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

    fn get_bus_services<S>(client: &LTAClient, skip: S) -> LTAResult<Vec<BusService>>
    where
        S: Into<Option<u32>>,
    {
        build_req_with_skip::<BusServiceResp, _, _>(client, api_url!("/BusServices"), skip.into())
    }

    fn get_bus_routes<S>(client: &LTAClient, skip: S) -> LTAResult<Vec<BusRoute>>
    where
        S: Into<Option<u32>>,
    {
        client.build_req_with_skip::<BusRouteResp, _>(api_url!("/BusRoutes"), skip.into())
        // build_req_with_skip::<BusRouteResp, _, _>(client, api_url!("/BusRoutes"), skip.into())
    }

    fn get_bus_stops<S>(client: &LTAClient, skip: S) -> LTAResult<Vec<BusStop>>
    where
        S: Into<Option<u32>>,
    {
        build_req_with_skip::<BusStopsResp, _, _>(client, api_url!("/BusStops"), skip.into())
    }
}