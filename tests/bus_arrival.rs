mod common;

use lta::GetBusArrivalResponse;
use lta::operations::get_bus_arrival::decode_get_bus_arrival_response;

#[test]
fn bus_arrival_decodes_every_vendored_fixture() {
    for (path, body) in common::json_fixtures("bus_arrival") {
        let response = satay_runtime::ResponseParts {
            status: http::StatusCode::OK,
            headers: http::HeaderMap::new(),
            body,
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
            assert_eq!(arrival.bus_stop_code, 83139);
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
fn legacy_bus_arrival_fixture_records_missing_monitored_incompatibility() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/bus_arrival/legacy/bus_arrival_legacy_0.json"
    );
    let body = std::fs::read(path).expect("read legacy Bus Arrival fixture");
    assert!(
        !body
            .windows(b"Monitored".len())
            .any(|window| window == b"Monitored"),
        "legacy fixture unexpectedly contains the v3 Monitored field"
    );

    let response = satay_runtime::ResponseParts {
        status: http::StatusCode::OK,
        headers: http::HeaderMap::new(),
        body,
    };
    let decoded =
        decode_get_bus_arrival_response(response).expect("decode legacy response envelope");
    let GetBusArrivalResponse::Ok(arrival) = decoded else {
        panic!("expected a successful legacy response envelope");
    };

    assert_eq!(arrival.services.len(), 11);
    assert!(
        arrival.services.iter().all(|service| {
            service.next_bus.is_none() && service.next_bus2.is_none() && service.next_bus3.is_none()
        }),
        "v2 timings without Monitored must not deserialize as valid v3 timings"
    );
}
