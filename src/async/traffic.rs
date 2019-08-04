//! All API pertaining to traffic related data
use futures::Future;
use reqwest::Error;

use crate::r#async::lta_client::LTAClient;
use crate::traffic::*;
use crate::utils::commons::Client;

/// Returns ERP rates of all vehicle types across all timings for each
/// zone.
///
/// **Update freq**: Ad-Hoc
pub fn get_erp_rates(
    client: &LTAClient,
) -> impl Future<Item = Vec<erp_rates::ErpRate>, Error = Error> {
    let rb = client.get_req_builder(erp_rates::URL);
    rb.send()
        .and_then(|mut f| f.json::<erp_rates::ErpRatesResp>())
        .map(|r| r.value)
}

/// Returns no. of available lots for HDB, LTA and URA carpark data.
/// The LTA carpark data consist of major shopping malls and developments within
/// Orchard, Marina, HarbourFront, Jurong Lake District.
/// (Note: list of LTA carpark data available on this API is subset of those listed on
/// One.Motoring and MyTransport Portals)
///
/// **Update freq**: 1 min
pub fn get_carkpark_avail(
    client: &LTAClient,
) -> impl Future<Item = Vec<carpark_avail::Carpark>, Error = Error> {
    let rb = client.get_req_builder(carpark_avail::URL);
    rb.send()
        .and_then(|mut f| f.json::<carpark_avail::CarparkAvailResp>())
        .map(|r| r.value)
}

/// Returns estimated travel times of expressways (in segments).
///
/// **Update freq**: 5min
pub fn get_est_travel_time(
    client: &LTAClient,
) -> impl Future<Item = Vec<est_travel_time::EstTravelTime>, Error = Error> {
    let rb = client.get_req_builder(est_travel_time::URL);
    rb.send()
        .and_then(|mut f| f.json::<est_travel_time::EstTravelTimeResp>())
        .map(|r| r.value)
}

/// Returns alerts of traffic lights that are currently faulty, or currently
/// undergoing scheduled maintenance.
///
/// **Update freq**: 2min or whenever there are updates
pub fn get_faulty_traffic_lights(
    client: &LTAClient,
) -> impl Future<Item = Vec<faulty_traffic_lights::FaultyTrafficLight>, Error = Error> {
    let rb = client.get_req_builder(faulty_traffic_lights::URL);
    rb.send()
        .and_then(|mut f| f.json::<faulty_traffic_lights::FaultyTrafficLightResp>())
        .map(|r| r.value)
}

/// Returns all planned road openings
///
/// **Update freq**: 24 hours – whenever there are updates
pub fn get_road_details(
    client: &LTAClient,
    road_details_type: road::RoadDetailsType,
) -> impl Future<Item = Vec<road::RoadDetails>, Error = Error> {
    let url = match road_details_type {
        road::RoadDetailsType::RoadOpening => road::URL_ROAD_OPENING,
        road::RoadDetailsType::RoadWorks => road::URL_ROAD_WORKS,
    };

    let rb = client.get_req_builder(url);
    rb.send()
        .and_then(|mut f| f.json::<road::RoadDetailsResp>())
        .map(|r| r.value)
}

/// Returns links to images of live traffic conditions along expressways and
/// Woodlands & Tuas Checkpoints.
///
/// **Update freq**: 1 to 5 minutes
pub fn get_traffic_images(
    client: &LTAClient,
) -> impl Future<Item = Vec<traffic_images::TrafficImage>, Error = Error> {
    let rb = client.get_req_builder(traffic_images::URL);
    rb.send()
        .and_then(|mut f| f.json::<traffic_images::TrafficImageResp>())
        .map(|r| r.value)
}

/// Returns current traffic speeds on expressways and arterial roads,
/// expressed in speed bands.
///
/// **Update freq**: 5 minutes
pub fn get_traffic_incidents(
    client: &LTAClient,
) -> impl Future<Item = Vec<traffic_incidents::TrafficIncident>, Error = Error> {
    let rb = client.get_req_builder(traffic_incidents::URL);
    rb.send()
        .and_then(|mut f| f.json::<traffic_incidents::TrafficIncidentResp>())
        .map(|r| r.value)
}

/// Returns current traffic speeds on expressways and arterial roads,
/// expressed in speed bands.
///
/// **Update freq**: 5 minutes
pub fn get_traffic_speed_band(
    client: &LTAClient,
) -> impl Future<Item = Vec<traffic_speed_bands::TrafficSpeedBand>, Error = Error> {
    let rb = client.get_req_builder(traffic_speed_bands::URL);
    rb.send()
        .and_then(|mut f| f.json::<traffic_speed_bands::TrafficSpeedBandResp>())
        .map(|r| r.value)
}

/// Returns traffic advisories (via variable message services) concerning
/// current traffic conditions that are displayed on EMAS signboards
/// along expressways and arterial roads.
///
/// **Update freq**: 2 minutes
pub fn get_vms_emas(client: &LTAClient) -> impl Future<Item = Vec<vms_emas::VMS>, Error = Error> {
    let rb = client.get_req_builder(vms_emas::URL);
    rb.send()
        .and_then(|mut f| f.json::<vms_emas::VMSResp>())
        .map(|r| r.value)
}

/// Returns bicycle parking locations within a radius. The default radius is
/// set as 0.5km
///
/// **Update freq**: Monthly
pub fn get_bike_parking(
    client: &LTAClient,
) -> impl Future<Item = Vec<bike_parking::BikeParking>, Error = Error> {
    let rb = client.get_req_builder(bike_parking::URL);
    rb.send()
        .and_then(|mut f| f.json::<bike_parking::BikeParkingResp>())
        .map(|r| r.value)
}
