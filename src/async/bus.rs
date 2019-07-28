use reqwest::Error;
use tokio::prelude::Future;

use crate::bus::*;
use crate::r#async::lta_client::LTAClient;
use crate::utils::commons::Client;

pub fn get_arrival(
    client: &LTAClient,
    bus_stop_code: u32,
    service_no: &str,
) -> impl Future<Item = bus_arrival::BusArrivalResp, Error = Error> {
    let rb = client.get_req_builder(bus_arrival::URL).query(&[
        ("BusStopCode", bus_stop_code.to_string()),
        ("ServiceNo", service_no.to_string()),
    ]);

    rb.send().and_then(|mut r| r.json())
}

pub fn get_bus_services(
    client: &LTAClient,
) -> impl Future<Item = Vec<bus_services::BusService>, Error = Error> {
    let rb = client.get_req_builder(bus_services::URL);
    rb.send()
        .and_then(|mut r| r.json::<bus_services::BusServiceResp>())
        .map(|r| r.value)
}

pub fn get_bus_routes(
    client: &LTAClient,
) -> impl Future<Item = Vec<bus_routes::BusRoute>, Error = Error> {
    let rb = client.get_req_builder(bus_routes::URL);
    rb.send()
        .and_then(|mut r| r.json::<bus_routes::BusRouteResp>())
        .map(|r| r.value)
}

pub fn get_bus_stops(
    client: &LTAClient,
) -> impl Future<Item = Vec<bus_stops::BusStop>, Error = Error> {
    let rb = client.get_req_builder(bus_routes::URL);
    rb.send()
        .and_then(|mut r| r.json::<bus_stops::BusStopsResp>())
        .map(|r| r.value)
}
