use lta_models::geo::prelude::{GeospatialLayerId, GeospatialWholeIslandRawResp};

use crate::{
    blocking::{prelude::GeoRequests, ClientExt, LTAClient},
    reqwest_blocking::ReqwestBlocking,
    Client, Geo, LTAResult,
};
use concat_string::concat_string;

impl GeoRequests<LTAClient<ReqwestBlocking>> for Geo {
    fn get_geospatial_whole_island(
        client: &LTAClient<ReqwestBlocking>,
        id: GeospatialLayerId,
    ) -> LTAResult<Vec<String>> {
        client.build_req_with_query::<GeospatialWholeIslandRawResp, _, _>(
            &concat_string!(client.base_url(), "/GeospatialWholeIsland"),
            |rb| rb.query(&[("ID", id)]),
        )
    }
}
