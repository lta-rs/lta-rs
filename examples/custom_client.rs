use lta::{reqwest_async::ReqwestAsync, Bus, BusRequests, Client, LTAClient, LTAError};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), LTAError> {
    let api_key = env!("API_KEY");

    let custom_reqwest_client = ReqwestAsync::builder()
        .connect_timeout(Duration::from_secs(10))
        .no_brotli()
        .build()
        .map_err(|e| LTAError::BackendError(Box::new(e)))?;

    let client = LTAClient::new(
        api_key,
        custom_reqwest_client,
        "http://datamall2.mytransport.sg/ltaodataservice",
    );
    let bus_timing = Bus::get_arrival(&client, 83139, None).await?;
    dbg!(bus_timing);
    Ok(())
}
