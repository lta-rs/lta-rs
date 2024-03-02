use lta_models::prelude::{BusArrivalResp, BusArrivalRespRaw as RawBusArrivalResp};
use ureq::Agent;

use crate::Client;
use crate::{
    blocking::{prelude::BusRequests, ClientExt, LTAClient},
    Bus, LTAResult,
};
use concat_string::concat_string;

impl BusRequests<LTAClient<Agent>> for Bus {
    fn get_arrival<'a>(
        client: &LTAClient<Agent>,
        bus_stop_code: u32,
        service_no: impl Into<Option<&'a str>>,
    ) -> LTAResult<BusArrivalResp> {
        let url = concat_string!(client.base_url(), "/BusArrivalv2");
        let bus_stop_code = bus_stop_code.to_string();

        match service_no.into() {
            Some(srv_no) => client.build_req_with_query::<RawBusArrivalResp, _, _>(&url, |rb| {
                let srv_no = srv_no.as_ref();
                rb.query("BusStopCode", &bus_stop_code)
                    .query("ServiceNo", srv_no)
            }),
            None => client.build_req_with_query::<RawBusArrivalResp, _, _>(&url, |rb| {
                rb.query("BusStopCode", &bus_stop_code)
            }),
        }
    }
}
