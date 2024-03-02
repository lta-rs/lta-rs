use crate::r#async::ClientExt;
use crate::reqwest_async::ReqwestAsync;
use crate::Client;
use crate::{Facility, FacilityRequests, LTAClient, LTAResult};
use concat_string::concat_string;
use lta_models::facility::prelude::FacilityMaintenanceRawResp;
use lta_models::prelude::StationCode;

impl FacilityRequests<LTAClient<ReqwestAsync>> for Facility {
    async fn get_facilities_maintenance(
        client: &LTAClient<ReqwestAsync>,
        station_code: StationCode,
    ) -> LTAResult<Vec<String>> {
        let url = concat_string!(client.base_url(), "/FacilitiesMaintenance");
        client
            .build_req_with_query::<FacilityMaintenanceRawResp, _, _>(url.as_str(), |rb| {
                rb.query(&[("StationCode", station_code)])
            })
            .await
    }
}
