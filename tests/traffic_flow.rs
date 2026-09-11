mod common;

use lta::operations::get_traffic_flow::{decode_get_traffic_flow_response, get_traffic_flow_parts};
use lta::{Api, GetTrafficFlowInput, GetTrafficFlowResponse};
use reqwest::Url;

#[test]
fn traffic_flow_request_has_no_pagination_parameter() {
    let request = get_traffic_flow_parts(GetTrafficFlowInput::new()).expect("build request parts");
    assert_eq!(request.method, http::Method::GET);
    assert_eq!(request.uri, "/TrafficFlow");

    let request = Api::new()
        .base_url("https://example.test/ltaodataservice/")
        .account_key("test-key")
        .traffic()
        .get_flow()
        .request()
        .expect("build authenticated request");
    assert_eq!(
        request.uri(),
        "https://example.test/ltaodataservice/TrafficFlow"
    );
    assert_eq!(request.headers()["AccountKey"], "test-key");
}

#[test]
fn traffic_flow_decodes_reference_fixture_as_links() {
    for (path, body) in common::json_fixtures("traffic_flow") {
        let wire = serde_json::from_slice::<serde_json::Value>(&body)
            .unwrap_or_else(|error| panic!("failed to parse {}: {error}", path.display()));

        let expected = wire["value"]
            .as_array()
            .expect("fixture value array")
            .iter()
            .map(|entry| Url::parse(entry["Link"].as_str().expect("fixture Link")).unwrap())
            .collect::<Vec<_>>();

        let response = satay_runtime::ResponseParts {
            status: http::StatusCode::OK,
            headers: http::HeaderMap::new(),
            body: body.as_ref(),
        };

        let decoded = decode_get_traffic_flow_response(response)
            .unwrap_or_else(|error| panic!("failed to decode {}: {error}", path.display()));

        let GetTrafficFlowResponse::Ok(links) = decoded else {
            panic!("expected a successful response for {}", path.display());
        };

        assert_eq!(links, expected);
        assert_eq!(links.len(), 1);
        // assert!(links[0].starts_with(
        //     "https://dmprod-datasets.s3.ap-southeast-1.amazonaws.com/traffic-flow/data/trafficflow.json"
        // ));
    }
}
