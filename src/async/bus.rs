use crate::models::bus::prelude::*;
use crate::r#async::build_req_with_query;
use crate::r#async::build_req_with_skip;
use crate::r#async::client::LTAClient;
use crate::{Bus, Client, LTAResult};
use async_trait::async_trait;

/// All API pertaining to buses
#[async_trait]
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
    async fn get_arrival<'a, S, A>(
        client: &C,
        bus_stop_code: u32,
        service_no: S,
    ) -> LTAResult<BusArrivalResp>
    where
        S: Into<Option<A>> + Send,
        A: AsRef<str> + Send;

    /// Returns detailed service information for all buses currently in
    /// operation, including: first stop, last stop, peak / offpeak frequency of
    /// dispatch.
    ///
    /// **Update freq**: Ad-Hoc
    async fn get_bus_services<S>(client: &C, skip: S) -> LTAResult<Vec<BusService>>
    where
        S: Into<Option<u32>> + Send;

    /// Returns detailed route information for all services currently in operation,
    /// including: all bus stops along each route, first/last bus timings for each stop
    ///
    /// **Update freq**: Ad-Hoc
    async fn get_bus_routes<S>(client: &C, skip: S) -> LTAResult<Vec<BusRoute>>
    where
        S: Into<Option<u32>> + Send;

    /// Returns detailed information for all bus stops currently being serviced by
    /// buses, including: Bus Stop Code, location coordinates.
    ///
    /// **Update freq**: Ad-Hoc
    async fn get_bus_stops<S>(client: &C, skip: S) -> LTAResult<Vec<BusStop>>
    where
        S: Into<Option<u32>> + Send;
}

#[async_trait]
impl BusRequests<LTAClient> for Bus {
    async fn get_arrival<'a, S, A>(
        client: &LTAClient,
        bus_stop_code: u32,
        service_no: S,
    ) -> LTAResult<BusArrivalResp>
    where
        S: Into<Option<A>> + Send,
        A: AsRef<str> + Send,
    {
        let url = api_url!("/BusArrivalv2");
        match service_no.into() {
            Some(srv_no) => {
                let srv_no = srv_no.as_ref();
                build_req_with_query::<RawBusArrivalResp, _, _, _>(client, url, |rb| {
                    rb.query(&[
                        ("BusStopCode", bus_stop_code.to_string().as_str()),
                        ("ServiceNo", srv_no),
                    ])
                })
                .await
            }
            None => {
                build_req_with_query::<RawBusArrivalResp, _, _, _>(client, url, |rb| {
                    rb.query(&[("BusStopCode", bus_stop_code.to_string())])
                })
                .await
            }
        }
    }

    async fn get_bus_services<S: Into<Option<u32>> + Send>(
        client: &LTAClient,
        skip: S,
    ) -> LTAResult<Vec<BusService>> {
        build_req_with_skip::<BusServiceResp, _, _>(client, api_url!("/BusServices"), skip.into())
            .await
    }

    async fn get_bus_routes<S: Into<Option<u32>> + Send>(
        client: &LTAClient,
        skip: S,
    ) -> LTAResult<Vec<BusRoute>> {
        build_req_with_skip::<BusRouteResp, _, _>(client, api_url!("/BusRoutes"), skip.into()).await
    }

    async fn get_bus_stops<S: Into<Option<u32>> + Send>(
        client: &LTAClient,
        skip: S,
    ) -> LTAResult<Vec<BusStop>> {
        build_req_with_skip::<BusStopsResp, _, _>(client, api_url!("/BusStops"), skip.into()).await
    }
}
