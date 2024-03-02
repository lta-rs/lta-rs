use lta_models::{facility::prelude::FacilityMaintenanceRawResp, prelude::StationCode};

use crate::{
    blocking::{prelude::FacilityRequests, ClientExt, LTAClient},
    reqwest_blocking::ReqwestBlocking,
    Client, Facility, LTAResult,
};
use concat_string::concat_string;

impl FacilityRequests<LTAClient<ReqwestBlocking>> for Facility {
    fn get_facilities_maintenance(
        client: &LTAClient<ReqwestBlocking>,
        station_code: StationCode,
    ) -> LTAResult<Vec<String>> {
        client.build_req_with_query::<FacilityMaintenanceRawResp, _, _>(
            &concat_string!(client.base_url(), "/FacilitiesMaintenance"),
            |rb| rb.query(&[("StationCode", station_code)]),
        )
    }
}
