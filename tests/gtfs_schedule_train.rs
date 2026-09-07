mod common;

use lta::operations::get_gtfs_schedule_train::{
    decode_get_gtfs_schedule_train_response, get_gtfs_schedule_train_parts,
};
use lta::{Api, GetGtfsScheduleTrainInput, GetGtfsScheduleTrainResponse};

#[test]
fn gtfs_schedule_train_request_has_exact_path_and_no_query() {
    let request = get_gtfs_schedule_train_parts(GetGtfsScheduleTrainInput::new())
        .expect("build request parts");
    assert_eq!(request.method, http::Method::GET);
    assert_eq!(request.uri, "/GTFSScheduleTrain");
    assert!(
        !request.uri.contains('?'),
        "GTFSScheduleTrain must not send query parameters, got {}",
        request.uri
    );

    let request = Api::new()
        .base_url("https://example.test/ltaodataservice/")
        .account_key("test-key")
        .train()
        .get_gtfs_schedule()
        .request()
        .expect("build authenticated request");
    assert_eq!(
        request.uri(),
        "https://example.test/ltaodataservice/GTFSScheduleTrain"
    );
    assert!(
        !request.uri().to_string().contains('?'),
        "authenticated GTFSScheduleTrain request must not contain a query string"
    );
    assert_eq!(request.headers()["AccountKey"], "test-key");
}

#[test]
fn gtfs_schedule_train_decodes_reference_fixture_as_links() {
    for (path, body) in common::json_fixtures("gtfs_schedule_train") {
        let wire = serde_json::from_slice::<serde_json::Value>(&body)
            .unwrap_or_else(|error| panic!("failed to parse {}: {error}", path.display()));
        let expected = wire["value"]
            .as_array()
            .expect("fixture value array")
            .iter()
            .map(|entry| entry["link"].as_str().expect("fixture link").to_owned())
            .collect::<Vec<_>>();

        let response = satay_runtime::ResponseParts {
            status: http::StatusCode::OK,
            headers: http::HeaderMap::new(),
            body,
        };
        let decoded = decode_get_gtfs_schedule_train_response(response)
            .unwrap_or_else(|error| panic!("failed to decode {}: {error}", path.display()));
        let GetGtfsScheduleTrainResponse::Ok(links) = decoded else {
            panic!("expected a successful response for {}", path.display());
        };

        assert_eq!(links, expected);
        assert_eq!(links.len(), 1);
        assert!(
            links[0].starts_with(
                "https://dmprod-datasets.s3.ap-southeast-1.amazonaws.com/train-gtfs-schedule/gtfs_schedule.zip"
            )
        );
    }
}
