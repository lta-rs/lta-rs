use crate::api_url;
use crate::blocking::{build_req_with_query, build_req_with_skip, LTAClient};
use crate::models::traffic::prelude::*;
use crate::{Client, LTAError, LTAResult, Traffic};

pub trait TrafficRequests<C: Client> {
    /// Returns ERP rates of all vehicle types across all timings for each
    /// zone.
    ///
    /// **Update freq**: Ad-Hoc
    fn get_erp_rates(client: &C, skip: Option<u32>) -> LTAResult<Vec<ErpRate>>;

    /// Returns no. of available lots for HDB, LTA and URA carpark data.
    /// The LTA carpark data consist of major shopping malls and developments within
    /// Orchard, Marina, HarbourFront, Jurong Lake District.
    /// (Note: list of LTA carpark data available on this API is subset of those listed on
    /// One.Motoring and MyTransport Portals)
    ///
    /// **Update freq**: 1 min
    fn get_carpark_avail(client: &C, skip: Option<u32>) -> LTAResult<Vec<CarPark>>;

    /// Returns estimated travel times of expressways (in segments).
    ///
    /// **Update freq**: 5min
    fn get_est_travel_time(client: &C, skip: Option<u32>) -> LTAResult<Vec<EstTravelTime>>;

    /// Returns alerts of traffic lights that are currently faulty, or currently
    /// undergoing scheduled maintenance.
    ///
    /// **Update freq**: 2min or whenever there are updates
    fn get_faulty_traffic_lights(
        client: &C,
        skip: Option<u32>,
    ) -> LTAResult<Vec<FaultyTrafficLight>>;

    /// Returns all planned road openings or road works depending on the `RoadDetailsType` supplied
    ///
    /// **Update freq**: 24 hours – whenever there are updates
    fn get_road_details(
        client: &C,
        road_details_type: RoadDetailsType,
        skip: Option<u32>,
    ) -> LTAResult<Vec<RoadDetails>>;

    /// Returns current traffic speeds on expressways and arterial roads,
    /// expressed in speed bands.
    ///
    /// **Update freq**: 5 minutes
    fn get_traffic_speed_band(client: &C, skip: Option<u32>) -> LTAResult<Vec<TrafficSpeedBand>>;

    /// Returns links to images of live traffic conditions along expressways and
    /// Woodlands & Tuas Checkpoints.
    ///
    /// **Update freq**: 1 to 5 minutes
    fn get_traffic_images(client: &C, skip: Option<u32>) -> LTAResult<Vec<TrafficImage>>;

    /// Returns current traffic speeds on expressways and arterial roads,
    /// expressed in speed bands.
    ///
    /// **Update freq**: 5 minutes
    fn get_traffic_incidents(client: &C, skip: Option<u32>) -> LTAResult<Vec<TrafficIncident>>;

    /// Returns traffic advisories (via variable message services) concerning
    /// current traffic conditions that are displayed on EMAS signboards
    /// along expressways and arterial roads.
    ///
    /// **Update freq**: 2 minutes
    fn get_vms_emas(client: &C, skip: Option<u32>) -> LTAResult<Vec<Vms>>;

    /// Returns bicycle parking locations within a radius
    ///
    /// Dist is default to 0.5 even if you provide `None`
    ///
    /// **Update freq**: Monthly
    fn get_bike_parking(
        client: &C,
        lat: f64,
        long: f64,
        dist: Option<f64>,
    ) -> LTAResult<Vec<BikeParking>>;
}

impl TrafficRequests<LTAClient> for Traffic {
    fn get_erp_rates(client: &LTAClient, skip: Option<u32>) -> LTAResult<Vec<ErpRate>> {
        build_req_with_skip::<ErpRatesResp, _, _>(client, api_url!("/ERPRates"), skip)
    }

    fn get_carpark_avail(client: &LTAClient, skip: Option<u32>) -> LTAResult<Vec<CarPark>> {
        build_req_with_skip::<CarparkAvailResp, _, _>(
            client,
            api_url!("/CarParkAvailabilityv2"),
            skip,
        )
    }

    fn get_est_travel_time(client: &LTAClient, skip: Option<u32>) -> LTAResult<Vec<EstTravelTime>> {
        build_req_with_skip::<EstTravelTimeResp, _, _>(client, api_url!("/EstTravelTimes"), skip)
    }

    fn get_faulty_traffic_lights(
        client: &LTAClient,
        skip: Option<u32>,
    ) -> LTAResult<Vec<FaultyTrafficLight>> {
        build_req_with_skip::<FaultyTrafficLightResp, _, _>(
            client,
            api_url!("/FaultyTrafficLights"),
            skip,
        )
    }

    fn get_road_details(
        client: &LTAClient,
        road_details_type: RoadDetailsType,
        skip: Option<u32>,
    ) -> LTAResult<Vec<RoadDetails>> {
        let url = match road_details_type {
            RoadDetailsType::RoadOpening => api_url!("/RoadOpenings"),
            RoadDetailsType::RoadWorks => api_url!("/RoadWorks"),
            _ => return Err(LTAError::UnknownEnumVariant),
        };

        build_req_with_skip::<RoadDetailsResp, _, _>(client, url, skip)
    }

    fn get_traffic_speed_band(
        client: &LTAClient,
        skip: Option<u32>,
    ) -> LTAResult<Vec<TrafficSpeedBand>> {
        build_req_with_skip::<TrafficSpeedBandResp, _, _>(
            client,
            api_url!("/TrafficSpeedBandsv2"),
            skip,
        )
    }

    fn get_traffic_images(client: &LTAClient, skip: Option<u32>) -> LTAResult<Vec<TrafficImage>> {
        build_req_with_skip::<TrafficImageResp, _, _>(client, api_url!("/Traffic-Imagesv2"), skip)
    }

    fn get_traffic_incidents(
        client: &LTAClient,
        skip: Option<u32>,
    ) -> LTAResult<Vec<TrafficIncident>> {
        build_req_with_skip::<TrafficIncidentResp, _, _>(
            client,
            api_url!("/TrafficIncidents"),
            skip,
        )
    }

    fn get_vms_emas(client: &LTAClient, skip: Option<u32>) -> LTAResult<Vec<Vms>> {
        build_req_with_skip::<VMSResp, _, _>(client, api_url!("/VMS"), skip)
    }

    fn get_bike_parking(
        client: &LTAClient,
        lat: f64,
        long: f64,
        dist: Option<f64>,
    ) -> LTAResult<Vec<BikeParking>> {
        let unwrapped_dist = dist.unwrap_or(0.5);
        build_req_with_query::<BikeParkingResp, _, _, _>(
            client,
            api_url!("/BicycleParkingv2"),
            |rb| rb.query(&[("Lat", lat), ("Long", long), ("Dist", unwrapped_dist)]),
        )
    }
}
