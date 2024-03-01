use crate::models::train::StationCode;
use crate::{Client, LTAResult};

use super::ClientExt;

pub trait FacilityRequests<C: Client + ClientExt> {
    /// Returns pre-signed links to JSON file containing facilities maintenance schedules of the particular station
    ///
    /// **Update Freq**: Adhoc
    fn get_facilities_maintenance(client: &C, station_code: StationCode) -> LTAResult<Vec<String>>;
}