use lta_models::{
    chrono::NaiveDate,
    crowd::passenger_vol,
    prelude::{
        CrowdDensityForecast, CrowdDensityForecastRawResp, MrtLine, StationCrowdLevel,
        StationCrowdLevelRawResp, VolType,
    },
};

use crate::{
    blocking::{prelude::CrowdRequests, LTAClient, ClientExt},
    reqwest_blocking::ReqwestBlocking,
    vol_type_to_url, Crowd, LTAResult,
};

impl CrowdRequests<LTAClient<ReqwestBlocking>> for Crowd {
    fn get_passenger_vol_by(
        client: &LTAClient<ReqwestBlocking>,
        vol_type: VolType,
        date: impl Into<Option<NaiveDate>>,
        skip: impl Into<Option<u32>>,
    ) -> LTAResult<Vec<String>> {
        let fmt_date = date
            .into()
            .map(|f| f.format(passenger_vol::FORMAT).to_string());

        let url = vol_type_to_url(vol_type)?;

        match fmt_date {
            Some(nd) => client
                .build_req_with_query::<passenger_vol::PassengerVolRawResp, _, _>(url, |rb| {
                    rb.query(&[("Date", nd)])
                }),
            None => client
                .build_req_with_skip::<passenger_vol::PassengerVolRawResp, _>(url, skip.into()),
        }
    }

    fn get_crowd_density_rt(
        client: &LTAClient<ReqwestBlocking>,
        train_line: MrtLine,
    ) -> LTAResult<Vec<StationCrowdLevel>> {
        client.build_req_with_query::<StationCrowdLevelRawResp, _, _>(
            api_url!("/PCDRealTime"),
            |rb| rb.query(&[("TrainLine", train_line)]),
        )
    }

    fn get_crowd_density_forecast(
        client: &LTAClient<ReqwestBlocking>,
        train_line: MrtLine,
    ) -> LTAResult<CrowdDensityForecast> {
        client.build_req_with_query::<CrowdDensityForecastRawResp, _, _>(
            api_url!("/PCDForecast"),
            |rb| rb.query(&[("TrainLine", train_line)]),
        )
    }
}
