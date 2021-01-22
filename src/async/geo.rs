use async_trait::async_trait;
use crate::{Client, LTAResult, LTAClient, Geo};
use crate::models::geo::prelude::*;
use crate::async::build_req_with_query;

#[async_trait]
pub trait GeoRequests<C: Client> {
    /// Returns the SHP files of the requested geospatial layer
    ///
    /// **Update Freq**: Adhoc
    async fn get_geospatial_whole_island(client: &C, id: GeospatialLayerId) -> LTAResult<Vec<String>>;
}

#[async_trait]
impl GeoRequests<LTAClient> for Geo {
    fn get_geospatial_whole_island(
        client: &LTAClient,
        id: GeospatialLayerId,
    ) -> LTAResult<Vec<String>> {
        build_req_with_query::<GeospatialWholeIslandRawResp, _, _, _>(
            client,
            api_url!("/GeospatialWholeIsland"),
            |rb| rb.query(&[("ID", id)]),
        ).await
    }
}