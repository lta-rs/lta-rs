use crate::models::train::prelude::*;
use crate::{Client, LTAResult};
use async_trait::async_trait;

use super::ClientExt;

#[async_trait]
pub trait FacilityRequests<C: Client + ClientExt + Send + Sync> {
    /// Returns pre-signed links to JSON file containing facilities maintenance schedules of the particular station
    ///
    /// **Update Freq**: Adhoc
    async fn get_facilities_maintenance(
        client: &C,
        station_code: StationCode,
    ) -> LTAResult<Vec<String>>;
}