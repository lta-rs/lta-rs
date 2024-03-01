use crate::api_url;
use crate::models::traffic::prelude::*;
use crate::{Client, LTAError, LTAResult};
use async_trait::async_trait;

use super::ClientExt;

#[async_trait]
pub trait TrafficRequests<C: Client + ClientExt + Send + Sync> {
    /// Returns ERP rates of all vehicle types across all timings for each
    /// zone.
    ///
    /// **Update freq**: Ad-Hoc
    async fn get_erp_rates<S>(client: &C, skip: S) -> LTAResult<Vec<ErpRate>>
    where
        S: Into<Option<u32>> + Send,
    {
        client
            .build_req_with_skip::<ErpRatesResp, _>(api_url!("/ERPRates"), skip.into())
            .await
    }

    /// Returns no. of available lots for HDB, LTA and URA carpark data.
    /// The LTA carpark data consist of major shopping malls and developments within
    /// Orchard, Marina, HarbourFront, Jurong Lake District.
    /// (Note: list of LTA carpark data available on this API is subset of those listed on
    /// One.Motoring and MyTransport Portals)
    ///
    /// **Update freq**: 1 min
    async fn get_carpark_avail<S>(client: &C, skip: S) -> LTAResult<Vec<CarPark>>
    where
        S: Into<Option<u32>> + Send,
    {
        client
            .build_req_with_skip::<CarparkAvailResp, _>(
                api_url!("/CarParkAvailabilityv2"),
                skip.into(),
            )
            .await
    }

    /// Returns estimated travel times of expressways (in segments).
    ///
    /// **Update freq**: 5min
    async fn get_est_travel_time<S>(client: &C, skip: S) -> LTAResult<Vec<EstTravelTime>>
    where
        S: Into<Option<u32>> + Send,
    {
        client
            .build_req_with_skip::<EstTravelTimeResp, _>(api_url!("/EstTravelTimes"), skip.into())
            .await
    }

    /// Returns alerts of traffic lights that are currently faulty, or currently
    /// undergoing scheduled maintenance.
    ///
    /// **Update freq**: 2min or whenever there are updates
    async fn get_faulty_traffic_lights<S>(client: &C, skip: S) -> LTAResult<Vec<FaultyTrafficLight>>
    where
        S: Into<Option<u32>> + Send,
    {
        client
            .build_req_with_skip::<FaultyTrafficLightResp, _>(
                api_url!("/FaultyTrafficLights"),
                skip.into(),
            )
            .await
    }

    /// Returns all planned road openings or road works depending on the `RoadDetailsType` supplied
    ///
    /// **Update freq**: 24 hours – whenever there are updates
    async fn get_road_details<S>(
        client: &C,
        road_details_type: RoadDetailsType,
        skip: S,
    ) -> LTAResult<Vec<RoadDetails>>
    where
        S: Into<Option<u32>> + Send,
    {
        let url = match road_details_type {
            RoadDetailsType::RoadOpening => api_url!("/RoadOpenings"),
            RoadDetailsType::RoadWorks => api_url!("/RoadWorks"),
            _ => return Err(LTAError::UnknownEnumVariant),
        };

        client
            .build_req_with_skip::<RoadDetailsResp, _>(url, skip.into())
            .await
    }

    /// Returns current traffic speeds on expressways and arterial roads,
    /// expressed in speed bands.
    ///
    /// **Update freq**: 5 minutes
    async fn get_traffic_speed_band<S>(client: &C, skip: S) -> LTAResult<Vec<TrafficSpeedBand>>
    where
        S: Into<Option<u32>> + Send,
    {
        client
            .build_req_with_skip::<TrafficSpeedBandResp, _>(
                api_url!("/TrafficSpeedBandsv2"),
                skip.into(),
            )
            .await
    }

    /// Returns links to images of live traffic conditions along expressways and
    /// Woodlands & Tuas Checkpoints.
    ///
    /// **Update freq**: 1 to 5 minutes
    async fn get_traffic_images<S>(client: &C, skip: S) -> LTAResult<Vec<TrafficImage>>
    where
        S: Into<Option<u32>> + Send,
    {
        client
            .build_req_with_skip::<TrafficImageResp, _>(api_url!("/Traffic-Imagesv2"), skip.into())
            .await
    }

    /// Returns current traffic speeds on expressways and arterial roads,
    /// expressed in speed bands.
    ///
    /// **Update freq**: 5 minutes
    async fn get_traffic_incidents<S>(client: &C, skip: S) -> LTAResult<Vec<TrafficIncident>>
    where
        S: Into<Option<u32>> + Send,
    {
        client
            .build_req_with_skip::<TrafficIncidentResp, _>(
                api_url!("/TrafficIncidents"),
                skip.into(),
            )
            .await
    }

    /// Returns traffic advisories (via variable message services) concerning
    /// current traffic conditions that are displayed on EMAS signboards
    /// along expressways and arterial roads.
    ///
    /// **Update freq**: 2 minutes
    async fn get_vms_emas<S>(client: &C, skip: S) -> LTAResult<Vec<Vms>>
    where
        S: Into<Option<u32>> + Send,
    {
        client
            .build_req_with_skip::<VMSResp, _>(api_url!("/VMS"), skip.into())
            .await
    }

    /// Returns bicycle parking locations within a radius
    ///
    /// Dist is default to 0.5 even if you provide `None`
    ///
    /// **Update freq**: Monthly
    async fn get_bike_parking<D>(
        client: &C,
        lat: f64,
        long: f64,
        dist: D,
    ) -> LTAResult<Vec<BikeParking>>
    where
        D: Into<Option<f64>> + Send;
}