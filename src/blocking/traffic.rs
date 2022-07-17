use crate::api_url;
use crate::models::traffic::prelude::*;
use crate::{Client, LTAError, LTAResult};

use super::ClientExt;

pub trait TrafficRequests<C: Client + ClientExt> {
    /// Returns ERP rates of all vehicle types across all timings for each
    /// zone.
    ///
    /// **Update freq**: Ad-Hoc
    fn get_erp_rates(client: &C, skip: impl Into<Option<u32>>) -> LTAResult<Vec<ErpRate>> {
        client.build_req_with_skip::<ErpRatesResp, _>(api_url!("/ERPRates"), skip.into())
    }

    /// Returns no. of available lots for HDB, LTA and URA carpark data.
    /// The LTA carpark data consist of major shopping malls and developments within
    /// Orchard, Marina, HarbourFront, Jurong Lake District.
    /// (Note: list of LTA carpark data available on this API is subset of those listed on
    /// One.Motoring and MyTransport Portals)
    ///
    /// **Update freq**: 1 min
    fn get_carpark_avail(client: &C, skip: impl Into<Option<u32>>) -> LTAResult<Vec<CarPark>> {
        client.build_req_with_skip::<CarparkAvailResp, _>(
            api_url!("/CarParkAvailabilityv2"),
            skip.into(),
        )
    }
    /// Returns estimated travel times of expressways (in segments).
    ///
    /// **Update freq**: 5min
    fn get_est_travel_time(
        client: &C,
        skip: impl Into<Option<u32>>,
    ) -> LTAResult<Vec<EstTravelTime>> {
        client.build_req_with_skip::<EstTravelTimeResp, _>(api_url!("/EstTravelTimes"), skip.into())
    }

    /// Returns alerts of traffic lights that are currently faulty, or currently
    /// undergoing scheduled maintenance.
    ///
    /// **Update freq**: 2min or whenever there are updates
    fn get_faulty_traffic_lights(
        client: &C,
        skip: impl Into<Option<u32>>,
    ) -> LTAResult<Vec<FaultyTrafficLight>> {
        client.build_req_with_skip::<FaultyTrafficLightResp, _>(
            api_url!("/FaultyTrafficLights"),
            skip.into(),
        )
    }

    /// Returns all planned road openings or road works depending on the `RoadDetailsType` supplied
    ///
    /// **Update freq**: 24 hours – whenever there are updates
    fn get_road_details(
        client: &C,
        road_details_type: RoadDetailsType,
        skip: impl Into<Option<u32>>,
    ) -> LTAResult<Vec<RoadDetails>> {
        let url = match road_details_type {
            RoadDetailsType::RoadOpening => api_url!("/RoadOpenings"),
            RoadDetailsType::RoadWorks => api_url!("/RoadWorks"),
            _ => return Err(LTAError::UnknownEnumVariant),
        };

        client.build_req_with_skip::<RoadDetailsResp, _>(url, skip.into())
    }
    /// Returns current traffic speeds on expressways and arterial roads,
    /// expressed in speed bands.
    ///
    /// **Update freq**: 5 minutes
    fn get_traffic_speed_band(
        client: &C,
        skip: impl Into<Option<u32>>,
    ) -> LTAResult<Vec<TrafficSpeedBand>> {
        client.build_req_with_skip::<TrafficSpeedBandResp, _>(
            api_url!("/TrafficSpeedBandsv2"),
            skip.into(),
        )
    }

    /// Returns links to images of live traffic conditions along expressways and
    /// Woodlands & Tuas Checkpoints.
    ///
    /// **Update freq**: 1 to 5 minutes
    fn get_traffic_images(
        client: &C,
        skip: impl Into<Option<u32>>,
    ) -> LTAResult<Vec<TrafficImage>> {
        client
            .build_req_with_skip::<TrafficImageResp, _>(api_url!("/Traffic-Imagesv2"), skip.into())
    }
    /// Returns current traffic speeds on expressways and arterial roads,
    /// expressed in speed bands.
    ///
    /// **Update freq**: 5 minutes
    fn get_traffic_incidents(
        client: &C,
        skip: impl Into<Option<u32>>,
    ) -> LTAResult<Vec<TrafficIncident>> {
        client.build_req_with_skip::<TrafficIncidentResp, _>(
            api_url!("/TrafficIncidents"),
            skip.into(),
        )
    }

    /// Returns traffic advisories (via variable message services) concerning
    /// current traffic conditions that are displayed on EMAS signboards
    /// along expressways and arterial roads.
    ///
    /// **Update freq**: 2 minutes
    fn get_vms_emas(client: &C, skip: impl Into<Option<u32>>) -> LTAResult<Vec<Vms>> {
        client.build_req_with_skip::<VMSResp, _>(api_url!("/VMS"), skip.into())
    }

    /// Returns bicycle parking locations within a radius
    ///
    /// Dist is default to 0.5 even if you provide `None`
    ///
    /// **Update freq**: Monthly
    fn get_bike_parking(
        client: &C,
        lat: f64,
        long: f64,
        dist: impl Into<Option<f64>>,
    ) -> LTAResult<Vec<BikeParking>>;
}