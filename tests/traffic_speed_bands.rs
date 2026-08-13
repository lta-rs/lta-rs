mod common;

use assert_float_eq::assert_float_absolute_eq;
use lta::operations::get_traffic_speed_bands::{
    decode_get_traffic_speed_bands_response, get_traffic_speed_bands_parts,
};
use lta::{
    Api, GetTrafficSpeedBandsInput, GetTrafficSpeedBandsResponse, RoadCategory, TrafficSpeedBand,
};

#[test]
fn traffic_speed_bands_request_preserves_optional_skip() {
    let request = get_traffic_speed_bands_parts(GetTrafficSpeedBandsInput::new())
        .expect("build request parts");
    assert_eq!(request.method, http::Method::GET);
    assert_eq!(request.uri, "/v3/TrafficSpeedBands");

    let request = get_traffic_speed_bands_parts(GetTrafficSpeedBandsInput::new().skip(500))
        .expect("build paged request parts");
    assert_eq!(request.uri, "/v3/TrafficSpeedBands?%24skip=500");

    let request = Api::new()
        .base_url("https://example.test/ltaodataservice/")
        .account_key("test-key")
        .traffic()
        .get_speed_bands()
        .skip(500)
        .request()
        .expect("build authenticated request");
    assert_eq!(
        request.uri(),
        "https://example.test/ltaodataservice/v3/TrafficSpeedBands?%24skip=500"
    );
    assert_eq!(request.headers()["AccountKey"], "test-key");
}

#[test]
fn traffic_speed_bands_decodes_every_vendored_fixture() {
    for (path, body) in common::json_fixtures("traffic_speed_bands") {
        let response = satay_runtime::ResponseParts {
            status: http::StatusCode::OK,
            headers: http::HeaderMap::new(),
            body,
        };

        let decoded = decode_get_traffic_speed_bands_response(response)
            .unwrap_or_else(|error| panic!("failed to decode {}: {error}", path.display()));
        let GetTrafficSpeedBandsResponse::Ok(bands) = decoded else {
            panic!("expected a successful response for {}", path.display());
        };

        for band in &bands {
            assert!(band.link_id > 0);
            assert!(!band.road_name.is_empty());
            assert!((1..=8).contains(&band.speed_band));
            assert!(band.min_speed <= band.max_speed);
            assert!(band.start_lon.is_finite());
            assert!(band.start_lat.is_finite());
            assert!(band.end_lon.is_finite());
            assert!(band.end_lat.is_finite());
            assert!(!matches!(band.road_category, RoadCategory::Other(_)));
        }

        if path
            .file_name()
            .is_some_and(|name| name == "traffic_speed_bands_0.json")
        {
            assert_eq!(bands.len(), 500);
            let first = &bands[0];
            assert_eq!(first.link_id, 103_000_000);
            assert_eq!(first.road_name, "KENT ROAD");
            assert_eq!(first.road_category, RoadCategory::SmallRoads);
            assert_eq!(first.speed_band, 7);
            assert_eq!(first.min_speed, 60);
            assert_eq!(first.max_speed, 69);
            assert_float_absolute_eq!(first.start_lon, 103.852_980_520_445_03);
            assert_float_absolute_eq!(first.start_lat, 1.317_014_237_656_002_3);
            assert_float_absolute_eq!(first.end_lon, 103.852_598_822_423_72);
            assert_float_absolute_eq!(first.end_lat, 1.316_684_002_866_307_6);
        }
    }
}

#[test]
fn traffic_speed_bands_response_projects_and_parses_wire_fields() {
    let response = satay_runtime::ResponseParts {
        status: http::StatusCode::OK,
        headers: http::HeaderMap::new(),
        body: br#"{
            "odata.metadata": "https://datamall2.mytransport.sg/ltaodataservice/$metadata#TrafficSpeedBands",
            "lastUpdatedTime": "2023-04-05 22:30:00",
            "value": [{
                "LinkID": "103000000",
                "RoadName": "KENT ROAD",
                "RoadCategory": "E",
                "SpeedBand": 7,
                "MinimumSpeed": "60",
                "MaximumSpeed": "69",
                "StartLon": "103.85298052044503",
                "StartLat": "1.3170142376560023",
                "EndLon": "103.85259882242372",
                "EndLat": "1.3166840028663076"
            }]
        }"#,
    };

    let decoded =
        decode_get_traffic_speed_bands_response(response).expect("decode projected response");
    let GetTrafficSpeedBandsResponse::Ok(bands) = decoded else {
        panic!("expected successful Traffic Speed Bands response");
    };

    assert_eq!(bands.len(), 1);
    let band = &bands[0];
    assert_eq!(band.link_id, 103_000_000);
    assert_eq!(band.road_category, RoadCategory::SmallRoads);
    assert_eq!(band.min_speed, 60);
    assert_eq!(band.max_speed, 69);
    assert_float_absolute_eq!(band.start_lon, 103.852_980_520_445_03);
    assert_float_absolute_eq!(band.start_lat, 1.317_014_237_656_002_3);
    assert_float_absolute_eq!(band.end_lon, 103.852_598_822_423_72);
    assert_float_absolute_eq!(band.end_lat, 1.316_684_002_866_307_6);
}

#[test]
fn traffic_speed_bands_map_every_road_category() {
    let cases = [
        ("A", RoadCategory::Expressway),
        ("B", RoadCategory::MajorArterialRoads),
        ("C", RoadCategory::ArterialRoads),
        ("D", RoadCategory::MinorArterialRoads),
        ("E", RoadCategory::SmallRoads),
        ("F", RoadCategory::SlipRoads),
        ("G", RoadCategory::NoCategoryInfoAvail),
    ];

    for (wire, expected) in cases {
        let decoded = serde_json::from_value::<RoadCategory>(serde_json::json!(wire))
            .unwrap_or_else(|error| panic!("failed to decode road category {wire}: {error}"));
        assert_eq!(decoded, expected);
    }
}

// Deviation from lta_models: its #[serde(other)] variant collapses every unknown
// category to the unit RoadCategory::Unknown. Satay open string enums preserve the
// unrecognized wire value in RoadCategory::Other(String). DataMall's canonical A-G
// values map to the same named public variants in both implementations.
#[test]
fn traffic_speed_bands_preserve_unknown_category_spelling() {
    let decoded = serde_json::from_value::<RoadCategory>(serde_json::json!("Z"))
        .expect("decode unknown road category");
    assert_eq!(decoded, RoadCategory::Other("Z".into()));
}

#[test]
fn traffic_speed_band_serializes_to_canonical_wire_shape() {
    let band = TrafficSpeedBand {
        link_id: 103_000_000,
        road_name: "KENT ROAD".into(),
        road_category: RoadCategory::SmallRoads,
        speed_band: 7,
        min_speed: 60,
        max_speed: 69,
        start_lon: 103.852_980_520_445_03,
        start_lat: 1.317_014_237_656_002_3,
        end_lon: 103.852_598_822_423_72,
        end_lat: 1.316_684_002_866_307_6,
    };

    let serialized = serde_json::to_value(band).expect("serialize traffic speed band");
    let object = serialized
        .as_object()
        .expect("traffic speed band JSON object");

    assert_eq!(object.len(), 10);
    assert_eq!(object["LinkID"], serde_json::json!("103000000"));
    assert_eq!(object["RoadName"], serde_json::json!("KENT ROAD"));
    assert_eq!(object["RoadCategory"], serde_json::json!("E"));
    assert_eq!(object["SpeedBand"], serde_json::json!(7));
    assert_eq!(object["MinimumSpeed"], serde_json::json!("60"));
    assert_eq!(object["MaximumSpeed"], serde_json::json!("69"));
    assert_eq!(object["StartLon"], serde_json::json!("103.85298052044503"));
    assert_eq!(object["StartLat"], serde_json::json!("1.3170142376560023"));
    assert_eq!(object["EndLon"], serde_json::json!("103.85259882242372"));
    assert_eq!(object["EndLat"], serde_json::json!("1.3166840028663076"));
}
