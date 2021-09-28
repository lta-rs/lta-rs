pub mod bus;
pub mod client;
pub mod crowd;
pub mod facility;
pub mod geo;
pub mod taxi;
pub mod traffic;
pub mod train;

use crate::{Client, LTAError, LTAResult};

pub use crate::r#async::client::LTAClient;

pub mod prelude {
    pub use crate::r#async::{
        bus::BusRequests, crowd::CrowdRequests, facility::FacilityReqeusts, geo::GeoRequests,
        taxi::TaxiRequests, traffic::TrafficRequests, train::TrainRequests,
    };
}

/// helper function to build request with skip
pub(crate) async fn build_req_with_skip<T, T2, C>(
    client: &C,
    url: &str,
    skip: Option<u32>,
) -> LTAResult<T2>
where
    C: Client<RB = reqwest::RequestBuilder>,
    for<'de> T: serde::Deserialize<'de> + Into<T2>,
{
    let skip = skip.unwrap_or(0);
    let rb = client.req_builder(url).query(&[("$skip", skip)]);

    let res = rb.send().await?;
    let res = handle_status_code(res).await?;
    Ok(res.json::<T>().await.map(Into::into)?)
}

/// helper function to build request with query
pub(crate) async fn build_req_with_query<T, T2, F, C>(
    client: &C,
    url: &str,
    query: F,
) -> LTAResult<T2>
where
    F: FnOnce(reqwest::RequestBuilder) -> reqwest::RequestBuilder,
    C: Client<RB = reqwest::RequestBuilder>,
    for<'de> T: serde::Deserialize<'de> + Into<T2>,
{
    let rb = client.req_builder(url);

    let res = query(rb).send().await?;
    let res = handle_status_code(res).await?;
    Ok(res.json::<T>().await.map(Into::into)?)
}

async fn handle_status_code(res: reqwest::Response) -> LTAResult<reqwest::Response> {
    use reqwest::StatusCode;
    let status_code = res.status();

    if status_code.is_success() {
        return Ok(res);
    }

    let body = res.text().await.map_err(|_| LTAError::FailedToParseBody)?;

    match status_code {
        StatusCode::UNAUTHORIZED => Err(LTAError::Unauthorized),
        StatusCode::NOT_FOUND => Err(LTAError::NotFound),
        _ => Err(LTAError::UnhandledStatusCode(status_code, body)),
    }
}

#[cfg(test)]
mod tests {
    use crate::models::geo::prelude::GeospatialLayerId;
    use crate::models::prelude::{StationCode, VolType};
    use crate::models::traffic::road::RoadDetailsType;
    use crate::prelude::*;
    use crate::r#async::prelude::*;
    use crate::{Client, LTAClient, LTAResult, LTAError};
    use std::env;

    macro_rules! gen_test {
        ($f: expr) => {{
            let client = get_client();
            let data = $f(&client, None).await?;
            println!("{:?}", data);
            Ok(())
        }};
    }

    fn get_client() -> LTAClient {
        let api_key = env::var("API_KEY").expect("API_KEY does not exist!");
        let client = LTAClient::with_api_key(api_key).unwrap();
        client
    }

    #[tokio::test]
    async fn get_bus_arrivals() -> LTAResult<()> {
        let client = get_client();
        let x = Bus::get_arrival(&client, 83139, None).await?;
        println!("{:?}", x);
        Ok(())
    }

    #[tokio::test]
    async fn get_bus_services() -> LTAResult<()> {
        gen_test!(Bus::get_bus_services)
    }

    #[tokio::test]
    async fn get_bus_routes() -> LTAResult<()> {
        gen_test!(Bus::get_bus_routes)
    }

    #[tokio::test]
    async fn get_bus_stops() -> LTAResult<()> {
        gen_test!(Bus::get_bus_stops)
    }

    #[ignore]
    #[tokio::test]
    async fn get_passenger_vol() -> LTAResult<()> {
        let client = get_client();
        let data = Crowd::get_passenger_vol_by(&client, VolType::OdBusStop, None, None).await?;
        println!("{:?}", data);
        Ok(())
    }

    #[tokio::test]
    async fn get_taxi_avail() -> LTAResult<()> {
        gen_test!(Taxi::get_taxi_avail)
    }

    #[tokio::test]
    async fn get_taxi_stands() -> LTAResult<()> {
        gen_test!(Taxi::get_taxi_stands)
    }

    #[tokio::test]
    async fn get_erp_rates() -> LTAResult<()> {
        gen_test!(Traffic::get_erp_rates)
    }

    #[tokio::test]
    async fn get_cp_avail() -> LTAResult<()> {
        gen_test!(Traffic::get_carpark_avail)
    }

    #[tokio::test]
    async fn get_est_travel_time() -> LTAResult<()> {
        gen_test!(Traffic::get_est_travel_time)
    }

    #[tokio::test]
    async fn get_faulty_traffic_lights() -> LTAResult<()> {
        gen_test!(Traffic::get_faulty_traffic_lights)
    }

    #[tokio::test]
    async fn get_road_details() -> LTAResult<()> {
        let client = get_client();
        let data = Traffic::get_road_details(&client, RoadDetailsType::RoadWorks, None).await?;
        println!("{:?}", data);
        Ok(())
    }

    #[tokio::test]
    async fn get_traffic_images() -> LTAResult<()> {
        gen_test!(Traffic::get_traffic_images)
    }

    #[tokio::test]
    async fn get_traffic_incidents() -> LTAResult<()> {
        gen_test!(Traffic::get_traffic_incidents)
    }

    #[tokio::test]
    async fn get_traffic_speed_band() -> LTAResult<()> {
        gen_test!(Traffic::get_traffic_speed_band)
    }

    #[tokio::test]
    async fn get_vms() -> LTAResult<()> {
        gen_test!(Traffic::get_vms_emas)
    }

    #[tokio::test]
    async fn get_bike_parking() -> LTAResult<()> {
        let client = get_client();
        let data = Traffic::get_bike_parking(&client, 1.364897, 103.766094, Some(15.0)).await?;
        println!("{:?}", data);
        Ok(())
    }

    #[tokio::test]
    async fn get_train_service_alerts() -> LTAResult<()> {
        let x = gen_test!(Train::get_train_service_alert);
        if let Err(e) = x {
            return match e {
                LTAError::RateLimitReached => Ok(()),
                _ => Err(e)
            }
        }
        Ok(())
    }

    #[tokio::test]
    async fn get_geospatial_whole_island() -> LTAResult<()> {
        let client = get_client();
        let data =
            Geo::get_geospatial_whole_island(&client, GeospatialLayerId::ArrowMarking).await?;
        println!("{:?}", data);
        Ok(())
    }

    #[tokio::test]
    async fn get_facility_maintenance() -> LTAResult<()> {
        let client = get_client();
        let data = Facility::get_facilities_maintenance(&client, StationCode::BP1).await?;
        println!("{:?}", data);
        Ok(())
    }
}
