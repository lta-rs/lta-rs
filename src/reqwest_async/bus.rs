use async_trait::async_trait;
use lta_models::prelude::{BusArrivalResp, RawBusArrivalResp};

use crate::reqwest_async::ReqwestAsync;
use crate::{r#async::ClientExt, Bus, BusRequests, LTAClient, LTAResult};

#[async_trait]
impl BusRequests<LTAClient<ReqwestAsync>> for Bus {
    async fn get_arrival<'a, S>(
        client: &LTAClient<ReqwestAsync>,
        bus_stop_code: u32,
        service_no: S,
    ) -> LTAResult<BusArrivalResp>
    where
        S: Into<Option<&'a str>> + Send,
    {
        let url = api_url!("/BusArrivalv2");
        match service_no.into() {
            Some(srv_no) => {
                let srv_no = srv_no.as_ref();
                client
                    .build_req_with_query::<RawBusArrivalResp, _, _>(url, |rb| {
                        rb.query(&[
                            ("BusStopCode", bus_stop_code.to_string().as_str()),
                            ("ServiceNo", srv_no),
                        ])
                    })
                    .await
            }
            None => {
                client
                    .build_req_with_query::<RawBusArrivalResp, _, _>(url, |rb| {
                        rb.query(&[("BusStopCode", bus_stop_code.to_string())])
                    })
                    .await
            }
        }
    }
}
