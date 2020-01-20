//! Async API calls for lta-rs. Currently uses async/await
pub mod bus;
pub mod crowd;
pub mod lta_client;
pub mod taxi;
pub mod traffic;
pub mod train;

use crate::lta_client::LTAClient;
use lta_utils_commons::{reqwest, serde, Client, LTAError};
use reqwest::RequestBuilder;

/// Builds an async request
pub(crate) async fn build_req_async_with_skip<T, M>(
    client: &LTAClient,
    url: &str,
    skip: Option<u32>,
) -> Result<M, LTAError>
where
    for<'de> T: serde::Deserialize<'de> + Into<M>,
{
    let skip = skip.unwrap_or(0);
    let rb = client.get_req_builder(url).query(&[("$skip", skip)]);
    rb.send().await?.json::<T>().await.map(|f| f.into())
}

/// Builds an async request that requires queries
pub(crate) async fn build_req_async_with_query<T, M, F>(
    client: &LTAClient,
    url: &str,
    query: F,
) -> Result<M, LTAError>
where
    F: FnOnce(RequestBuilder) -> RequestBuilder,
    for<'de> T: serde::Deserialize<'de> + Into<M>,
{
    let rb = client.get_req_builder(url);
    query(rb)
        .send()
        .await?
        .json::<T>()
        .await
        .map(|f: T| f.into())
}

#[cfg(test)]
mod tests {
    use crate::lta_client::LTAClient;
    use crate::{bus, crowd, taxi, traffic, train};
    use lta_models::prelude::VolType;
    use lta_models::traffic::road::RoadDetailsType;
    use lta_utils_commons::{Client, LTAResult};
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
        let client = LTAClient::with_api_key(api_key);
        client
    }

    #[tokio::test]
    async fn get_bus_arrivals() -> LTAResult<()> {
        let client = get_client();
        let x = bus::get_arrival(&client, 83139, None).await?;
        println!("{:?}", x);
        Ok(())
    }

    #[tokio::test]
    async fn get_bus_services() -> LTAResult<()> {
        gen_test!(bus::get_bus_services)
    }

    #[tokio::test]
    async fn get_bus_routes() -> LTAResult<()> {
        gen_test!(bus::get_bus_routes)
    }

    #[tokio::test]
    async fn get_bus_stops() -> LTAResult<()> {
        gen_test!(bus::get_bus_stops)
    }

    #[tokio::test]
    async fn get_passenger_vol() -> LTAResult<()> {
        let client = get_client();
        let data = crowd::get_passenger_vol_by(&client, VolType::OdBusStop, None, None).await?;
        println!("{:?}", data);
        Ok(())
    }

    #[tokio::test]
    async fn get_taxi_avail() -> LTAResult<()> {
        gen_test!(taxi::get_taxi_avail)
    }

    #[tokio::test]
    async fn get_taxi_stands() -> LTAResult<()> {
        gen_test!(taxi::get_taxi_stands)
    }

    #[tokio::test]
    async fn get_erp_rates() -> LTAResult<()> {
        gen_test!(traffic::get_erp_rates)
    }

    #[tokio::test]
    async fn get_cp_avail() -> LTAResult<()> {
        gen_test!(traffic::get_carkpark_avail)
    }

    #[tokio::test]
    async fn get_est_travel_time() -> LTAResult<()> {
        gen_test!(traffic::get_est_travel_time)
    }

    #[tokio::test]
    async fn get_faulty_traffic_lights() -> LTAResult<()> {
        gen_test!(traffic::get_faulty_traffic_lights)
    }

    #[tokio::test]
    async fn get_road_details() -> LTAResult<()> {
        let client = get_client();
        let data = traffic::get_road_details(&client, RoadDetailsType::RoadWorks, None).await?;
        println!("{:?}", data);
        Ok(())
    }

    #[tokio::test]
    async fn get_traffic_images() -> LTAResult<()> {
        gen_test!(traffic::get_traffic_images)
    }

    #[tokio::test]
    async fn get_traffic_incidents() -> LTAResult<()> {
        gen_test!(traffic::get_traffic_incidents)
    }

    #[tokio::test]
    async fn get_traffic_speed_band() -> LTAResult<()> {
        gen_test!(traffic::get_traffic_speed_band)
    }

    #[tokio::test]
    async fn get_vms() -> LTAResult<()> {
        gen_test!(traffic::get_vms_emas)
    }

    #[tokio::test]
    async fn get_bike_parking() -> LTAResult<()> {
        let client = get_client();
        let data = traffic::get_bike_parking(&client, 1.364897, 103.766094, Some(15.0)).await?;
        println!("{:?}", data);
        Ok(())
    }

    #[tokio::test]
    async fn get_train_service_alerts() -> LTAResult<()> {
        gen_test!(train::get_train_service_alert)
    }
}
