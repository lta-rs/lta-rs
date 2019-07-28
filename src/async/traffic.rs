use reqwest::Error;
use tokio::prelude::Future;

use crate::r#async::lta_client::LTAClient;
use crate::traffic::*;
use crate::utils::commons::Client;

pub fn get_erp_rates(
    client: &LTAClient,
) -> impl Future<Item = Vec<erp_rates::ErpRate>, Error = Error> {
    let rb = client.get_req_builder(erp_rates::URL);
    rb.send()
        .and_then(|mut f| f.json::<erp_rates::ErpRatesResp>())
        .map(|r| r.value)
}

pub fn get_carkpark_avail(
    client: &LTAClient,
) -> impl Future<Item = Vec<carpark_avail::Carpark>, Error = Error> {
    let rb = client.get_req_builder(carpark_avail::URL);
    rb.send()
        .and_then(|mut f| f.json::<carpark_avail::CarparkAvailResp>())
        .map(|r| r.value)
}

pub fn get_est_travel_time(
    client: &LTAClient,
) -> impl Future<Item = Vec<est_travel_time::EstTravelTime>, Error = Error> {
    let rb = client.get_req_builder(est_travel_time::URL);
    rb.send()
        .and_then(|mut f| f.json::<est_travel_time::EstTravelTimeResp>())
        .map(|r| r.value)
}

pub fn get_faulty_traffic_lights(
    client: &LTAClient,
) -> impl Future<Item = Vec<faulty_traffic_lights::FaultyTrafficLight>, Error = Error> {
    let rb = client.get_req_builder(faulty_traffic_lights::URL);
    rb.send()
        .and_then(|mut f| f.json::<faulty_traffic_lights::FaultyTrafficLightResp>())
        .map(|r| r.value)
}

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

pub fn get_traffic_images(
    client: &LTAClient,
) -> impl Future<Item = Vec<traffic_images::TrafficImage>, Error = Error> {
    let rb = client.get_req_builder(traffic_images::URL);
    rb.send()
        .and_then(|mut f| f.json::<traffic_images::TrafficImageResp>())
        .map(|r| r.value)
}

pub fn get_traffic_incidents(
    client: &LTAClient,
) -> impl Future<Item = Vec<traffic_incidents::TrafficIncident>, Error = Error> {
    let rb = client.get_req_builder(traffic_incidents::URL);
    rb.send()
        .and_then(|mut f| f.json::<traffic_incidents::TrafficIncidentResp>())
        .map(|r| r.value)
}

pub fn get_traffic_speed_band(
    client: &LTAClient,
) -> impl Future<Item = Vec<traffic_speed_bands::TrafficSpeedBand>, Error = Error> {
    let rb = client.get_req_builder(traffic_speed_bands::URL);
    rb.send()
        .and_then(|mut f| f.json::<traffic_speed_bands::TrafficSpeedBandResp>())
        .map(|r| r.value)
}

pub fn get_vms_emas(client: &LTAClient) -> impl Future<Item = Vec<vms_emas::VMS>, Error = Error> {
    let rb = client.get_req_builder(vms_emas::URL);
    rb.send()
        .and_then(|mut f| f.json::<vms_emas::VMSResp>())
        .map(|r| r.value)
}

pub fn get_bike_parking(
    client: &LTAClient,
) -> impl Future<Item = Vec<bike_parking::BikeParking>, Error = Error> {
    let rb = client.get_req_builder(bike_parking::URL);
    rb.send()
        .and_then(|mut f| f.json::<bike_parking::BikeParkingResp>())
        .map(|r| r.value)
}
