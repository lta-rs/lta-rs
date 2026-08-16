mod common;

use lta::operations::get_road_openings::{
    decode_get_road_openings_response, get_road_openings_parts,
};
use lta::{Api, EventId, GetRoadOpeningsInput, GetRoadOpeningsResponse, RoadDetails};

// Deviation from the handwritten lta-rs reference: `get_road_details` took a
// `RoadDetailsType` selector that dispatched one method between `/RoadOpenings`
// and `/RoadWorks`. The generated `getRoadOpenings` endpoint is dedicated to
// `/RoadOpenings` and drops the selector; `/RoadWorks` has its own endpoint
// (`getRoadWorks`).
//
// The vendored fixture is a synthetic golden fixture: `lta-models` has no
// RoadOpenings dump, and the issue requires a dedicated one. It models the
// DataMall wire shape exactly (the sibling `road_works.json` shape) and pins
// decode behavior, field mappings, and date parsing.
#[test]
fn traffic_road_openings_request_preserves_optional_skip() {
    let request =
        get_road_openings_parts(GetRoadOpeningsInput::new()).expect("build request parts");
    assert_eq!(request.method, http::Method::GET);
    assert_eq!(request.uri, "/RoadOpenings");

    let request = get_road_openings_parts(GetRoadOpeningsInput::new().skip(500))
        .expect("build paged request parts");
    assert_eq!(request.uri, "/RoadOpenings?%24skip=500");

    let request = Api::new()
        .base_url("https://example.test/ltaodataservice/")
        .account_key("test-key")
        .traffic()
        .get_road_openings()
        .skip(500)
        .request()
        .expect("build authenticated request");
    assert_eq!(
        request.uri(),
        "https://example.test/ltaodataservice/RoadOpenings?%24skip=500"
    );
    assert_eq!(request.headers()["AccountKey"], "test-key");
}

#[test]
fn traffic_road_openings_decodes_every_vendored_fixture() {
    for (path, body) in common::json_fixtures("road_openings") {
        let response = satay_runtime::ResponseParts {
            status: http::StatusCode::OK,
            headers: http::HeaderMap::new(),
            body,
        };

        let decoded = decode_get_road_openings_response(response)
            .unwrap_or_else(|error| panic!("failed to decode {}: {error}", path.display()));
        let GetRoadOpeningsResponse::Ok(openings) = decoded else {
            panic!("expected a successful response for {}", path.display());
        };

        for opening in &openings {
            assert!(!opening.event_id.is_empty());
            assert!(opening.start_date <= opening.end_date);
            assert!(!opening.service_dept.is_empty());
            assert!(!opening.road_name.is_empty());
        }

        if path
            .file_name()
            .is_some_and(|name| name == "road_openings_0.json")
        {
            assert_eq!(openings.len(), 20);
            let first = &openings[0];
            assert_eq!(first.event_id.as_ref(), "RMAPP-202401-0001");
            assert_eq!(
                first.start_date,
                satay_runtime::parse_date("2024-01-15").expect("parse reference start date")
            );
            assert_eq!(
                first.end_date,
                satay_runtime::parse_date("2024-07-14").expect("parse reference end date")
            );
            assert_eq!(first.service_dept, "LTA");
            assert_eq!(first.road_name, "WOODLANDS AVENUE 12");
            assert_eq!(first.other, "Road opening");
        }
    }
}

#[test]
fn traffic_road_openings_response_projects_and_parses_wire_fields() {
    let response = satay_runtime::ResponseParts {
        status: http::StatusCode::OK,
        headers: http::HeaderMap::new(),
        body: br#"{
            "odata.metadata": "https://datamall2.mytransport.sg/ltaodataservice/$metadata#RoadOpenings",
            "value": [{
                "EventID": "RMAPP-202405-0003",
                "StartDate": "2024-05-01",
                "EndDate": "2024-11-30",
                "SvcDept": "PRIVATE",
                "RoadName": "NEW UPPER CHANGI ROAD",
                "Other": "Road opening"
            }]
        }"#,
    };

    let decoded = decode_get_road_openings_response(response).expect("decode projected response");
    let GetRoadOpeningsResponse::Ok(openings) = decoded else {
        panic!("expected successful Road Openings response");
    };

    assert_eq!(openings.len(), 1);
    let opening = &openings[0];
    assert_eq!(opening.event_id.as_ref(), "RMAPP-202405-0003");
    assert_eq!(
        opening.start_date,
        satay_runtime::parse_date("2024-05-01").expect("parse wire start date")
    );
    assert_eq!(
        opening.end_date,
        satay_runtime::parse_date("2024-11-30").expect("parse wire end date")
    );
    assert_eq!(opening.service_dept, "PRIVATE");
    assert_eq!(opening.road_name, "NEW UPPER CHANGI ROAD");
    assert_eq!(opening.other, "Road opening");
}

#[test]
fn traffic_road_opening_rejects_malformed_dates() {
    let valid = serde_json::json!({
        "EventID": "RMAPP-202405-0003",
        "StartDate": "2024-05-01",
        "EndDate": "2024-11-30",
        "SvcDept": "PRIVATE",
        "RoadName": "NEW UPPER CHANGI ROAD",
        "Other": "Road opening"
    });

    for (field, malformed) in [
        ("StartDate", serde_json::json!("2024/05/01")),
        ("EndDate", serde_json::json!("2024-13-30")),
    ] {
        let mut wire = valid.clone();
        wire[field] = malformed;
        assert!(
            serde_json::from_value::<RoadDetails>(wire).is_err(),
            "{field} accepted a malformed date"
        );
    }
}

#[test]
fn traffic_road_opening_serializes_to_canonical_wire_shape() {
    let opening = RoadDetails {
        event_id: EventId::try_from("RMAPP-202405-0003").expect("valid event id"),
        start_date: satay_runtime::parse_date("2024-05-01").expect("parse start date"),
        end_date: satay_runtime::parse_date("2024-11-30").expect("parse end date"),
        service_dept: "PRIVATE".into(),
        road_name: "NEW UPPER CHANGI ROAD".into(),
        other: "Road opening".into(),
    };

    let serialized = serde_json::to_value(opening).expect("serialize road opening");
    let object = serialized.as_object().expect("road opening JSON object");

    assert_eq!(object.len(), 6);
    assert_eq!(object["EventID"], serde_json::json!("RMAPP-202405-0003"));
    assert_eq!(object["StartDate"], serde_json::json!("2024-05-01"));
    assert_eq!(object["EndDate"], serde_json::json!("2024-11-30"));
    assert_eq!(object["SvcDept"], serde_json::json!("PRIVATE"));
    assert_eq!(
        object["RoadName"],
        serde_json::json!("NEW UPPER CHANGI ROAD")
    );
    assert_eq!(object["Other"], serde_json::json!("Road opening"));
}
