mod common;

use lta::operations::get_road_works::{decode_get_road_works_response, get_road_works_parts};
use lta::{Api, EventId, GetRoadWorksInput, GetRoadWorksResponse, RoadDetails};

// Deviation from the handwritten lta-rs reference: `get_road_details` took a
// `RoadDetailsType` selector that dispatched one method between `/RoadOpenings`
// and `/RoadWorks`. The generated `getRoadWorks` endpoint is dedicated to
// `/RoadWorks` and drops the selector; `/RoadOpenings` gets its own endpoint
// (`getRoadOpenings`) when it is generated.
#[test]
fn traffic_road_works_request_preserves_optional_skip() {
    let request = get_road_works_parts(GetRoadWorksInput::new()).expect("build request parts");
    assert_eq!(request.method, http::Method::GET);
    assert_eq!(request.uri, "/RoadWorks");

    let request = get_road_works_parts(GetRoadWorksInput::new().skip(500))
        .expect("build paged request parts");
    assert_eq!(request.uri, "/RoadWorks?%24skip=500");

    let request = Api::new()
        .base_url("https://example.test/ltaodataservice/")
        .account_key("test-key")
        .traffic()
        .get_road_works()
        .skip(500)
        .request()
        .expect("build authenticated request");
    assert_eq!(
        request.uri(),
        "https://example.test/ltaodataservice/RoadWorks?%24skip=500"
    );
    assert_eq!(request.headers()["AccountKey"], "test-key");
}

#[test]
fn traffic_road_works_decodes_every_vendored_fixture() {
    for (path, body) in common::json_fixtures("road_works") {
        let response = satay_runtime::ResponseParts {
            status: http::StatusCode::OK,
            headers: http::HeaderMap::new(),
            body,
        };

        let decoded = decode_get_road_works_response(response)
            .unwrap_or_else(|error| panic!("failed to decode {}: {error}", path.display()));
        let GetRoadWorksResponse::Ok(works) = decoded else {
            panic!("expected a successful response for {}", path.display());
        };

        for work in &works {
            assert!(!work.event_id.is_empty());
            assert!(work.start_date <= work.end_date);
            assert!(!work.service_dept.is_empty());
            assert!(!work.road_name.is_empty());
        }

        if path
            .file_name()
            .is_some_and(|name| name == "road_works_0.json")
        {
            assert_eq!(works.len(), 500);
            let first = &works[0];
            assert_eq!(first.event_id.as_ref(), "RMAPP-202109-0626");
            assert_eq!(
                first.start_date,
                satay_runtime::parse_date("2021-09-21").expect("parse reference start date")
            );
            assert_eq!(
                first.end_date,
                satay_runtime::parse_date("2026-11-30").expect("parse reference end date")
            );
            assert_eq!(first.service_dept, "PRIVATE");
            assert_eq!(first.road_name, "ADMIRALTY LANE");
            assert_eq!(first.other, "For details");
        }
    }
}

#[test]
fn traffic_road_works_response_projects_and_parses_wire_fields() {
    let response = satay_runtime::ResponseParts {
        status: http::StatusCode::OK,
        headers: http::HeaderMap::new(),
        body: br#"{
            "odata.metadata": "http://datamall2.mytransport.sg/ltaodataservice/$metadata#RoadWorks",
            "value": [{
                "EventID": "RMAPP-202109-0626",
                "StartDate": "2021-09-21",
                "EndDate": "2026-11-30",
                "SvcDept": "PRIVATE",
                "RoadName": "ADMIRALTY LANE",
                "Other": "For details"
            }]
        }"#,
    };

    let decoded = decode_get_road_works_response(response).expect("decode projected response");
    let GetRoadWorksResponse::Ok(works) = decoded else {
        panic!("expected successful Road Works response");
    };

    assert_eq!(works.len(), 1);
    let work = &works[0];
    assert_eq!(work.event_id.as_ref(), "RMAPP-202109-0626");
    assert_eq!(
        work.start_date,
        satay_runtime::parse_date("2021-09-21").expect("parse wire start date")
    );
    assert_eq!(
        work.end_date,
        satay_runtime::parse_date("2026-11-30").expect("parse wire end date")
    );
    assert_eq!(work.service_dept, "PRIVATE");
    assert_eq!(work.road_name, "ADMIRALTY LANE");
    assert_eq!(work.other, "For details");
}

#[test]
fn traffic_road_work_validates_event_id_format() {
    assert!(EventId::try_from("RMAPP-202109-0626").is_ok());
    assert!(EventId::try_from("RMINRM-202205-0008").is_ok());

    for malformed in [
        "RMAPP-202109-062",        // suffix group is not 4 digits
        "RMAPP-20210-0626",        // date group is not 6 digits
        "rmapp-202109-0626",       // lowercase prefix
        "202109-0626",             // missing letter prefix
        "RMAPP-202109-0626X",      // trailing junk
        "RMAPP-２０２１０９-0626", // numeric groups must use ASCII digits
    ] {
        assert!(
            EventId::try_from(malformed).is_err(),
            "{malformed} accepted as an event id"
        );
    }
}

#[test]
fn traffic_road_work_rejects_malformed_dates() {
    let valid = serde_json::json!({
        "EventID": "RMAPP-202109-0626",
        "StartDate": "2021-09-21",
        "EndDate": "2026-11-30",
        "SvcDept": "PRIVATE",
        "RoadName": "ADMIRALTY LANE",
        "Other": "For details"
    });

    for (field, malformed) in [
        ("StartDate", serde_json::json!("2021/09/21")),
        ("EndDate", serde_json::json!("2026-13-30")),
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
fn traffic_road_work_serializes_to_canonical_wire_shape() {
    let work = RoadDetails {
        event_id: EventId::try_from("RMAPP-202109-0626").expect("valid event id"),
        start_date: satay_runtime::parse_date("2021-09-21").expect("parse start date"),
        end_date: satay_runtime::parse_date("2026-11-30").expect("parse end date"),
        service_dept: "PRIVATE".into(),
        road_name: "ADMIRALTY LANE".into(),
        other: "For details".into(),
    };

    let serialized = serde_json::to_value(work).expect("serialize road work");
    let object = serialized.as_object().expect("road work JSON object");

    assert_eq!(object.len(), 6);
    assert_eq!(object["EventID"], serde_json::json!("RMAPP-202109-0626"));
    assert_eq!(object["StartDate"], serde_json::json!("2021-09-21"));
    assert_eq!(object["EndDate"], serde_json::json!("2026-11-30"));
    assert_eq!(object["SvcDept"], serde_json::json!("PRIVATE"));
    assert_eq!(object["RoadName"], serde_json::json!("ADMIRALTY LANE"));
    assert_eq!(object["Other"], serde_json::json!("For details"));
}
