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
    use crate::{bus, crowd, taxi, traffic, train};
    use crate::client_config::CLIENT_CONFIG;
    use crate::crowd::passenger_vol::VolType;
    use crate::traffic::carpark_avail::Carpark;
    use crate::traffic::traffic_speed_bands::TrafficSpeedBand;

    #[test]
    fn get_arrivals() {
        let resp = bus::get_arrival(83139, "15");
        match resp {
            Ok(bus_arrival_resp) => println!("{:?}", bus_arrival_resp),
            Err(e) => println!("{:?}", e)
        };
    }

    #[test]
    fn get_bus_services() {
        let resp = bus::get_bus_services();
        match resp {
            Ok(r) => println!("{:?}", r),
            Err(e) => println!("{:?}", e)
        };
    }

    #[test]
    fn get_bus_routes() {
        let resp = bus::get_bus_routes();
        match resp {
            Ok(r) => println!("{:?}", r),
            Err(e) => println!("{:?}", e)
        };
    }

    #[test]
    fn get_bus_stops() {
        let resp = bus::get_bus_stops();
        match resp {
            Ok(r) => println!("{:?}", r),
            Err(e) => println!("{:?}", e)
        };
    }

    #[test]
    fn get_passenger_vol() {
        let resp = crowd::get_passenger_vol_by(VolType::OdTrain);

        match resp {
            Ok(r) => println!("{:?}", r),
            Err(e) => println!("{:?}", e)
        };
    }

    #[test]
    fn get_taxi_avail() {
        let resp = taxi::get_taxi_avail();

        match resp {
            Ok(r) => println!("{:?}", r),
            Err(e) => println!("{:?}", e)
        };
    }

    #[test]
    fn get_erp_rates() {
        let resp = traffic::get_erp_rates();

        match resp {
            Ok(r) => println!("{:?}", r),
            Err(e) => println!("{:?}", e)
        };
    }

    #[test]
    fn get_cp_avail() {
        let resp = traffic::get_carpark_avail();

        match resp {
            Ok(r) => {
                println!("{:?}", r);
                let only_none: Vec<Carpark> = r.into_iter().filter(|p| {
                    p.coords.is_none()
                }).collect();
                println!("{:?}", only_none);
            }
            Err(e) => println!("{:?}", e)
        };
    }

    #[test]
    fn get_est_travel_time() {
        let resp = traffic::get_est_travel_time();

        match resp {
            Ok(r) => println!("{:?}", r),
            Err(e) => println!("{:?}", e)
        };
    }

    #[test]
    fn get_faulty_traffic_lights() {
        let resp = traffic::get_faulty_traffic_lights();

        match resp {
            Ok(r) => println!("{:?}", r),
            Err(e) => println!("{:?}", e)
        };
    }

    #[test]
    fn get_road_details() {
        let resp = traffic::get_road_details(traffic::road::RoadDetailsType::RoadWorks);

        match resp {
            Ok(r) => println!("{:?}", r),
            Err(e) => println!("{:?}", e)
        };
    }

    #[test]
    fn get_traffic_images() {
        let resp = traffic::get_traffic_images();

        match resp {
            Ok(r) => println!("{:?}", r),
            Err(e) => println!("{:?}", e)
        };
    }

    #[test]
    fn get_traffic_incidents() {
        let resp = traffic::get_traffic_incidents();

        match resp {
            Ok(r) => println!("{:?}", r),
            Err(e) => println!("{:?}", e)
        };
    }

    #[test]
    fn get_traffic_speed_band() {
        let resp = traffic::get_traffic_speed_band();

        match resp {
            Ok(r) => {
                println!("{:?}", r);
                let only_none: Vec<TrafficSpeedBand> = r.into_iter().filter(|p| {
                    p.coord_start_end.is_none()
                }).collect();

                println!("{:?}", only_none);
            }
            Err(e) => println!("{:?}", e)
        };
    }

    #[test]
    fn get_vms() {
        let resp = traffic::get_vms_emas();

        match resp {
            Ok(r) => println!("{:?}", r),
            Err(e) => println!("{:?}", e)
        }
    }


    #[test]
    fn get_bike_parking() {
        let resp =
            traffic::get_bike_parking(1.364897, 103.766094, 0.5);

        match resp {
            Ok(r) => println!("{:?}", r),
            Err(e) => println!("{:?}", e)
        }
    }

    #[test]
    fn get_train_service_alerts() {
        let resp = train::get_train_service_alert();

        match resp {
            Ok(r) => println!("{:?}", r),
            Err(e) => println!("{:?}", e)
        }
    }
}