use lta_models::prelude::{BusArrivalResp, BusArrivalRespRaw as RawBusArrivalResp};

use crate::Client;
use crate::{
    blocking::{prelude::BusRequests, ClientExt, LTAClient},
    reqwest_blocking::ReqwestBlocking,
    Bus, LTAResult,
};
use concat_string::concat_string;

impl BusRequests<LTAClient<ReqwestBlocking>> for Bus {
    fn get_arrival<'a>(
        client: &LTAClient<ReqwestBlocking>,
        bus_stop_code: u32,
        service_no: impl Into<Option<&'a str>>,
    ) -> LTAResult<BusArrivalResp> {
        let url = concat_string!(client.base_url(), "/BusArrivalv2");

        match service_no.into() {
            Some(srv_no) => client.build_req_with_query::<RawBusArrivalResp, _, _>(&url, |rb| {
                let srv_no = srv_no.as_ref();
                rb.query(&[
                    ("BusStopCode", bus_stop_code.to_string().as_str()),
                    ("ServiceNo", srv_no),
                ])
            }),
            None => client.build_req_with_query::<RawBusArrivalResp, _, _>(&url, |rb| {
                rb.query(&[("BusStopCode", bus_stop_code.to_string())])
            }),
        }
    }
}
