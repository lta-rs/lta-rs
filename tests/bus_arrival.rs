mod common;

use lta::operations::get_bus_arrival::{decode_get_bus_arrival_response, get_bus_arrival_parts};
use lta::{BusStopCode, GetBusArrivalInput, GetBusArrivalResponse};

#[test]
fn bus_arrival_decodes_every_vendored_fixture() {
    for (path, body) in common::json_fixtures("bus_arrival") {
        let response = satay_runtime::ResponseParts {
            status: http::StatusCode::OK,
            headers: http::HeaderMap::new(),
            body: body.as_ref(),
        };
        let decoded = decode_get_bus_arrival_response(response)
            .unwrap_or_else(|error| panic!("failed to decode {}: {error}", path.display()));
        let GetBusArrivalResponse::Ok(arrival) = decoded else {
            panic!("expected a successful response for {}", path.display());
        };

        if path
            .file_name()
            .is_some_and(|name| name == "bus_arrival_0.json")
        {
            assert_eq!(arrival.bus_stop_code.as_ref(), "83139");
            assert_eq!(arrival.services.len(), 1);
            let service = &arrival.services[0];
            assert_eq!(service.service_no.as_ref(), "15");
            for timing in [&service.next_bus, &service.next_bus2, &service.next_bus3] {
                let timing = timing.as_ref().expect("v3 timing should deserialize");
                assert!(timing.monitored);
                assert!(timing.latitude.is_finite());
                assert!(timing.longitude.is_finite());
            }
        }
    }
}

#[test]
fn bus_arrival_validates_and_preserves_fixed_width_stop_codes() {
    let bus_stop_code = BusStopCode::try_from("01012").expect("valid bus stop code");
    let request = get_bus_arrival_parts(GetBusArrivalInput::new(bus_stop_code))
        .expect("build Bus Arrival request");
    assert_eq!(request.uri, "/v3/BusArrival?BusStopCode=01012");

    for malformed in ["1012", "001012", "01A12"] {
        assert!(
            BusStopCode::try_from(malformed).is_err(),
            "{malformed} accepted as a bus stop code"
        );
    }
}
