use lta::{
    blocking::{prelude::TrafficRequests, LTAClient},
    reqwest_blocking::ReqwestBlocking,
    Client, LTAError, Traffic,
};

fn main() -> Result<(), LTAError> {
    let api_key = env!("API_KEY");
    let client = LTAClient::<ReqwestBlocking>::with_api_key(api_key)?;
    let erp_rates = Traffic::get_erp_rates(&client, None)?;
    dbg!(erp_rates);
    Ok(())
}
