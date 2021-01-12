use crate::api_url;
use crate::blocking::{build_req_with_query, Client, LTAClient, build_req_with_skip};
use crate::models::bus::prelude::*;
use crate::LTAResult;

pub trait BusRequests<C: Client> {
    fn get_arrival(
        client: &C,
        bus_stop_code: u32,
        service_no: Option<&str>,
    ) -> LTAResult<BusArrivalResp>;
    fn get_bus_services(client: &C, skip: Option<u32>) -> LTAResult<Vec<BusService>>;
    fn get_bus_routes(client: &C, skip: Option<u32>) -> LTAResult<Vec<BusRoute>>;
    /// Returns detailed information for all bus stops currently being serviced by
    /// buses, including: Bus Stop Code, location coordinates.
    ///
    /// **Update freq**: Ad-Hoc
    ///
    fn get_bus_stops(client: &C, skip: Option<u32>) -> LTAResult<Vec<BusStop>>;
}

#[derive(Debug, Copy, Clone)]
pub struct Bus;

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
