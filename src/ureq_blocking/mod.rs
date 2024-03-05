pub mod bus;
pub mod client;
pub mod crowd;
pub mod facility;
pub mod geo;
pub mod taxi;
pub mod traffic;
pub mod train;

use crate::{
    blocking::{ClientExt, LTAClient},
    Client, LTAError, LTAResult,
};
pub use ureq::Agent;
use ureq::Response;

impl ClientExt for LTAClient<Agent> {
    fn build_req_with_skip<T, T2>(&self, url: &str, skip: Option<u32>) -> LTAResult<T2>
    where
        for<'de> T: serde::Deserialize<'de> + Into<T2>,
    {
        let skip = skip.unwrap_or(0);
        let rb = self
            .req_builder(url)
            .query("$skip", skip.to_string().as_str());

        rb.call()
            .map_err(|e| LTAError::BackendError(Box::new(e)))
            .and_then(handle_status_code)?
            .into_json::<T>()
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
            .call()
            .map_err(|e| LTAError::BackendError(Box::new(e)))
            .and_then(handle_status_code)?
            .into_json::<T>()
            .map(Into::into)
            .map_err(|_| LTAError::FailedToParseBody)
    }
}

fn handle_status_code(res: Response) -> LTAResult<Response> {
    let status_code = res.status();

    if status_code >= 200 && status_code < 300 {
        return Ok(res);
    }

    let body = res.into_string().map_err(|_| LTAError::FailedToParseBody)?;

    if body.contains("exceeded") {
        return Err(LTAError::RateLimitReached);
    }

    match status_code {
        500 => Err(LTAError::InternalServerError),
        404 => Err(LTAError::NotFound),
        401 => Err(LTAError::Unauthorized),
        _ => Err(LTAError::UnhandledStatusCode(
            http::status::StatusCode::from_u16(status_code).unwrap(),
            body,
        )),
    }
}

#[cfg(test)]
mod tests {
    use crate::blocking::geo::GeoRequests;
    use crate::blocking::prelude::*;
    use crate::blocking::*;
    use crate::prelude::*;
    use crate::LTAError;
    use crate::LTAResult;
    use crate::{Client, Facility, Geo};
    use lta_models::geo::geospatial_whole_island::GeospatialLayerId;
    use lta_models::prelude::*;
    use std::env;
    use ureq::Agent;

    macro_rules! gen_test {
        ($f: expr) => {{
            let client = get_client();
            let data = $f(&client, None);
            println!("{:?}", data);
            Ok(())
        }};
    }

    fn get_client() -> LTAClient<Agent> {
        let api_key = env::var("API_KEY").expect("API_KEY does not exist!");
        let client =
            LTAClient::with_api_key(api_key, "http://datamall2.mytransport.sg/ltaodataservice")
                .unwrap();
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
        let client = LTAClient::<Agent>::with_api_key(
            api_key,
            "http://datamall2.mytransport.sg/ltaodataservice",
        )
        .unwrap();
        let data = Bus::get_arrival(&client, 83139, None);
        if let Ok(_) = data {
            panic!("Should not be Ok()")
        }
    }

    #[test]
    #[ignore = "Ignored until LTA fixes their side. See issue#44"]
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
    fn get_traffic_flow() -> LTAResult<()> {
        let client = get_client();
        let data = Traffic::get_traffic_flow(&client)?;
        println!("{:?}", &data);
        assert_eq!(data.len(), 1);
        Ok(())
    }

    #[test]
    fn get_crowd_density_rt() -> LTAResult<()> {
        let client = get_client();
        let data = Crowd::get_crowd_density_rt(&client, MrtLine::BPL)?;
        println!("{:?}", data);
        Ok(())
    }

    #[ignore]
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
