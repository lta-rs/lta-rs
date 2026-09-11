mod common;

use std::env;
use std::time::Duration;

use assert_float_eq::assert_float_absolute_eq;
use lta::operations::get_bus_stops::{decode_get_bus_stops_response, get_bus_stops_parts};
use lta::{Api, BusStopCode, GetBusStopsInput, GetBusStopsResponse};
use satay_reqwest::ReqwestActionExt;

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
fn bus_stops_request_serializes_bus_stop_code_with_leading_zeroes() {
    let bus_stop_code = BusStopCode::try_from("01012").expect("valid bus stop code");

    let request = get_bus_stops_parts(GetBusStopsInput::new().bus_stop_code(bus_stop_code.clone()))
        .expect("build filtered request parts");
    assert_eq!(request.uri, "/BusStops?BusStopCode=01012");

    let request = get_bus_stops_parts(
        GetBusStopsInput::new()
            .bus_stop_code(bus_stop_code.clone())
            .skip(500),
    )
    .expect("build filtered and paged request parts");
    assert_eq!(request.uri, "/BusStops?%24skip=500&BusStopCode=01012");

    let request = Api::new()
        .base_url("https://example.test/ltaodataservice/")
        .account_key("test-key")
        .bus()
        .get_stops()
        .bus_stop_code(bus_stop_code)
        .request()
        .expect("build authenticated request");
    assert_eq!(
        request.uri(),
        "https://example.test/ltaodataservice/BusStops?BusStopCode=01012"
    );
    assert_eq!(request.headers()["AccountKey"], "test-key");
}

#[test]
fn bus_stops_response_projects_fixture_into_reference_fields() {
    let response = satay_runtime::ResponseParts {
        status: http::StatusCode::OK,
        headers: http::HeaderMap::new(),
        body: &br#"{
            "odata.metadata": "https://datamall2.mytransport.sg/ltaodataservice/$metadata#BusStops",
            "value": [{
                "BusStopCode": "01012",
                "RoadName": "Victoria St",
                "Description": "Hotel Grand Pacific",
                "Latitude": 1.29684825487647,
                "Longitude": 103.85253591654006
            }]
        }"#[..],
    };

    let decoded = decode_get_bus_stops_response(response).expect("decode projected response");

    let GetBusStopsResponse::<Box<_>>::Ok(stops) = decoded else {
        panic!("expected successful Bus Stops response");
    };

    assert_eq!(stops.len(), 1);
    let stop = &stops[0];
    assert_eq!(stop.bus_stop_code.as_ref(), "01012");
    assert_eq!(stop.road_name.as_ref(), "Victoria St");
    assert_eq!(stop.desc.as_ref(), "Hotel Grand Pacific");
    assert_float_absolute_eq!(*stop.lat, 1.296_848_254_876_47);
    assert_float_absolute_eq!(*stop.long, 103.852_535_916_540_06);
}

#[test]
fn bus_stops_decodes_every_vendored_fixture() {
    for (path, body) in common::json_fixtures("bus_stops") {
        let response = satay_runtime::ResponseParts {
            status: http::StatusCode::OK,
            headers: http::HeaderMap::new(),
            body: body.as_ref(),
        };

        let decoded = decode_get_bus_stops_response(response)
            .unwrap_or_else(|error| panic!("failed to decode {}: {error}", path.display()));

        let GetBusStopsResponse::<Box<_>>::Ok(stops) = decoded else {
            panic!("expected a successful response for {}", path.display());
        };

        for stop in stops {
            assert!(
                !stop.road_name.is_empty(),
                "empty road name in {}",
                path.display()
            );
            assert!(
                (*stop.lat).is_finite(),
                "invalid latitude in {}",
                path.display()
            );
            assert!(
                (*stop.long).is_finite(),
                "invalid longitude in {}",
                path.display()
            );
        }
    }
}
#[tokio::test]
async fn bus_stops_downloads_and_decodes_live_fixture() {
    let Ok(account_key) = env::var("LTA_ACCOUNT_KEY") else {
        eprintln!("LTA_ACCOUNT_KEY unset; skipping live fixture download");
        return;
    };

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .expect("build reqwest client");

    let all = Api::new()
        .account_key(&account_key)
        .bus()
        .get_stops()
        .skip(0)
        .send_with(&client)
        .await
        .expect("fetch all bus stops");
    let GetBusStopsResponse::Ok(all_stops) = all else {
        panic!("expected a successful all-stops response");
    };
    assert!(
        all_stops.len() > 1,
        "expected more than one bus stop in the unfiltered fixture"
    );
    assert!(
        all_stops
            .iter()
            .any(|stop| stop.bus_stop_code.as_ref() == "01012"),
        "unfiltered fixture should contain stop 01012"
    );

    let bus_stop_code = BusStopCode::try_from("01012").expect("valid bus stop code");
    let filtered = Api::new()
        .account_key(&account_key)
        .bus()
        .get_stops()
        .bus_stop_code(bus_stop_code)
        .send_with(&client)
        .await
        .expect("fetch BusStopCode-filtered bus stops");
    let GetBusStopsResponse::Ok(filtered_stops) = filtered else {
        panic!("expected a successful filtered response");
    };
    assert!(!filtered_stops.is_empty(), "filtered fixture is empty");
    for stop in &filtered_stops {
        assert_eq!(
            stop.bus_stop_code.as_ref(),
            "01012",
            "filtered response leaked a different bus stop"
        );
    }
}
