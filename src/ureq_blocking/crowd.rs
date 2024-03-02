use concat_string::concat_string;
use lta_models::{
    crowd::passenger_vol,
    prelude::{
        CrowdDensityForecast, CrowdDensityForecastRawResp, MrtLine, StationCrowdLevel,
        StationCrowdLevelRawResp, VolType,
    },
};
use time::{macros::format_description, Date};
use ureq::Agent;

use crate::{
    blocking::{prelude::CrowdRequests, ClientExt, LTAClient},
    vol_type_to_url, Client, Crowd, LTAResult,
};

impl CrowdRequests<LTAClient<Agent>> for Crowd {
    fn get_passenger_vol_by(
        client: &LTAClient<Agent>,
        vol_type: VolType,
        date: impl Into<Option<Date>>,
        skip: impl Into<Option<u32>>,
    ) -> LTAResult<Vec<String>> {
        let format = format_description!("[year]-[month]-[day]");
        let fmt_date = date
            .into()
            .map(|f| f.format(&format))
            .expect("Failed to format.")
            .ok();

        let url = vol_type_to_url(client.base_url(), vol_type)?;

        match fmt_date {
            Some(nd) => client
                .build_req_with_query::<passenger_vol::PassengerVolRawResp, _, _>(&url, |rb| {
                    rb.query("Date", &nd)
                }),
            None => client
                .build_req_with_skip::<passenger_vol::PassengerVolRawResp, _>(&url, skip.into()),
        }
    }

    fn get_crowd_density_rt(
        client: &LTAClient<Agent>,
        train_line: MrtLine,
    ) -> LTAResult<Vec<StationCrowdLevel>> {
        client.build_req_with_query::<StationCrowdLevelRawResp, _, _>(
            &concat_string!(client.base_url(), "/PCDRealTime"),
            |rb| rb.query("TrainLine", &format!("{:?}", train_line)),
        )
    }

    fn get_crowd_density_forecast(
        client: &LTAClient<Agent>,
        train_line: MrtLine,
    ) -> LTAResult<CrowdDensityForecast> {
        client.build_req_with_query::<CrowdDensityForecastRawResp, _, _>(
            &concat_string!(client.base_url(), "/PCDForecast"),
            |rb| rb.query("TrainLine", &format!("{:?}", train_line)),
        )
    }
}
