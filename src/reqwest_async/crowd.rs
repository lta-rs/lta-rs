use lta_models::{
    crowd::passenger_vol,
    prelude::{
        CrowdDensityForecast, CrowdDensityForecastRawResp, MrtLine, StationCrowdLevel,
        StationCrowdLevelRawResp, VolType,
    },
};

use crate::{
    r#async::ClientExt, reqwest_async::ReqwestAsync, vol_type_to_url, Client, Crowd, CrowdRequests,
    LTAClient, LTAResult,
};
use concat_string::concat_string;
use time::{macros::format_description, Date};

impl CrowdRequests<LTAClient<ReqwestAsync>> for Crowd {
    async fn get_passenger_vol_by<S, D>(
        client: &LTAClient<ReqwestAsync>,
        vol_type: VolType,
        date: D,
        skip: S,
    ) -> LTAResult<Vec<String>>
    where
        S: Into<Option<u32>>,
        D: Into<Option<Date>>,
    {
        let format = format_description!("[year]-[month]-[day]");
        let fmt_date = date
            .into()
            .map(|f| f.format(&format))
            .expect("Failed to format.")
            .ok();

        let url = vol_type_to_url(client.base_url(), vol_type)?;

        match fmt_date {
            Some(nd) => {
                client
                    .build_req_with_query::<passenger_vol::PassengerVolRawResp, _, _>(&url, |rb| {
                        rb.query(&[("Date", nd)])
                    })
                    .await
            }
            None => {
                client
                    .build_req_with_skip::<passenger_vol::PassengerVolRawResp, _>(&url, skip.into())
                    .await
            }
        }
    }

    async fn get_crowd_density_rt(
        client: &LTAClient<ReqwestAsync>,
        train_line: MrtLine,
    ) -> LTAResult<Vec<StationCrowdLevel>> {
        client
            .build_req_with_query::<StationCrowdLevelRawResp, _, _>(
                &concat_string!(client.base_url(), "/PCDRealTime"),
                |rb| rb.query(&[("TrainLine", train_line)]),
            )
            .await
    }

    async fn get_crowd_density_forecast(
        client: &LTAClient<ReqwestAsync>,
        train_line: MrtLine,
    ) -> LTAResult<CrowdDensityForecast> {
        client
            .build_req_with_query::<CrowdDensityForecastRawResp, _, _>(
                &concat_string!(client.base_url(), "/PCDForecast"),
                |rb| rb.query(&[("TrainLine", train_line)]),
            )
            .await
    }
}
