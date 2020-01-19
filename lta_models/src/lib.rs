//! Data Structures for lta-rs

pub mod bus;
pub mod bus_enums;
pub mod crowd;
pub mod taxi;
pub mod traffic;
pub mod train;

/// Data structures for all data
pub mod prelude {
    pub use {
        crate::bus::prelude::*, crate::crowd::prelude::*, crate::taxi::prelude::*,
        crate::traffic::prelude::*, crate::train::prelude::*,
    };
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use serde::{Deserialize, Serialize};
    use std::fmt::Debug;

    fn generate_test<'de, I, S, F>(input_fn: F)
    where
        F: FnOnce() -> &'de str,
        I: Deserialize<'de> + Into<S>,
        S: Serialize + Debug,
    {
        let data = input_fn();
        let de: S = serde_json::from_str::<I>(data)
            .map(|f: I| f.into())
            .unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        println!("{}", ser);
    }

    macro_rules! gen_test {
        ($a:ty, $b:ty, $c:expr) => {
            generate_test::<$a, $b, _>(|| include_str!($c));
        };
    }

    #[test]
    fn bike_parking() {
        gen_test!(
            BikeParkingResp,
            Vec<BikeParking>,
            "../../dumped_data/bike_parking.json"
        );
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
    fn bus_routes() {
        gen_test!(
            BusRouteResp,
            Vec<BusRoute>,
            "../../dumped_data/bus_route.json"
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
    fn bus_stops() {
        gen_test!(
            BusStopsResp,
            Vec<BusStop>,
            "../../dumped_data/bus_stops.json"
        );
    }

    #[test]
    fn carpark_avail() {
        gen_test!(
            CarparkAvailResp,
            Vec<CarPark>,
            "../../dumped_data/carpark_avail.json"
        );
    }

    #[test]
    fn erp_rates() {
        gen_test!(
            ErpRatesResp,
            Vec<ErpRate>,
            "../../dumped_data/erp_rates.json"
        );
    }

    #[test]
    fn faulty_traffic_lights() {
        gen_test!(
            FaultyTrafficLightResp,
            Vec<FaultyTrafficLight>,
            "../../dumped_data/faulty_traffic_lights.json"
        );
    }

    #[test]
    fn passenger_vol_bus_stops() {
        gen_test!(
            PassengerVolRawResp,
            Vec<String>,
            "../../dumped_data/passenger_vol_bus_stops.json"
        );
    }

    #[test]
    fn passenger_vol_od_bus_stops() {
        gen_test!(
            PassengerVolRawResp,
            Vec<String>,
            "../../dumped_data/passenger_vol_od_bus_stops.json"
        );
    }

    #[test]
    fn passenger_vol_od_train() {
        gen_test!(
            PassengerVolRawResp,
            Vec<String>,
            "../../dumped_data/passenger_vol_od_train.json"
        );
    }

    #[test]
    fn passenger_vol_train() {
        gen_test!(
            PassengerVolRawResp,
            Vec<String>,
            "../../dumped_data/passenger_vol_train.json"
        );
    }

    #[test]
    fn taxi_avail() {
        gen_test!(
            TaxiAvailResp,
            Vec<Coordinates>,
            "../../dumped_data/taxi_avail.json"
        );
    }

    #[test]
    fn taxi_stands() {
        gen_test!(
            TaxiStandsResp,
            Vec<TaxiStand>,
            "../../dumped_data/taxi_stands.json"
        );
    }

    #[test]
    fn train_service_alert() {
        gen_test!(
            TrainServiceAlertResp,
            TrainServiceAlert,
            "../../dumped_data/train_service_alert.json"
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
