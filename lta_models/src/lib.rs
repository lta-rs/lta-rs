//! Data Structures for lta-rs

pub mod bus;
pub mod bus_enums;
pub mod crowd;
pub mod taxi;
pub mod traffic;
pub mod train;

pub mod prelude {
    pub use {
        crate::bus::prelude::*,
        crate::crowd::prelude,
        crate::taxi::prelude::*,
        crate::traffic::prelude::*,
        crate::train::prelude::*
    };
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use serde::{Deserialize, Serialize};

    fn generate_test<'de, I, S, F>(input_fn: F)
    where
        F: FnOnce() -> &'de str,
        I: Deserialize<'de> + Into<S>,
        S: Serialize,
    {
        let data = input_fn();
        let de: S = serde_json::from_str::<I>(data)
            .map(|f: I| f.into())
            .unwrap();
        let _ser = serde_json::to_string(&de).unwrap();
    }

    macro_rules! gen_test {
        ($a:ty, $b:ty, $c:expr) => {
            generate_test::<$a, $b, _>(|| include_str!($c));
        };
    }

    #[test]
    fn bus_arrival() {
        gen_test!(
            RawBusArrivalResp,
            BusArrivalResp,
            "../../dumped_data/bus_arrival.json"
        );
    }

    #[test]
    fn bus_service() {
        gen_test!(
            BusServiceResp,
            Vec<BusService>,
            "../../dumped_data/bus_services.json"
        );
    }

    #[test]
    fn bus_routes() {
        gen_test!(
            BusRouteResp,
            Vec<BusRoute>,
            "../../dumped_data/bus_route.json"
        );
    }

    #[test]
    fn bus_stops() {
        gen_test!(
            BusStopsResp,
            Vec<BusStop>,
            "../../dumped_data/bus_stops.json"
        );
    }

    #[test]
    fn est_travel_time() {
        gen_test!(
            EstTravelTimeResp,
            Vec<EstTravelTime>,
            "../../dumped_data/est_travel_time.json"
        );
    }
}
