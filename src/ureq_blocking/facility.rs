use lta_models::{facility::prelude::FacilityMaintenanceRawResp, prelude::StationCode};
use ureq::Agent;

use crate::{
    blocking::{prelude::FacilityRequests, LTAClient, ClientExt},
    Facility, LTAResult,
};

impl FacilityRequests<LTAClient<Agent>> for Facility {
    fn get_facilities_maintenance(
        client: &LTAClient<Agent>,
        station_code: StationCode,
    ) -> LTAResult<Vec<String>> {
        client.build_req_with_query::<FacilityMaintenanceRawResp, _, _>(
            api_url!("/FacilitiesMaintenance"),
            |rb| rb.query("StationCode", &format!("{:?}", station_code)),
        )
    }
}
