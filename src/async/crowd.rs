use crate::models::chrono::NaiveDate;
use crate::models::crowd::crowd_density::CrowdDensityForecast;
use crate::models::crowd::passenger_vol;
use crate::models::crowd::prelude::*;
use crate::r#async::client::LTAClient;
use crate::r#async::{build_req_with_query, build_req_with_skip};
use crate::{vol_type_to_url, Client, Crowd, LTAResult};
use async_trait::async_trait;
use lta_models::crowd::crowd_density::CrowdDensityForecastRawResp;
use lta_models::prelude::MrtLine;

/// All APIs pertaining to transportation crowd
#[async_trait]
pub trait CrowdRequests<C: Client> {
    /// **Update freq**: By 15th of every month, the passenger volume for previous month data
    /// will be generated
    ///
    /// Note: Link will expire after 5mins!
    async fn get_passenger_vol_by<S, D>(
        client: &C,
        vol_type: passenger_vol::VolType,
        date: D,
        skip: S,
    ) -> LTAResult<Vec<String>>
    where
        S: Into<Option<u32>> + Send,
        D: Into<Option<NaiveDate>> + Send;

    /// Returns real-time platform crowdedness level for the MRT/LRT stations of a
    /// particular train network line
    ///
    /// **Update freq**: 10 minutes
    async fn get_crowd_density_rt(
        client: &C,
        train_line: MrtLine,
    ) -> LTAResult<Vec<StationCrowdLevel>>;

    /// Returns forecasted platform crowdedness level for the MRT/LRT stations of a
    /// particular train network line at 30 minutes interval
    ///
    /// **Update freq**: 24hours
    async fn get_crowd_density_forecast(
        client: &C,
        train_line: MrtLine,
    ) -> LTAResult<CrowdDensityForecast>;
}

#[async_trait]
impl CrowdRequests<LTAClient> for Crowd {
    async fn get_passenger_vol_by<S, D>(
        client: &LTAClient,
        vol_type: VolType,
        date: D,
        skip: S,
    ) -> LTAResult<Vec<String>>
    where
        S: Into<Option<u32>> + Send,
        D: Into<Option<NaiveDate>> + Send,
    {
        let fmt_date = date
            .into()
            .map(|f| f.format(passenger_vol::FORMAT).to_string());

        let url = vol_type_to_url(vol_type)?;

        match fmt_date {
            Some(nd) => {
                build_req_with_query::<passenger_vol::PassengerVolRawResp, _, _, _>(
                    client,
                    url,
                    |rb| rb.query(&[("Date", nd)]),
                )
                .await
            }
            None => {
                build_req_with_skip::<passenger_vol::PassengerVolRawResp, _, _>(
                    client,
                    url,
                    skip.into(),
                )
                .await
            }
        }
    }

    async fn get_crowd_density_rt(
        client: &LTAClient,
        train_line: MrtLine,
    ) -> LTAResult<Vec<StationCrowdLevel>> {
        build_req_with_query::<StationCrowdLevelRawResp, _, _, _>(
            client,
            api_url!("/PCDRealTime"),
            |rb| rb.query(&[("TrainLine", train_line)]),
        )
        .await
    }

    async fn get_crowd_density_forecast(
        client: &LTAClient,
        train_line: MrtLine,
    ) -> LTAResult<CrowdDensityForecast> {
        build_req_with_query::<CrowdDensityForecastRawResp, _, _, _>(
            client,
            api_url!("/PCDForecast"),
            |rb| rb.query(&[("TrainLine", train_line)]),
        )
        .await
    }
}
