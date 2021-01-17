use crate::blocking::{build_req_with_query, LTAClient};
use crate::models::facility::facilities_maintenance::{FacilityLink, FacilityMaintenanceRawResp};
use crate::models::train::StationCode;
use crate::{Client, Facility, LTAResult};

pub trait FacilityReqeusts<C: Client> {
    /// Returns pre-signed links to JSON file containing facilities maintenance schedules of the particular station
    ///
    /// **Update Freq**: Adhoc
    fn get_facilities_maintenance(client: &C, station_code: StationCode) -> LTAResult<Vec<String>>;
}

impl FacilityReqeusts<LTAClient> for Facility {
    fn get_facilities_maintenance(
        client: &LTAClient,
        station_code: StationCode,
    ) -> LTAResult<Vec<String>> {
        build_req_with_query::<FacilityMaintenanceRawResp, _, _, _>(
            client,
            api_url!("/FacilitiesMaintenance"),
            |rb| rb.query(&[("StationCode", station_code)]),
        )
    }
}
