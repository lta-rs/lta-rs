use lta_models::geo::prelude::{GeospatialLayerId, GeospatialWholeIslandRawResp};
use ureq::Agent;

use crate::{
    blocking::{prelude::GeoRequests, ClientExt, LTAClient},
    Client, Geo, LTAResult,
};
use concat_string::concat_string;

impl GeoRequests<LTAClient<Agent>> for Geo {
    fn get_geospatial_whole_island(
        client: &LTAClient<Agent>,
        id: GeospatialLayerId,
    ) -> LTAResult<Vec<String>> {
        client.build_req_with_query::<GeospatialWholeIslandRawResp, _, _>(
            &concat_string!(client.base_url(), "/GeospatialWholeIsland"),
            |rb| rb.query("ID", &format!("{:?}", id)),
        )
    }
}
