use crate::blocking::{build_req_with_query, LTAClient};
use crate::models::geo::geospatial_whole_island::{
    GeospatialLayerId, GeospatialWholeIslandRawResp,
};
use crate::{Client, Geo, LTAResult};

pub trait GeoRequests<C: Client> {
    /// Returns the SHP files of the requested geospatial layer
    ///
    /// **Update Freq**: Adhoc
    fn get_geospatial_whole_island(client: &C, id: GeospatialLayerId) -> LTAResult<Vec<String>>;
}

impl GeoRequests<LTAClient> for Geo {
    fn get_geospatial_whole_island(
        client: &LTAClient,
        id: GeospatialLayerId,
    ) -> LTAResult<Vec<String>> {
        build_req_with_query::<GeospatialWholeIslandRawResp, _, _, _>(
            client,
            api_url!("/GeospatialWholeIsland"),
            |rb| rb.query(&[("ID", id)]),
        )
    }
}
