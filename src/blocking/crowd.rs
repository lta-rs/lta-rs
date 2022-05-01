use crate::blocking::{build_req_with_query, build_req_with_skip, LTAClient};
use crate::models::chrono::NaiveDate;
use crate::models::crowd::passenger_vol;
use crate::{vol_type_to_url, Client, Crowd, LTAResult};
use lta_models::crowd::passenger_vol::VolType;

/// All APIs pertaining to transportation crowd
pub trait CrowdRequests<C: Client> {
    /// Creates a new client for every call
    /// **Update freq**: By 15th of every month, the passenger volume for previous month data
    /// will be generated
    ///
    /// Note: Link will expire after 5mins!
    fn get_passenger_vol_by<S, D>(
        client: &C,
        vol_type: passenger_vol::VolType,
        date: D,
        skip: S,
    ) -> LTAResult<Vec<String>>
    where
        S: Into<Option<u32>>,
        D: Into<Option<NaiveDate>>;
}

impl CrowdRequests<LTAClient> for Crowd {
    fn get_passenger_vol_by<S, D>(
        client: &LTAClient,
        vol_type: VolType,
        date: D,
        skip: S,
    ) -> LTAResult<Vec<String>>
    where
        S: Into<Option<u32>>,
        D: Into<Option<NaiveDate>>,
    {
        let fmt_date = date.into().map(|f| f.format(passenger_vol::FORMAT).to_string());

        let url = vol_type_to_url(vol_type)?;

        match fmt_date {
            Some(nd) => build_req_with_query::<passenger_vol::PassengerVolRawResp, _, _, _>(
                client,
                url,
                |rb| rb.query(&[("Date", nd)]),
            ),
            None => {
                build_req_with_skip::<passenger_vol::PassengerVolRawResp, _, _>(client, url, skip.into())
            }
        }
    }
}
