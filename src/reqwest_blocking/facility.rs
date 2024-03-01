use lta_models::{facility::prelude::FacilityMaintenanceRawResp, prelude::StationCode};

use crate::{
    blocking::{prelude::FacilityRequests, LTAClient, ClientExt},
    reqwest_blocking::ReqwestBlocking,
    Facility, LTAResult,
};

impl FacilityRequests<LTAClient<ReqwestBlocking>> for Facility {
    fn get_facilities_maintenance(
        client: &LTAClient<ReqwestBlocking>,
        station_code: StationCode,
    ) -> LTAResult<Vec<String>> {
        client.build_req_with_query::<FacilityMaintenanceRawResp, _, _>(
            api_url!("/FacilitiesMaintenance"),
            |rb| rb.query(&[("StationCode", station_code)]),
        )
    }
}
