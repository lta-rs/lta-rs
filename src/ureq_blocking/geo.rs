use lta_models::geo::prelude::{GeospatialLayerId, GeospatialWholeIslandRawResp};
use ureq::Agent;

use crate::{
    blocking::{prelude::GeoRequests, LTAClient, ClientExt},
    Geo, LTAResult,
};

impl GeoRequests<LTAClient<Agent>> for Geo {
    fn get_geospatial_whole_island(
        client: &LTAClient<Agent>,
        id: GeospatialLayerId,
    ) -> LTAResult<Vec<String>> {
        client.build_req_with_query::<GeospatialWholeIslandRawResp, _, _>(
            api_url!("/GeospatialWholeIsland"),
            |rb| rb.query("ID", &format!("{:?}", id)),
        )
    }
}
