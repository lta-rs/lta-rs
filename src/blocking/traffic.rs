use crate::api_url;
use crate::blocking::{build_req_with_query, build_req_with_skip, LTAClient};
use crate::models::traffic::prelude::*;
use crate::{Client, LTAError, LTAResult, Traffic};

pub trait TrafficRequests<C: Client> {
    /// Returns ERP rates of all vehicle types across all timings for each
    /// zone.
    ///
    /// **Update freq**: Ad-Hoc
    fn get_erp_rates<S>(client: &C, skip: S) -> LTAResult<Vec<ErpRate>>
    where
        S: Into<Option<u32>>;

    /// Returns no. of available lots for HDB, LTA and URA carpark data.
    /// The LTA carpark data consist of major shopping malls and developments within
    /// Orchard, Marina, HarbourFront, Jurong Lake District.
    /// (Note: list of LTA carpark data available on this API is subset of those listed on
    /// One.Motoring and MyTransport Portals)
    ///
    /// **Update freq**: 1 min
    fn get_carpark_avail<S>(client: &C, skip: S) -> LTAResult<Vec<CarPark>>
    where
        S: Into<Option<u32>>;

    /// Returns estimated travel times of expressways (in segments).
    ///
    /// **Update freq**: 5min
    fn get_est_travel_time<S>(client: &C, skip: S) -> LTAResult<Vec<EstTravelTime>>
    where
        S: Into<Option<u32>>;

    /// Returns alerts of traffic lights that are currently faulty, or currently
    /// undergoing scheduled maintenance.
    ///
    /// **Update freq**: 2min or whenever there are updates
    fn get_faulty_traffic_lights<S>(client: &C, skip: S) -> LTAResult<Vec<FaultyTrafficLight>>
    where
        S: Into<Option<u32>>;

    /// Returns all planned road openings or road works depending on the `RoadDetailsType` supplied
    ///
    /// **Update freq**: 24 hours – whenever there are updates
    fn get_road_details<S>(
        client: &C,
        road_details_type: RoadDetailsType,
        skip: S,
    ) -> LTAResult<Vec<RoadDetails>>
    where
        S: Into<Option<u32>>;

    /// Returns current traffic speeds on expressways and arterial roads,
    /// expressed in speed bands.
    ///
    /// **Update freq**: 5 minutes
    fn get_traffic_speed_band<S>(client: &C, skip: S) -> LTAResult<Vec<TrafficSpeedBand>>
    where
        S: Into<Option<u32>>;

    /// Returns links to images of live traffic conditions along expressways and
    /// Woodlands & Tuas Checkpoints.
    ///
    /// **Update freq**: 1 to 5 minutes
    fn get_traffic_images<S>(client: &C, skip: S) -> LTAResult<Vec<TrafficImage>>
    where
        S: Into<Option<u32>>;

    /// Returns current traffic speeds on expressways and arterial roads,
    /// expressed in speed bands.
    ///
    /// **Update freq**: 5 minutes
    fn get_traffic_incidents<S>(client: &C, skip: S) -> LTAResult<Vec<TrafficIncident>>
    where
        S: Into<Option<u32>>;

    /// Returns traffic advisories (via variable message services) concerning
    /// current traffic conditions that are displayed on EMAS signboards
    /// along expressways and arterial roads.
    ///
    /// **Update freq**: 2 minutes
    fn get_vms_emas<S>(client: &C, skip: S) -> LTAResult<Vec<Vms>>
    where
        S: Into<Option<u32>>;

    /// Returns bicycle parking locations within a radius
    ///
    /// Dist is default to 0.5 even if you provide `None`
    ///
    /// **Update freq**: Monthly
    fn get_bike_parking<D>(client: &C, lat: f64, long: f64, dist: D) -> LTAResult<Vec<BikeParking>>
    where
        D: Into<Option<f64>>;
}

impl TrafficRequests<LTAClient> for Traffic {
    fn get_erp_rates<S>(client: &LTAClient, skip: S) -> LTAResult<Vec<ErpRate>>
    where
        S: Into<Option<u32>>,
    {
        build_req_with_skip::<ErpRatesResp, _, _>(client, api_url!("/ERPRates"), skip.into())
    }

    fn get_carpark_avail<S>(client: &LTAClient, skip: S) -> LTAResult<Vec<CarPark>>
    where
        S: Into<Option<u32>>,
    {
        build_req_with_skip::<CarparkAvailResp, _, _>(
            client,
            api_url!("/CarParkAvailabilityv2"),
            skip.into(),
        )
    }

    fn get_est_travel_time<S>(client: &LTAClient, skip: S) -> LTAResult<Vec<EstTravelTime>>
    where
        S: Into<Option<u32>>,
    {
        build_req_with_skip::<EstTravelTimeResp, _, _>(
            client,
            api_url!("/EstTravelTimes"),
            skip.into(),
        )
    }

    fn get_faulty_traffic_lights<S>(
        client: &LTAClient,
        skip: S,
    ) -> LTAResult<Vec<FaultyTrafficLight>>
    where
        S: Into<Option<u32>>,
    {
        build_req_with_skip::<FaultyTrafficLightResp, _, _>(
            client,
            api_url!("/FaultyTrafficLights"),
            skip.into(),
        )
    }

    fn get_road_details<S>(
        client: &LTAClient,
        road_details_type: RoadDetailsType,
        skip: S,
    ) -> LTAResult<Vec<RoadDetails>>
    where
        S: Into<Option<u32>>,
    {
        let url = match road_details_type {
            RoadDetailsType::RoadOpening => api_url!("/RoadOpenings"),
            RoadDetailsType::RoadWorks => api_url!("/RoadWorks"),
            _ => return Err(LTAError::UnknownEnumVariant),
        };

        build_req_with_skip::<RoadDetailsResp, _, _>(client, url, skip.into())
    }

    fn get_traffic_speed_band<S>(client: &LTAClient, skip: S) -> LTAResult<Vec<TrafficSpeedBand>>
    where
        S: Into<Option<u32>>,
    {
        build_req_with_skip::<TrafficSpeedBandResp, _, _>(
            client,
            api_url!("/TrafficSpeedBandsv2"),
            skip.into(),
        )
    }

    fn get_traffic_images<S>(client: &LTAClient, skip: S) -> LTAResult<Vec<TrafficImage>>
    where
        S: Into<Option<u32>>,
    {
        build_req_with_skip::<TrafficImageResp, _, _>(
            client,
            api_url!("/Traffic-Imagesv2"),
            skip.into(),
        )
    }

    fn get_traffic_incidents<S>(client: &LTAClient, skip: S) -> LTAResult<Vec<TrafficIncident>>
    where
        S: Into<Option<u32>>,
    {
        build_req_with_skip::<TrafficIncidentResp, _, _>(
            client,
            api_url!("/TrafficIncidents"),
            skip.into(),
        )
    }

    fn get_vms_emas<S>(client: &LTAClient, skip: S) -> LTAResult<Vec<Vms>>
    where
        S: Into<Option<u32>>,
    {
        build_req_with_skip::<VMSResp, _, _>(client, api_url!("/VMS"), skip.into())
    }

    fn get_bike_parking<D>(
        client: &LTAClient,
        lat: f64,
        long: f64,
        dist: D,
    ) -> LTAResult<Vec<BikeParking>>
    where
        D: Into<Option<f64>>,
    {
        let unwrapped_dist = dist.into().unwrap_or(0.5);
        build_req_with_query::<BikeParkingResp, _, _, _>(
            client,
            api_url!("/BicycleParkingv2"),
            |rb| rb.query(&[("Lat", lat), ("Long", long), ("Dist", unwrapped_dist)]),
        )
    }
}
