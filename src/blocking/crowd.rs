use crate::blocking::{build_req_with_query, build_req_with_skip, Client, LTAClient};
use crate::models::chrono::NaiveDate;
use crate::models::crowd::passenger_vol;
use crate::{Crowd, LTAError, LTAResult};
use lta_models::crowd::passenger_vol::VolType;

/// All APIs pertaining to transportation crowd
pub trait CrowdRequests<C: Client> {
    /// Creates a new client for every call
    /// **Update freq**: By 15th of every month, the passenger volume for previous month data
    /// will be generated
    ///
    /// Note: Link will expire after 5mins!
    fn get_passenger_vol_by(
        client: &C,
        vol_type: passenger_vol::VolType,
        date: Option<NaiveDate>,
        skip: Option<u32>,
    ) -> LTAResult<Vec<String>>;
}

impl CrowdRequests<LTAClient> for Crowd {
    fn get_passenger_vol_by(
        client: &LTAClient,
        vol_type: VolType,
        date: Option<NaiveDate>,
        skip: Option<u32>,
    ) -> LTAResult<Vec<String>> {
        let fmt_date = date.map(|f| f.format(passenger_vol::FORMAT).to_string());

        let url = match vol_type {
            VolType::BusStops => passenger_vol::URL_BY_BUS_STOPS,
            VolType::OdBusStop => passenger_vol::URL_BY_OD_BUS_STOPS,
            VolType::Train => passenger_vol::URL_BY_TRAIN,
            VolType::OdTrain => passenger_vol::URL_BY_OD_TRAIN,
            _ => return Err(LTAError::UnknownEnumVariant),
        };

        match fmt_date {
            Some(nd) => build_req_with_query::<passenger_vol::PassengerVolRawResp, _, _, _>(
                client,
                url,
                |rb| rb.query(&[("Date", nd)]),
            ),
            None => {
                build_req_with_skip::<passenger_vol::PassengerVolRawResp, _, _>(client, url, skip)
            }
        }
    }
}
