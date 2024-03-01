use lta::{LTAError, LTAClient, Client, Bus, BusRequests};

#[tokio::main]
async fn main() -> Result<(), LTAError> {
    let api_key = env!("API_KEY");
    let client = LTAClient::with_api_key(api_key)?;
    let bus_timing =  Bus::get_arrival(&client, 83139, None).await?;
    dbg!(bus_timing);
    Ok(())
}