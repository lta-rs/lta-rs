use super::ClientExt;
use crate::models::geo::prelude::*;
use crate::{Client, LTAResult};

pub trait GeoRequests<C: Client + ClientExt + Send> {
    /// Returns the SHP files of the requested geospatial layer
    ///
    /// **Update Freq**: Adhoc
    async fn get_geospatial_whole_island(
        client: &C,
        id: GeospatialLayerId,
    ) -> LTAResult<Vec<String>>;
}
