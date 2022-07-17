use crate::models::geo::geospatial_whole_island::{
    GeospatialLayerId,
};
use crate::{Client, LTAResult};

use super::ClientExt;

pub trait GeoRequests<C: Client + ClientExt> {
    /// Returns the SHP files of the requested geospatial layer
    ///
    /// **Update Freq**: Adhoc
    fn get_geospatial_whole_island(client: &C, id: GeospatialLayerId) -> LTAResult<Vec<String>>;
}

