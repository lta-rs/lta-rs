#[macro_use]
extern crate criterion;
#[macro_use]
extern crate lazy_static;
extern crate serde_json;
use criterion::Criterion;
use lta::bus::bus_arrival::RawBusArrivalResp;
use lta::bus::bus_routes::{BusRoute, BusRouteResp};
use lta::bus::bus_services::{BusService, BusServiceResp};
use lta::bus::bus_stops::{BusStop, BusStopsResp};
use lta::prelude::BusArrivalResp;
use serde::de::Deserialize;
use serde::Serialize;

const BUS_ARRIVAL_JSON: &str = include_str!("../dumped_data/bus_arrival.json");
const BUS_ROUTE_JSON: &str = include_str!("../dumped_data/bus_route.json");
const BUS_SERVICES_JSON: &str = include_str!("../dumped_data/bus_services.json");
const BUS_STOPS_JSON: &str = include_str!("../dumped_data/bus_stops.json");

const CARPARK_AVAIL_JSON: &str = include_str!("../dumped_data/carpark_avail.json");
const ERP_RATES_JSON: &str = include_str!("../dumped_data/erp_rates.json");
const EST_TRAVEL_TIME_JSON: &str = include_str!("../dumped_data/est_travel_time.json");
const FAULTY_TRAFFIC_LIGHTS_JSON: &str = include_str!("../dumped_data/est_travel_time.json");

lazy_static! {
    static ref BUS_ARRIVAL_RESP: BusArrivalResp = { de::<RawBusArrivalResp, _>(BUS_ARRIVAL_JSON) };
    static ref BUS_ROUTE_RESP: Vec<BusRoute> = { de::<BusRouteResp, _>(BUS_ROUTE_JSON) };
    static ref BUS_SERVICES_RESP: Vec<BusService> = { de::<BusServiceResp, _>(BUS_SERVICES_JSON) };
    static ref BUS_STOPS_RESP: Vec<BusStop> = { de::<BusStopsResp, _>(BUS_STOPS_JSON) };
}

fn de<'a, M, T>(data: &'a str) -> T
where
    M: Into<T> + Deserialize<'a>,
    T: Deserialize<'a>,
{
    serde_json::from_str::<M>(data).unwrap().into()
}

fn ser<T>(data: &T) -> String
where
    T: Serialize,
{
    serde_json::to_string(data).unwrap()
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn criterion_bus_benchmark(c: &mut Criterion) {
    c.bench_function("de_bus_arrival", |b| b.iter(|| de::<RawBusArrivalResp, BusArrivalResp>(BUS_ARRIVAL_JSON)));
    c.bench_function("ser_bus_arrival", |b| b.iter(|| ser(&*BUS_ARRIVAL_RESP)));

    c.bench_function("de_route", |b| b.iter(|| de::<BusRouteResp, Vec<BusRoute>>(BUS_ROUTE_JSON)));
    c.bench_function("ser_route", |b| b.iter(|| ser(&*BUS_ROUTE_RESP)));

    c.bench_function("de_services", |b| b.iter(|| de::<BusServiceResp, Vec<BusService>>(BUS_SERVICES_JSON)));
    c.bench_function("ser_services", |b| b.iter(|| ser(&*BUS_SERVICES_RESP)));

    c.bench_function("de_bus_stop", |b| b.iter(|| de::<BusStopsResp, Vec<BusStop>>(BUS_STOPS_JSON)));
    c.bench_function("ser_bus_stop", |b| b.iter(|| ser(&*BUS_STOPS_RESP)));
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn criterion_traffic_benchmark(c: &mut Criterion) {
    unimplemented!()
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn criterion_crowd_benchmark(c: &mut Criterion) {
    unimplemented!()
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn criterion_taxi_benchmark(c: &mut Criterion) {
    unimplemented!()
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn criterion_train_benchmark(c: &mut Criterion) {
    unimplemented!()
}

criterion_group!(benches, criterion_bus_benchmark);
criterion_main!(benches);
