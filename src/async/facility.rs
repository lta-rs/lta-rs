use crate::models::facility::prelude::FacilityMaintenanceRawResp;
use crate::models::train::prelude::*;
use crate::r#async::build_req_with_query;
use crate::{Client, Facility, LTAClient, LTAResult};
use async_trait::async_trait;

#[async_trait]
pub trait FacilityRequests<C: Client> {
    /// Returns pre-signed links to JSON file containing facilities maintenance schedules of the particular station
    ///
    /// **Update Freq**: Adhoc
    async fn get_facilities_maintenance(
        client: &C,
        station_code: StationCode,
    ) -> LTAResult<Vec<String>>;
}

#[async_trait]
impl FacilityRequests<LTAClient> for Facility {
    async fn get_facilities_maintenance(
        client: &LTAClient,
        station_code: StationCode,
    ) -> LTAResult<Vec<String>> {
        build_req_with_query::<FacilityMaintenanceRawResp, _, _, _>(
            client,
            api_url!("/FacilitiesMaintenance"),
            |rb| rb.query(&[("StationCode", station_code)]),
        )
        .await
    }
}
