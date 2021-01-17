use crate::models::chrono::NaiveDate;
use crate::models::crowd::passenger_vol;
use crate::models::crowd::prelude::*;
use crate::r#async::client::LTAClient;
use crate::r#async::{build_req_with_query, build_req_with_skip};
use crate::{vol_type_to_url, Client, Crowd, LTAResult};
use async_trait::async_trait;

/// All APIs pertaining to transportation crowd\
#[async_trait]
pub trait CrowdRequests<C: Client> {
    /// Creates a new client for every call
    /// **Update freq**: By 15th of every month, the passenger volume for previous month data
    /// will be generated
    ///
    /// Note: Link will expire after 5mins!
    async fn get_passenger_vol_by(
        client: &C,
        vol_type: passenger_vol::VolType,
        date: Option<NaiveDate>,
        skip: Option<u32>,
    ) -> LTAResult<Vec<String>>;
}

#[async_trait]
impl CrowdRequests<LTAClient> for Crowd {
    async fn get_passenger_vol_by(
        client: &LTAClient,
        vol_type: VolType,
        date: Option<NaiveDate>,
        skip: Option<u32>,
    ) -> LTAResult<Vec<String>> {
        let fmt_date = date.map(|f| f.format(passenger_vol::FORMAT).to_string());

        let url = vol_type_to_url(vol_type)?;

        match fmt_date {
            Some(nd) => {
                build_req_with_query::<passenger_vol::PassengerVolRawResp, _, _, _>(
                    client,
                    url,
                    |rb| rb.query(&[("Date", nd)]),
                )
                .await
            }
            None => {
                build_req_with_skip::<passenger_vol::PassengerVolRawResp, _, _>(client, url, skip)
                    .await
            }
        }
    }
}
