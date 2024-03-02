use lta::{
    blocking::{prelude::TrafficRequests, LTAClient},
    ureq_blocking::Agent,
    Client, LTAError, Traffic,
};

fn main() -> Result<(), LTAError> {
    let api_key = env!("API_KEY");
    let client = LTAClient::<Agent>::with_api_key(
        api_key,
        "http://datamall2.mytransport.sg/ltaodataservice",
    )?;
    let erp_rates = Traffic::get_carpark_avail(&client, None)?;
    dbg!(erp_rates);
    Ok(())
}
