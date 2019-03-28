#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate reqwest;
extern crate serde;
extern crate toml;

pub mod client_config;
pub mod bus;
pub mod bus_enums;
pub mod utils;
pub mod crowd;
pub mod taxi;
pub mod traffic;
pub mod train;

#[cfg(test)]
mod tests {
    use std::env;
    use std::fmt::Debug;

    use crate::{bus, crowd, taxi, traffic, train};
    use crate::client_config::CLIENT_CONFIG;
    use crate::crowd::passenger_vol::VolType;
    use crate::traffic::carpark_avail::Carpark;
    use crate::traffic::traffic_speed_bands::TrafficSpeedBand;

    fn run_test_and_print<F, T>(f: F)
        where F: Fn() -> reqwest::Result<T>,
              T: Debug
    {
        let api_key = env::var("API_KEY").unwrap();
        CLIENT_CONFIG.lock().unwrap().with_api_key(api_key.as_str());
        let res = f();
        match res {
            Ok(r) => println!("{:?}", r),
            Err(e) => println!("{:?}", e)
        };
    }

    #[test]
    fn get_arrivals() {
        run_test_and_print(|| bus::get_arrival(83139, "15"));
    }

    #[test]
    fn get_bus_services() {
        run_test_and_print(|| bus::get_bus_services());
    }

    #[test]
    fn get_bus_routes() {
        run_test_and_print(|| bus::get_bus_routes())
    }

    #[test]
    fn get_bus_stops() {
        run_test_and_print(|| bus::get_bus_stops());
    }

    #[test]
    fn get_passenger_vol() {
        run_test_and_print(|| crowd::get_passenger_vol_by(VolType::OdTrain))
    }

    #[test]
    fn get_taxi_avail() {
        run_test_and_print(|| taxi::get_taxi_avail())
    }

    #[test]
    fn get_erp_rates() {
        run_test_and_print(|| traffic::get_erp_rates());
    }

    #[test]
    fn get_cp_avail() {
        run_test_and_print(|| traffic::get_carpark_avail());
    }

    #[test]
    fn get_est_travel_time() {
        run_test_and_print(traffic::get_est_travel_time());
    }

    #[test]
    fn get_faulty_traffic_lights() {
        run_test_and_print(|| traffic::get_faulty_traffic_lights());
    }

    #[test]
    fn get_road_details() {
        run_test_and_print(|| traffic::get_road_details(traffic::road::RoadDetailsType::RoadWorks));
    }

    #[test]
    fn get_traffic_images() {
        run_test_and_print(|| traffic::get_traffic_images());
    }

    #[test]
    fn get_traffic_incidents() {
        run_test_and_print(|| traffic::get_traffic_incidents());
    }

    #[test]
    fn get_traffic_speed_band() {
        run_test_and_print(|| traffic::get_traffic_speed_band());
    }

    #[test]
    fn get_vms() {
        run_test_and_print(|| traffic::get_vms_emas());
    }


    #[test]
    fn get_bike_parking() {
        run_test_and_print(|| traffic::get_bike_parking(1.364897, 103.766094, 0.5));
    }

    #[test]
    fn get_train_service_alerts() {
        run_test_and_print(|| train::get_train_service_alert())
    }
}