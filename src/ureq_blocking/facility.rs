use lta_models::{facility::prelude::FacilityMaintenanceRawResp, prelude::StationCode};
use ureq::Agent;

use crate::{
    blocking::{prelude::FacilityRequests, ClientExt, LTAClient},
    Client, Facility, LTAResult,
};

use concat_string::concat_string;

impl FacilityRequests<LTAClient<Agent>> for Facility {
    fn get_facilities_maintenance(
        client: &LTAClient<Agent>,
        station_code: StationCode,
    ) -> LTAResult<Vec<String>> {
        client.build_req_with_query::<FacilityMaintenanceRawResp, _, _>(
            &concat_string!(client.base_url(), "/FacilitiesMaintenance"),
            |rb| rb.query("StationCode", &format!("{:?}", station_code)),
        )
    }
}
