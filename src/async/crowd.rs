use crate::models::crowd::crowd_density::CrowdDensityForecast;
use crate::models::crowd::passenger_vol;
use crate::models::crowd::prelude::*;
use crate::{Client, LTAResult};
use lta_models::prelude::MrtLine;
use time::Date;

use super::ClientExt;

/// All APIs pertaining to transportation crowd
pub trait CrowdRequests<C: Client + ClientExt> {
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
        S: Into<Option<u32>>,
        D: Into<Option<Date>>;

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
