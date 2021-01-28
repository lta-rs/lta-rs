use lta::models::bus::bus_arrival::BusArrivalResp;
use lta::{blocking, Client};
use lta::{Bus, LTAClient, LTAResult};
use tokio::join;

fn blocking_get_arrival(blocking_client: &blocking::LTAClient) -> LTAResult<()> {
    use lta::blocking::bus::BusRequests;

    let bus_data = Bus::get_arrival(blocking_client, 83139, None)?;
    dbg!(bus_data);

    Ok(())
}

/// Do 2 requests concurrently
async fn async_get_arrival(client: &LTAClient) -> LTAResult<()> {
    use lta::BusRequests;

    let fut = Bus::get_arrival(client, 83139, None);
    let fut_2 = Bus::get_arrival(client, 83139, None);
    let (bus_data, bus_data_2) = join!(fut, fut_2);

    let bus_data = bus_data?;
    let bus_data_2 = bus_data_2?;

    dbg!(bus_data);
    dbg!(bus_data_2);

    Ok(())
}

/// If using blocking, remove tokio::main and async
#[tokio::main]
async fn main() -> LTAResult<()> {
    let key = env!("API_KEY");
    // let blocking_client = blocking::LTAClient::with_api_key(key)?;
    let async_client = LTAClient::with_api_key(key)?;

    // let _ = blocking_get_arrival(&blocking_client);
    let _ = async_get_arrival(&async_client).await;

    Ok(())
}
