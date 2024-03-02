use crate::models::crowd::passenger_vol;
use crate::{Client, LTAResult};
use lta_models::crowd::crowd_density::CrowdDensityForecast;
use lta_models::prelude::{MrtLine, StationCrowdLevel};
use time::Date;

use super::ClientExt;

/// All APIs pertaining to transportation crowd
pub trait CrowdRequests<C: Client + ClientExt> {
    /// Creates a new client for every call
    /// **Update freq**: By 15th of every month, the passenger volume for previous month data
    /// will be generated
    ///
    /// Note: Link will expire after 5mins!
    fn get_passenger_vol_by(
        client: &C,
        vol_type: passenger_vol::VolType,
        date: impl Into<Option<Date>>,
        skip: impl Into<Option<u32>>,
    ) -> LTAResult<Vec<String>>;

    /// Returns real-time platform crowdedness level for the MRT/LRT stations of a
    /// particular train network line
    ///
    /// **Update freq**: 10 minutes
    fn get_crowd_density_rt(client: &C, train_line: MrtLine) -> LTAResult<Vec<StationCrowdLevel>>;

    /// Returns forecasted platform crowdedness level for the MRT/LRT stations of a
    /// particular train network line at 30 minutes interval
    ///
    /// **Update freq**: 24hours
    fn get_crowd_density_forecast(
        client: &C,
        train_line: MrtLine,
    ) -> LTAResult<CrowdDensityForecast>;
}
