mod common;

use lta::operations::get_ev_charging_points_batch::{
    decode_get_ev_charging_points_batch_response, get_ev_charging_points_batch_parts,
};
use lta::{Api, GetEvChargingPointsBatchInput, GetEvChargingPointsBatchResponse};

#[test]
fn ev_charging_points_batch_request_has_exact_path_and_no_query() {
    let request = get_ev_charging_points_batch_parts(GetEvChargingPointsBatchInput::new())
        .expect("build request parts");
    assert_eq!(request.method, http::Method::GET);
    assert_eq!(request.uri, "/EVCBatch");
    assert!(
        !request.uri.contains('?'),
        "EVCBatch must not send query parameters, got {}",
        request.uri
    );

    let request = Api::new()
        .base_url("https://example.test/ltaodataservice/")
        .account_key("test-key")
        .traffic()
        .get_ev_charging_points_batch()
        .request()
        .expect("build authenticated request");
    assert_eq!(
        request.uri(),
        "https://example.test/ltaodataservice/EVCBatch"
    );
    assert!(
        !request.uri().to_string().contains('?'),
        "authenticated EVCBatch request must not contain a query string"
    );
    assert_eq!(request.headers()["AccountKey"], "test-key");
}

#[test]
fn ev_charging_points_batch_decodes_reference_fixture_as_links() {
    for (path, body) in common::json_fixtures("ev_charging_points_batch") {
        let wire = serde_json::from_slice::<serde_json::Value>(&body)
            .unwrap_or_else(|error| panic!("failed to parse {}: {error}", path.display()));
        let expected = wire["value"]
            .as_array()
            .expect("fixture value array")
            .iter()
            .map(|entry| entry["Link"].as_str().expect("fixture Link").to_owned())
            .collect::<Vec<_>>();

        let response = satay_runtime::ResponseParts {
            status: http::StatusCode::OK,
            headers: http::HeaderMap::new(),
            body,
        };
        let decoded = decode_get_ev_charging_points_batch_response(response)
            .unwrap_or_else(|error| panic!("failed to decode {}: {error}", path.display()));
        let GetEvChargingPointsBatchResponse::Ok(links) = decoded else {
            panic!("expected a successful response for {}", path.display());
        };

        assert_eq!(links, expected);
        assert_eq!(links.len(), 1);
        assert!(
            links[0]
                .starts_with("https://dmprod-datasets.s3.ap-southeast-1.amazonaws.com/ev-batch/")
        );
    }
}
