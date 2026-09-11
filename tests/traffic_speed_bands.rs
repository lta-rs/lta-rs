mod common;

use std::collections::BTreeSet;

use assert_float_eq::assert_float_absolute_eq;
use lta::operations::get_traffic_speed_bands::{
    decode_get_traffic_speed_bands_response, get_traffic_speed_bands_parts,
};
use lta::{
    Api, GetTrafficSpeedBandsInput, GetTrafficSpeedBandsResponse, LinkId, RoadCategory, SpeedBand,
    TrafficSpeedBand,
};

#[test]
fn traffic_speed_bands_request_preserves_optional_skip() {
    let request = get_traffic_speed_bands_parts(GetTrafficSpeedBandsInput::new())
        .expect("build request parts");
    assert_eq!(request.method, http::Method::GET);
    assert_eq!(request.uri, "/v4/TrafficSpeedBands");

    let request = get_traffic_speed_bands_parts(GetTrafficSpeedBandsInput::new().skip(500))
        .expect("build paged request parts");
    assert_eq!(request.uri, "/v4/TrafficSpeedBands?%24skip=500");

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
        "https://example.test/ltaodataservice/v4/TrafficSpeedBands?%24skip=500"
    );
    assert_eq!(request.headers()["AccountKey"], "test-key");
}

#[test]
fn traffic_speed_bands_decodes_every_vendored_fixture() {
    for (path, body) in common::json_fixtures("traffic_speed_bands") {
        let response = satay_runtime::ResponseParts {
            status: http::StatusCode::OK,
            headers: http::HeaderMap::new(),
            body: body.as_ref(),
        };

        let decoded = decode_get_traffic_speed_bands_response(response)
            .unwrap_or_else(|error| panic!("failed to decode {}: {error}", path.display()));

        let GetTrafficSpeedBandsResponse::<Box<_>>::Ok(bands) = decoded else {
            panic!("expected a successful response for {}", path.display());
        };

        for band in &bands {
            // v4 LinkIDs are variable-length numeric strings ("2".."11099"),
            // not the v3 fixed 9-digit shape.
            assert!(!band.link_id.as_ref().is_empty());
            assert!(
                band.link_id.as_ref().bytes().all(|b| b.is_ascii_digit()),
                "non-numeric LinkID in {}",
                path.display()
            );
            assert!(!band.road_name.is_empty());
            let speed_band = *band.speed_band;
            assert!((1..=8).contains(&speed_band));
            if let Some(max_speed) = band.max_speed {
                assert!(band.min_speed <= max_speed);
            } else {
                assert_eq!(speed_band, 8);
                assert_eq!(band.min_speed, 70);
            }
            assert!(band.start_lon.is_finite());
            assert!(band.start_lat.is_finite());
            assert!(band.end_lon.is_finite());
            assert!(band.end_lat.is_finite());
            // Closed enum: every decoded value is a documented variant, and
            // v4 reserves 7: it must never appear on the wire.
            assert_ne!(band.road_category.as_str(), "7");
        }

        if path
            .file_name()
            .is_some_and(|name| name == "traffic_speed_bands_0.json")
        {
            assert_eq!(bands.len(), 500);
            let first = &bands[0];
            assert_eq!(first.link_id.as_ref(), "2");
            assert_eq!(first.road_name.as_ref(), "NARAYANAN CHETTY ROAD");
            assert_eq!(first.road_category, RoadCategory::SmallRoads);
            assert_eq!(*first.speed_band, 4);
            assert_eq!(first.min_speed, 30);
            assert_eq!(first.max_speed, Some(39));
            assert_float_absolute_eq!(first.start_lon, 103.838_305);
            assert_float_absolute_eq!(first.start_lat, 1.292_06);
            assert_float_absolute_eq!(first.end_lon, 103.838_331);
            assert_float_absolute_eq!(first.end_lat, 1.292_044);
        }

        if path
            .file_name()
            .is_some_and(|name| name == "traffic_speed_bands_1.json")
        {
            // Sanitized v4 capture: one live record per documented category.
            assert_eq!(bands.len(), 7);
            let categories = bands
                .iter()
                .map(|band| band.road_category.as_str().to_owned())
                .collect::<BTreeSet<_>>();
            assert_eq!(
                categories,
                BTreeSet::from(["1", "2", "3", "4", "5", "6", "8"].map(str::to_owned))
            );
        }
    }
}

/// The vendored v4 fixtures collectively cover every documented road category
/// (1-6 plus 8) and never the reserved value 7.
#[test]
fn traffic_speed_bands_fixtures_cover_all_v4_categories() {
    let mut covered = BTreeSet::new();

    for (path, body) in common::json_fixtures("traffic_speed_bands") {
        let response = satay_runtime::ResponseParts {
            status: http::StatusCode::OK,
            headers: http::HeaderMap::new(),
            body: body.as_ref(),
        };

        let decoded = decode_get_traffic_speed_bands_response(response)
            .unwrap_or_else(|error| panic!("failed to decode {}: {error}", path.display()));

        let GetTrafficSpeedBandsResponse::<Box<_>>::Ok(bands) = decoded else {
            panic!("expected a successful response for {}", path.display());
        };

        covered.extend(
            bands
                .iter()
                .map(|band| band.road_category.as_str().to_owned()),
        );
    }

    assert_eq!(
        covered,
        BTreeSet::from(["1", "2", "3", "4", "5", "6", "8"].map(str::to_owned))
    );
    assert!(!covered.contains("7"));
}

#[test]
fn traffic_speed_bands_response_projects_and_parses_wire_fields() {
    let response = satay_runtime::ResponseParts {
        status: http::StatusCode::OK,
        headers: http::HeaderMap::new(),
        body: &br#"{
            "odata.metadata": "https://datamall2.mytransport.sg/ltaodataservice/$metadata#TrafficSpeedBands",
            "lastUpdatedTime": "2026-09-07 17:05:00",
            "value": [{
                "LinkID": "2",
                "RoadName": "NARAYANAN CHETTY ROAD",
                "RoadCategory": "5",
                "SpeedBand": 4,
                "MinimumSpeed": "30",
                "MaximumSpeed": "39",
                "StartLon": "103.838305",
                "StartLat": "1.29206",
                "EndLon": "103.838331",
                "EndLat": "1.292044"
            }]
        }"#[..],
    };

    let decoded =
        decode_get_traffic_speed_bands_response(response).expect("decode projected response");

    let GetTrafficSpeedBandsResponse::<Box<_>>::Ok(bands) = decoded else {
        panic!("expected successful Traffic Speed Bands response");
    };

    assert_eq!(bands.len(), 1);
    let band = &bands[0];
    assert_eq!(band.link_id.as_ref(), "2");
    assert_eq!(band.road_category, RoadCategory::SmallRoads);
    assert_eq!(band.min_speed, 30);
    assert_eq!(band.max_speed, Some(39));
    assert_float_absolute_eq!(band.start_lon, 103.838_305);
    assert_float_absolute_eq!(band.start_lat, 1.292_06);
    assert_float_absolute_eq!(band.end_lon, 103.838_331);
    assert_float_absolute_eq!(band.end_lat, 1.292_044);
}

#[test]
fn traffic_speed_bands_map_every_road_category() {
    let cases = [
        ("1", RoadCategory::Expressway),
        ("2", RoadCategory::MajorArterialRoads),
        ("3", RoadCategory::ArterialRoads),
        ("4", RoadCategory::MinorArterialRoads),
        ("5", RoadCategory::SmallRoads),
        ("6", RoadCategory::SlipRoads),
        ("8", RoadCategory::ShortTunnels),
    ];

    for (wire, expected) in cases {
        let decoded = serde_json::from_value::<RoadCategory>(serde_json::json!(wire))
            .unwrap_or_else(|error| panic!("failed to decode road category {wire}: {error}"));
        assert_eq!(decoded, expected);
    }
}

// v4 deviation from #61 (v3): LinkIDs are variable-length numeric strings instead
// of fixed 9-digit codes, and categories are numeric "1"-"6"/"8" instead of
// letters A-G (G is gone; 8 ShortTunnels is new; 7 is reserved and unused).
// RoadCategory is a closed enum: only the seven documented spellings decode,
// so the reserved value 7, the retired v3 letters, and any other spelling are
// rejected instead of being preserved.
#[test]
fn traffic_speed_bands_reject_unknown_category_spellings() {
    for wire in ["Z", "7", "0", "9", "E", "G", "A", ""] {
        assert!(
            serde_json::from_value::<RoadCategory>(serde_json::json!(wire)).is_err(),
            "unexpectedly accepted road category {wire:?}"
        );
    }
}

#[test]
fn traffic_speed_band_fields_use_semantic_types() {
    fn assert_types(band: &TrafficSpeedBand) {
        let _: &LinkId = &band.link_id;
        let _: &SpeedBand = &band.speed_band;
        let _: u8 = band.min_speed;
        let _: Option<u8> = band.max_speed;
    }

    let _ = assert_types;
}
#[test]
fn traffic_speed_band_rejects_values_outside_strict_domains() {
    let valid = serde_json::json!({
        "LinkID": "2",
        "RoadName": "NARAYANAN CHETTY ROAD",
        "RoadCategory": "5",
        "SpeedBand": 4,
        "MinimumSpeed": "30",
        "MaximumSpeed": "39",
        "StartLon": "103.838305",
        "StartLat": "1.29206",
        "EndLon": "103.838331",
        "EndLat": "1.292044"
    });

    for (field, invalid) in [
        ("LinkID", serde_json::json!("")),
        ("LinkID", serde_json::json!("ABC")),
        ("LinkID", serde_json::json!("12A")),
        ("RoadCategory", serde_json::json!("7")),
        ("RoadCategory", serde_json::json!("E")),
        ("SpeedBand", serde_json::json!(0)),
        ("SpeedBand", serde_json::json!(9)),
        ("MinimumSpeed", serde_json::json!("256")),
        ("MaximumSpeed", serde_json::json!("256")),
    ] {
        let mut wire = valid.clone();
        wire[field] = invalid;
        assert!(
            serde_json::from_value::<TrafficSpeedBand>(wire).is_err(),
            "{field} accepted a value outside its strict domain"
        );
    }

    // v4 LinkIDs vary in length: single digit through five digits are all live.
    assert!(LinkId::try_from("2").is_ok());
    assert!(LinkId::try_from("1472").is_ok());
    assert!(LinkId::try_from("11099").is_ok());
    assert!(LinkId::try_from("").is_err());
    assert!(LinkId::try_from("ABC").is_err());
    assert!(SpeedBand::try_from(1).is_ok());
    assert!(SpeedBand::try_from(8).is_ok());
    assert!(SpeedBand::try_from(0).is_err());
    assert!(SpeedBand::try_from(9).is_err());
}

#[test]
fn traffic_speed_band_serializes_to_canonical_wire_shape() {
    // Live v4 shape for the open-ended band: AYER RAJAH EXPRESSWAY (category 1).
    let band = TrafficSpeedBand::<Box<_>> {
        link_id: LinkId::try_from("11099").expect("valid link id"),
        road_name: "AYER RAJAH EXPRESSWAY".into(),
        road_category: RoadCategory::Expressway,
        speed_band: SpeedBand::try_from(8).expect("valid speed band"),
        min_speed: 70,
        max_speed: None,
        start_lon: 103.830_025,
        start_lat: 1.274_183,
        end_lon: 103.829_264,
        end_lat: 1.274_822,
    };

    let serialized = serde_json::to_value(band).expect("serialize traffic speed band");
    let object = serialized
        .as_object()
        .expect("traffic speed band JSON object");

    assert_eq!(object.len(), 10);
    assert_eq!(object["LinkID"], serde_json::json!("11099"));
    assert_eq!(
        object["RoadName"],
        serde_json::json!("AYER RAJAH EXPRESSWAY")
    );
    assert_eq!(object["RoadCategory"], serde_json::json!("1"));
    assert_eq!(object["SpeedBand"], serde_json::json!(8));
    assert_eq!(object["MinimumSpeed"], serde_json::json!("70"));
    assert_eq!(object["MaximumSpeed"], serde_json::json!("999"));
    assert_eq!(object["StartLon"], serde_json::json!("103.830025"));
    assert_eq!(object["StartLat"], serde_json::json!("1.274183"));
    assert_eq!(object["EndLon"], serde_json::json!("103.829264"));
    assert_eq!(object["EndLat"], serde_json::json!("1.274822"));
}
