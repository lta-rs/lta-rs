use async_trait::async_trait;
use lta_models::geo::prelude::{GeospatialLayerId, GeospatialWholeIslandRawResp};

use crate::r#async::ClientExt;
use crate::{reqwest_async::ReqwestAsync, Geo, GeoRequests, LTAClient, LTAResult};

#[async_trait]
impl GeoRequests<LTAClient<ReqwestAsync>> for Geo {
    async fn get_geospatial_whole_island(
        client: &LTAClient<ReqwestAsync>,
        id: GeospatialLayerId,
    ) -> LTAResult<Vec<String>> {
        client
            .build_req_with_query::<GeospatialWholeIslandRawResp, _, _>(
                api_url!("/GeospatialWholeIsland"),
                |rb| rb.query(&[("ID", id)]),
            )
            .await
    }
}
