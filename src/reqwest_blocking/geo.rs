use lta_models::geo::prelude::{GeospatialLayerId, GeospatialWholeIslandRawResp};

use crate::{
    blocking::{prelude::GeoRequests, LTAClient, ClientExt},
    reqwest_blocking::ReqwestBlocking,
    Geo, LTAResult,
};

impl GeoRequests<LTAClient<ReqwestBlocking>> for Geo {
    fn get_geospatial_whole_island(
        client: &LTAClient<ReqwestBlocking>,
        id: GeospatialLayerId,
    ) -> LTAResult<Vec<String>> {
        client.build_req_with_query::<GeospatialWholeIslandRawResp, _, _>(
            api_url!("/GeospatialWholeIsland"),
            |rb| rb.query(&[("ID", id)]),
        )
    }
}
