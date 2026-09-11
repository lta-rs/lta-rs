mod common;

use assert_float_eq::assert_float_absolute_eq;
use lta::operations::get_traffic_incidents::{
    decode_get_traffic_incidents_response, get_traffic_incidents_parts,
};
use lta::{Api, GetTrafficIncidentsInput, GetTrafficIncidentsResponse, IncidentType};

#[test]
fn traffic_incidents_request_preserves_optional_skip() {
    let request =
        get_traffic_incidents_parts(GetTrafficIncidentsInput::new()).expect("build request parts");
    assert_eq!(request.method, http::Method::GET);
    assert_eq!(request.uri, "/TrafficIncidents");

    let request = get_traffic_incidents_parts(GetTrafficIncidentsInput::new().skip(500))
        .expect("paged parts");
    assert_eq!(request.uri, "/TrafficIncidents?%24skip=500");

    let request = Api::new()
        .base_url("https://example.test/ltaodataservice/")
        .account_key("test-key")
        .traffic()
        .get_incidents()
        .skip(500)
        .request()
        .expect("build authenticated request");
    assert_eq!(
        request.uri(),
        "https://example.test/ltaodataservice/TrafficIncidents?%24skip=500"
    );
    assert_eq!(request.headers()["AccountKey"], "test-key");
}

#[test]
fn traffic_incidents_decodes_every_vendored_fixture() {
    for (path, body) in common::json_fixtures("traffic_incidents") {
        let response = satay_runtime::ResponseParts {
            status: http::StatusCode::OK,
            headers: http::HeaderMap::new(),
            body: body.as_ref(),
        };

        let decoded = decode_get_traffic_incidents_response(response)
            .unwrap_or_else(|error| panic!("failed to decode {}: {error}", path.display()));

        let GetTrafficIncidentsResponse::<Box<_>>::Ok(incidents) = decoded else {
            panic!("expected a successful response for {}", path.display());
        };

        for incident in &incidents {
            assert!((*incident.lat).is_finite());
            assert!((*incident.long).is_finite());
            assert!(!incident.msg.is_empty());
        }

        if path
            .file_name()
            .is_some_and(|name| name == "traffic_incidents_0.json")
        {
            assert_eq!(incidents.len(), 39);
            assert_eq!(
                incidents
                    .iter()
                    .filter(|incident| incident.incident_type == IncidentType::Accident)
                    .count(),
                1
            );
            assert_eq!(
                incidents
                    .iter()
                    .filter(|incident| incident.incident_type == IncidentType::Roadwork)
                    .count(),
                38
            );
            assert_eq!(incidents[0].incident_type, IncidentType::Roadwork);
            assert_float_absolute_eq!(*incidents[0].lat, 1.327_178_430_338_124_7);
            assert_float_absolute_eq!(*incidents[0].long, 103.735_713_119_904_26);
            assert!(incidents[0].msg.contains("Roadworks on AYE"));
        }
    }
}

#[test]
fn traffic_incidents_accepts_aliased_incident_spellings() {
    let response = satay_runtime::ResponseParts {
        status: http::StatusCode::OK,
        headers: http::HeaderMap::new(),
        body: &br#"{
            "odata.metadata": "http://datamall2.mytransport.sg/ltaodataservice/$metadata#IncidentSet",
            "value": [{
                "Type": "Road Works",
                "Latitude": 1.0,
                "Longitude": 103.0,
                "Message": "lane closure"
            }, {
                "Type": "Heavy Traffic",
                "Latitude": 1.1,
                "Longitude": 103.1,
                "Message": "stop-and-go"
            }, {
                "Type": "Unattended Vehicle",
                "Latitude": 1.2,
                "Longitude": 103.2,
                "Message": "left lane blocked"
            }]
        }"#[..],
    };

    let decoded = decode_get_traffic_incidents_response(response).expect("decode projected");
    let GetTrafficIncidentsResponse::<Box<_>>::Ok(incidents) = decoded else {
        panic!("expected successful Traffic Incidents response");
    };

    assert_eq!(incidents[0].incident_type, IncidentType::RoadWorks);
    assert_eq!(incidents[1].incident_type, IncidentType::HeavyTraffic);
    assert_eq!(incidents[2].incident_type, IncidentType::UnattendedVehicle);
}

#[test]
fn traffic_incidents_unknown_spelling_falls_back_to_other() {
    let response = satay_runtime::ResponseParts {
        status: http::StatusCode::OK,
        headers: http::HeaderMap::new(),
        body: &br#"{
            "odata.metadata": "http://datamall2.mytransport.sg/ltaodataservice/$metadata#IncidentSet",
            "value": [{
                "Type": "Flooding",
                "Latitude": 1.0,
                "Longitude": 103.0,
                "Message": "road flooded"
            }]
        }"#[..],
    };

    let decoded = decode_get_traffic_incidents_response(response).expect("decode projected");
    let GetTrafficIncidentsResponse::<Box<_>>::Ok(incidents) = decoded else {
        panic!("expected successful Traffic Incidents response");
    };

    assert_eq!(
        incidents[0].incident_type,
        IncidentType::Other("Flooding".into())
    );
}

// Deviation from lta_models: the reference serde derives accept the canonical
// variant name ("RoadWorks") in addition to the wire alias ("Road Works").
// The generated deserializer matches only the DataMall wire spellings, so the
// canonical spelling falls back to Other(String). DataMall sends only the
// aliased spellings, so real wire data is unaffected.
#[test]
fn traffic_incidents_canonical_spelling_falls_back_to_other() {
    assert_eq!(
        serde_json::from_str::<IncidentType>("\"RoadWorks\"").expect("decode canonical spelling"),
        IncidentType::Other("RoadWorks".into())
    );
    assert_eq!(
        serde_json::from_str::<IncidentType>("\"Road Works\"").expect("decode wire spelling"),
        IncidentType::RoadWorks
    );
}
