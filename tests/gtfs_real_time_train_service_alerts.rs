mod common;

use lta::operations::get_gtfs_real_time_train_service_alerts::{
    decode_get_gtfs_real_time_train_service_alerts_response,
    get_gtfs_real_time_train_service_alerts_parts,
};
use lta::{Api, GetGtfsRealTimeTrainServiceAlertsInput, GetGtfsRealTimeTrainServiceAlertsResponse};
use reqwest::Url;

#[test]
fn gtfs_real_time_train_service_alerts_request_has_exact_path_and_no_query() {
    let request = get_gtfs_real_time_train_service_alerts_parts(
        GetGtfsRealTimeTrainServiceAlertsInput::new(),
    )
    .expect("build request parts");
    assert_eq!(request.method, http::Method::GET);
    assert_eq!(request.uri, "/GTFSRealTimeTrainServiceAlerts");
    assert!(
        !request.uri.contains('?'),
        "GTFSRealTimeTrainServiceAlerts must not send query parameters, got {}",
        request.uri
    );

    let request = Api::new()
        .base_url("https://example.test/ltaodataservice/")
        .account_key("test-key")
        .train()
        .get_gtfs_real_time_service_alerts()
        .request()
        .expect("build authenticated request");
    assert_eq!(
        request.uri(),
        "https://example.test/ltaodataservice/GTFSRealTimeTrainServiceAlerts"
    );
    assert!(
        !request.uri().to_string().contains('?'),
        "authenticated GTFSRealTimeTrainServiceAlerts request must not contain a query string"
    );
    assert_eq!(request.headers()["AccountKey"], "test-key");
}

#[test]
fn gtfs_real_time_train_service_alerts_decodes_reference_fixture_as_links() {
    for (path, body) in common::json_fixtures("gtfs_real_time_train_service_alerts") {
        let wire = serde_json::from_slice::<serde_json::Value>(&body)
            .unwrap_or_else(|error| panic!("failed to parse {}: {error}", path.display()));

        let expected = wire["value"]
            .as_array()
            .expect("fixture value array")
            .iter()
            .map(|entry| Url::parse(entry["link"].as_str().expect("fixture link")).unwrap())
            .collect::<Vec<_>>();

        let response = satay_runtime::ResponseParts {
            status: http::StatusCode::OK,
            headers: http::HeaderMap::new(),
            body: body.as_ref(),
        };

        let decoded = decode_get_gtfs_real_time_train_service_alerts_response(response)
            .unwrap_or_else(|error| panic!("failed to decode {}: {error}", path.display()));

        let GetGtfsRealTimeTrainServiceAlertsResponse::Ok(links) = decoded else {
            panic!("expected a successful response for {}", path.display());
        };

        assert_eq!(links, expected);
        assert_eq!(links.len(), 1);
        // assert!(
        //     links[0].starts_with(
        //         "https://dmprod-datasets.s3.ap-southeast-1.amazonaws.com/train-gtfs-real-time/gtfs_realtime.pb"
        //     )
        // );
    }
}
