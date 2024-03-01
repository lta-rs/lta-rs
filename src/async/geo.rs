use crate::models::geo::prelude::*;
use crate::{Client, LTAResult};
use async_trait::async_trait;

use super::ClientExt;

#[async_trait]
pub trait GeoRequests<C: Client + ClientExt + Send + Sync> {
    /// Returns the SHP files of the requested geospatial layer
    ///
    /// **Update Freq**: Adhoc
    async fn get_geospatial_whole_island(
        client: &C,
        id: GeospatialLayerId,
    ) -> LTAResult<Vec<String>>;
}
