use assert_float_eq::assert_float_absolute_eq;
use lta::operations::get_bus_stops::{decode_get_bus_stops_response, get_bus_stops_parts};
use lta::{Api, GetBusStopsInput, GetBusStopsResponse};

#[test]
fn bus_stops_request_preserves_optional_skip() {
    let request = get_bus_stops_parts(GetBusStopsInput::new()).expect("build request parts");
    assert_eq!(request.method, http::Method::GET);
    assert_eq!(request.uri, "/BusStops");

    let request =
        get_bus_stops_parts(GetBusStopsInput::new().skip(500)).expect("build paged request parts");
    assert_eq!(request.uri, "/BusStops?%24skip=500");

    let request = Api::new()
        .base_url("https://example.test/ltaodataservice/")
        .account_key("test-key")
        .bus()
        .get_stops()
        .skip(500)
        .request()
        .expect("build authenticated request");
    assert_eq!(
        request.uri(),
        "https://example.test/ltaodataservice/BusStops?%24skip=500"
    );
    assert_eq!(request.headers()["AccountKey"], "test-key");
}

#[test]
fn bus_stops_response_projects_fixture_into_reference_fields() {
    let response = satay_runtime::ResponseParts {
        status: http::StatusCode::OK,
        headers: http::HeaderMap::new(),
        body: br#"{
            "odata.metadata": "https://datamall2.mytransport.sg/ltaodataservice/$metadata#BusStops",
            "value": [{
                "BusStopCode": "01012",
                "RoadName": "Victoria St",
                "Description": "Hotel Grand Pacific",
                "Latitude": 1.29684825487647,
                "Longitude": 103.85253591654006
            }]
        }"#,
    };

    let decoded = decode_get_bus_stops_response(response).expect("decode projected response");
    let GetBusStopsResponse::Ok(stops) = decoded else {
        panic!("expected successful Bus Stops response");
    };

    assert_eq!(stops.len(), 1);
    let stop = &stops[0];
    assert_eq!(stop.bus_stop_code, 1012);
    assert_eq!(stop.road_name, "Victoria St");
    assert_eq!(stop.desc, "Hotel Grand Pacific");
    assert_float_absolute_eq!(stop.lat, 1.296_848_254_876_47);
    assert_float_absolute_eq!(stop.long, 103.852_535_916_540_06);
}
