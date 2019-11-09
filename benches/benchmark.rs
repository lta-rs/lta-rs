#[macro_use]
extern crate criterion;
#[macro_use]
extern crate lazy_static;
extern crate serde_json;
use criterion::Criterion;
use lta::bus::bus_arrival::RawBusArrivalResp;
use lta::prelude::BusArrivalResp;
use serde::de::Deserialize;
use serde::Serialize;

const BUS_ARRIVAL_JSON: &str = include_str!("bus_arrival.json");

lazy_static! {
    static ref BUS_ARRIVAL_RESP: BusArrivalResp = { deserialize_bus_arrival() };
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

fn deserialize_bus_arrival() -> BusArrivalResp {
    de::<RawBusArrivalResp, _>(BUS_ARRIVAL_JSON)
}

fn serialize_bus_arrival(data: &BusArrivalResp) -> String {
    ser(data)
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("de_bus_arrival", |b| b.iter(|| deserialize_bus_arrival()));
    c.bench_function("ser_bus_arrival", |b| {
        b.iter(|| serialize_bus_arrival(&*BUS_ARRIVAL_RESP))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
