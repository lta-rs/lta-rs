pub mod bus;
pub mod client;
pub mod crowd;
pub mod facility;
pub mod geo;
pub mod taxi;
pub mod traffic;
pub mod train;

pub use reqwest::blocking::{Client as ReqwestBlocking, RequestBuilder, Response};

use crate::{
    blocking::{ClientExt, LTAClient},
    LTAError, LTAResult, Client,
};

impl ClientExt for LTAClient<ReqwestBlocking> {
    fn build_req_with_skip<T, T2>(&self, url: &str, skip: Option<u32>) -> LTAResult<T2>
    where
        for<'de> T: serde::Deserialize<'de> + Into<T2>,
    {
        let skip = skip.unwrap_or(0);
        let rb = self.req_builder(url).query(&[("$skip", skip)]);
        rb.send()
            .map_err(|_| LTAError::BackendError)
            .and_then(handle_status_code)?
            .json::<T>()
            .map(Into::into)
            .map_err(|_| LTAError::FailedToParseBody)
    }

    fn build_req_with_query<T, T2, F>(&self, url: &str, query: F) -> LTAResult<T2>
    where
        F: FnOnce(Self::RB) -> Self::RB,
        for<'de> T: serde::Deserialize<'de> + Into<T2>,
    {
        let rb = self.req_builder(url);
        query(rb)
            .send()
            .map_err(|_| LTAError::BackendError)
            .and_then(handle_status_code)?
            .json::<T>()
            .map(Into::into)
            .map_err(|_| LTAError::FailedToParseBody)
    }
}

fn handle_status_code(res: Response) -> LTAResult<Response> {
    use reqwest::StatusCode;

    let status_code = res.status();

    if status_code.is_success() {
        return Ok(res);
    }

    let body = res.text().map_err(|_| LTAError::FailedToParseBody)?;

    if body.contains("exceeded") {
        return Err(LTAError::RateLimitReached);
    }

    match status_code {
        StatusCode::UNAUTHORIZED => Err(LTAError::Unauthorized),
        StatusCode::NOT_FOUND => Err(LTAError::NotFound),
        _ => Err(LTAError::UnhandledStatusCode(status_code, body)),
    }
}


#[cfg(test)]
mod tests {
    use crate::LTAError;
    use crate::blocking::geo::GeoRequests;
    use crate::blocking::prelude::*;
    use crate::blocking::*;
    use crate::prelude::*;
    use crate::LTAResult;
    use crate::{Client, Facility, Geo};
    use lta_models::geo::geospatial_whole_island::GeospatialLayerId;
    use lta_models::prelude::*;
    use std::env;
    use std::fs::File;
    use std::io::prelude::*;
    use crate::reqwest_blocking::ReqwestBlocking;

    macro_rules! gen_test {
        ($f: expr) => {{
            let client = get_client();
            let data = $f(&client, None);
            println!("{:?}", data);
            Ok(())
        }};
    }

    fn get_client() -> LTAClient<ReqwestBlocking> {
        let api_key = env::var("API_KEY").expect("API_KEY does not exist!");
        let client = LTAClient::with_api_key(api_key).unwrap();
        client
    }

    #[test]
    fn get_bus_arrivals() -> LTAResult<()> {
        let client = get_client();
        let data = Bus::get_arrival(&client, 83139, None)?;
        println!("{:?}", data);
        Ok(())
    }

    #[test]
    fn get_bus_arrivals_must_fail() {
        let api_key = "FAKE_KEY";
        let client = LTAClient::<ReqwestBlocking>::with_api_key(api_key).unwrap();
        let data = Bus::get_arrival(&client, 83139, None);
        if let Ok(_) = data {
            panic!("Should not be Ok()")
        }
    }

    #[test]
    fn get_bus_services() -> LTAResult<()> {
        gen_test!(Bus::get_bus_services)
    }

    #[test]
    fn get_bus_routes() -> LTAResult<()> {
        gen_test!(Bus::get_bus_routes)
    }

    #[test]
    fn get_bus_stops() -> LTAResult<()> {
        gen_test!(Bus::get_bus_stops)
    }

    #[ignore]
    #[test]
    fn get_passenger_vol() -> LTAResult<()> {
        let client = get_client();
        let data = Crowd::get_passenger_vol_by(&client, VolType::OdBusStop, None, None)?;
        println!("{:?}", data);
        Ok(())
    }

    #[test]
    fn get_taxi_avail() -> LTAResult<()> {
        gen_test!(Taxi::get_taxi_avail)
    }

    #[test]
    fn get_taxi_stands() -> LTAResult<()> {
        gen_test!(Taxi::get_taxi_stands)
    }

    #[test]
    fn get_erp_rates() -> LTAResult<()> {
        gen_test!(Traffic::get_erp_rates)
    }

    #[test]
    fn get_cp_avail() -> LTAResult<()> {
        gen_test!(Traffic::get_carpark_avail)
    }

    #[test]
    fn get_est_travel_time() -> LTAResult<()> {
        gen_test!(Traffic::get_est_travel_time)
    }

    #[test]
    fn get_faulty_traffic_lights() -> LTAResult<()> {
        gen_test!(Traffic::get_faulty_traffic_lights)
    }

    #[test]
    fn get_road_details() -> LTAResult<()> {
        let client = get_client();
        let data = Traffic::get_road_details(&client, RoadDetailsType::RoadWorks, None)?;
        println!("{:?}", data);
        Ok(())
    }

    #[test]
    fn get_traffic_images() -> LTAResult<()> {
        gen_test!(Traffic::get_traffic_images)
    }

    #[test]
    fn get_traffic_incidents() -> LTAResult<()> {
        gen_test!(Traffic::get_traffic_incidents)
    }

    #[test]
    fn get_traffic_speed_band() -> LTAResult<()> {
        gen_test!(Traffic::get_traffic_speed_band)
    }

    #[test]
    fn get_vms() -> LTAResult<()> {
        gen_test!(Traffic::get_vms_emas)
    }

    #[test]
    fn get_bike_parking() -> LTAResult<()> {
        let client = get_client();
        let data = Traffic::get_bike_parking(&client, 1.364897, 103.766094, 15.0)?;
        println!("{:?}", data);
        Ok(())
    }

    #[test]
    fn get_crowd_density_rt() -> LTAResult<()> {
        let client = get_client();
        let data = Crowd::get_crowd_density_rt(&client, MrtLine::BPL)?;
        println!("{:?}", data);
        Ok(())
    }

    #[test]
    fn get_crowd_density_forecast() -> LTAResult<()> {
        let client = get_client();
        let data = Crowd::get_crowd_density_forecast(&client, MrtLine::NSL);
        match data {
            Ok(d) => println!("{:?}", d),
            Err(e) => match e {
                LTAError::RateLimitReached => (),
                _ => panic!("{:?}", e),
            },
        }
        Ok(())
    }

    #[test]
    fn get_train_service_alerts() -> LTAResult<()> {
        let client = get_client();
        let x = Train::get_train_service_alert(&client, None);

        if let Err(e) = x {
            return match e {
                LTAError::RateLimitReached => Ok(()),
                _ => Err(e),
            };
        }

        Ok(())
    }

    #[test]
    #[ignore]
    #[rustfmt::skip]
    fn dump_json() -> Result<(), Box<dyn std::error::Error>>{
        use crate::models;

        let api_key = env::var("API_KEY").expect("`API_KEY` not present as env var!");
        let client = LTAClient::<ReqwestBlocking>::with_api_key(api_key).unwrap();
        let urls_with_query = [
            (lta_models::bus::bus_arrival::URL, &[("BusStopCode", "83139"), ("", ""), ("", "")], "bus_arrival.json"),
            (lta_models::traffic::bike_parking::URL, &[("Lat", "1.364897"), ("Long", "103.766094"), ("Dist", "15.0")], "bike_parking.json"),
        ];

        let urls = [
            (models::bus::bus_routes::URL, "bus_route.json"),
            (models::bus::bus_services::URL, "bus_services.json"),
            (models::bus::bus_stops::URL, "bus_stops.json"),
            (models::taxi::taxi_avail::URL, "taxi_avail.json"),
            (models::traffic::carpark_avail::URL, "carpark_avail.json"),
            (models::traffic::erp_rates::URL, "erp_rates.json"),
            (models::traffic::est_travel_time::URL, "est_travel_time.json"),
            (models::traffic::faulty_traffic_lights::URL, "faulty_traffic_lights.json"),
            (models::train::train_service_alert::URL, "train_service_alert.json"),
            (models::crowd::passenger_vol::URL_BY_BUS_STOPS, "passenger_vol_bus_stops.json"),
            (models::crowd::passenger_vol::URL_BY_OD_BUS_STOPS, "passenger_vol_od_bus_stops.json"),
            (models::crowd::passenger_vol::URL_BY_OD_TRAIN, "passenger_vol_od_train.json"),
            (models::crowd::passenger_vol::URL_BY_TRAIN, "passenger_vol_train.json"),
            (models::taxi::taxi_stands::URL, "taxi_stands.json")
        ];
        let mut results: Vec<(String, &str)> = Vec::with_capacity(15);

        for url in urls.iter() {
            let rb = client.req_builder(url.0);
            let data = rb
                .send()
                .map(|res| res.text().unwrap())?;

            println!("{}", &data);
            results.push((data, url.1))
        }

        for url in urls_with_query.iter() {
            let rb = client.req_builder(url.0);
            let data = rb
                .query(url.1)
                .send()
                .map(|res| res.text().unwrap())?;

            println!("{}", &data);
            results.push((data, url.2))
        }
        results.into_iter().for_each(|f| {
            let mut file = File::create(format!("./dumped_data/{}", f.1)).unwrap();
            file.write_all(f.0.as_bytes()).unwrap();
        });

        Ok(())
    }

    #[test]
    fn get_geospatial_whole_island() -> LTAResult<()> {
        let client = get_client();
        let data = Geo::get_geospatial_whole_island(&client, GeospatialLayerId::ArrowMarking)?;
        println!("{:?}", data);
        Ok(())
    }

    #[test]
    fn get_facility_maintenance() -> LTAResult<()> {
        let client = get_client();
        let data = Facility::get_facilities_maintenance(&client, StationCode::BP1)?;
        println!("{:?}", data);
        Ok(())
    }
}

