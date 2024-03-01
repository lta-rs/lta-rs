use crate::reqwest_async::ReqwestAsync;
use crate::{Facility, FacilityRequests, LTAClient, LTAResult};
use async_trait::async_trait;
use lta_models::facility::prelude::FacilityMaintenanceRawResp;
use lta_models::prelude::StationCode;
use crate::r#async::ClientExt;

#[async_trait]
impl FacilityRequests<LTAClient<ReqwestAsync>> for Facility {
    async fn get_facilities_maintenance(
        client: &LTAClient<ReqwestAsync>,
        station_code: StationCode,
    ) -> LTAResult<Vec<String>> {
        client
            .build_req_with_query::<FacilityMaintenanceRawResp, _, _>(
                api_url!("/FacilitiesMaintenance"),
                |rb| rb.query(&[("StationCode", station_code)]),
            )
            .await
    }
}
