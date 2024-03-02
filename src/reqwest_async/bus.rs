use crate::Client;
use concat_string::concat_string;
use lta_models::prelude::{BusArrivalResp, BusArrivalRespRaw as RawBusArrivalResp};

use crate::reqwest_async::ReqwestAsync;
use crate::{r#async::ClientExt, Bus, BusRequests, LTAClient, LTAResult};

impl BusRequests<LTAClient<ReqwestAsync>> for Bus {
    async fn get_arrival<'a, S>(
        client: &LTAClient<ReqwestAsync>,
        bus_stop_code: u32,
        service_no: S,
    ) -> LTAResult<BusArrivalResp>
    where
        S: Into<Option<&'a str>>,
    {
        let url = concat_string!(client.base_url(), "/BusArrivalv2");

        match service_no.into() {
            Some(srv_no) => {
                client
                    .build_req_with_query::<RawBusArrivalResp, _, _>(url.as_str(), |rb| {
                        rb.query(&[
                            ("BusStopCode", bus_stop_code.to_string().as_str()),
                            ("ServiceNo", srv_no),
                        ])
                    })
                    .await
            }
            None => {
                client
                    .build_req_with_query::<RawBusArrivalResp, _, _>(url.as_str(), |rb| {
                        rb.query(&[("BusStopCode", bus_stop_code.to_string())])
                    })
                    .await
            }
        }
    }
}
