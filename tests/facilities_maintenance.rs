mod common;

use lta::operations::get_facilities_maintenance::{
    decode_get_facilities_maintenance_response, get_facilities_maintenance_parts,
};
use lta::{Api, GetFacilitiesMaintenanceInput, GetFacilitiesMaintenanceResponse, StationCode};

#[test]
fn facilities_maintenance_request_sends_required_station_code() {
    let request =
        get_facilities_maintenance_parts(GetFacilitiesMaintenanceInput::new(StationCode::Ns1))
            .expect("build request parts");
    assert_eq!(request.method, http::Method::GET);
    assert_eq!(request.uri, "/FacilitiesMaintenance?StationCode=NS1");

    let request = Api::new()
        .base_url("https://example.test/ltaodataservice/")
        .account_key("test-key")
        .facility()
        .get_facilities_maintenance(StationCode::Ew24)
        .request()
        .expect("build authenticated request");
    assert_eq!(
        request.uri(),
        "https://example.test/ltaodataservice/FacilitiesMaintenance?StationCode=EW24"
    );
    assert_eq!(request.headers()["AccountKey"], "test-key");
}

// Deviation from lta_models: the reference type is `lta_models::train::StationCode`
// with variants named `NS1`/`EW24`/... and a `#[serde(other)] Unknown` fallback.
// Satay names generated enum variants with upper camel case (`Ns1`, `Ew24`); the
// wire values and the variant set match the reference (minus `Unknown`, which is
// unreachable for a request-only parameter).
//
// Maintainer-approved deviation: the enum also carries every station code LTA has
// officially announced for future lines (CRL CR1-CR23, CRL Punggol extension
// CP1-CP4, JRL JS1-JS12/JS2A/JW1-JW5/JE1-JE7, TELe TE32, DTL2e DE1-DE2, and the
// NS3A infill), which the reference deliberately excludes until they open.
#[test]
fn facilities_maintenance_station_code_serializes_to_wire_value() {
    assert_eq!(StationCode::Ns1.as_str(), "NS1");
    assert_eq!(StationCode::Ew24.as_str(), "EW24");
    assert_eq!(StationCode::Te22a.as_str(), "TE22A");
    assert_eq!(StationCode::Stc.as_str(), "STC");
    assert_eq!(
        serde_json::to_value(StationCode::Ns1).expect("serialize station code"),
        serde_json::json!("NS1")
    );
    assert_eq!(
        serde_json::from_str::<StationCode>("\"EW24\"").expect("decode station code"),
        StationCode::Ew24
    );

    // Future announced codes serialize and round-trip with their official wire
    // spelling.
    for (variant, wire) in [
        (StationCode::Cr1, "CR1"),
        (StationCode::Cr23, "CR23"),
        (StationCode::Cp4, "CP4"),
        (StationCode::Te32, "TE32"),
        (StationCode::Ns3a, "NS3A"),
        (StationCode::De2, "DE2"),
        (StationCode::Js12, "JS12"),
        (StationCode::Js2a, "JS2A"),
        (StationCode::Jw5, "JW5"),
        (StationCode::Je7, "JE7"),
    ] {
        assert_eq!(variant.as_str(), wire);
        assert_eq!(
            serde_json::from_str::<StationCode>(&serde_json::to_string(&wire).expect("quote"))
                .expect("decode wire code"),
            variant
        );
    }
}

// Deviation from lta_models: the reference derives `#[serde(other)] Unknown`, so an
// unrecognized station code deserializes to `StationCode::Unknown`. The generated
// enum is closed, so the same input is a deserialization error. The type is
// request-only, so this never affects real requests; unknown codes are rejected at
// compile time instead of producing a nonsense `?StationCode=Unknown` request.
#[test]
fn facilities_maintenance_station_code_rejects_unknown_wire_values() {
    assert!(serde_json::from_str::<StationCode>("\"ZZ9\"").is_err());
}

#[test]
fn facilities_maintenance_decodes_every_vendored_fixture() {
    for (path, body) in common::json_fixtures("facilities_maintenance") {
        let wire = serde_json::from_slice::<serde_json::Value>(&body)
            .unwrap_or_else(|error| panic!("failed to parse {}: {error}", path.display()));

        let response = satay_runtime::ResponseParts {
            status: http::StatusCode::OK,
            headers: http::HeaderMap::new(),
            body,
        };

        let decoded = decode_get_facilities_maintenance_response(response)
            .unwrap_or_else(|error| panic!("failed to decode {}: {error}", path.display()));
        let GetFacilitiesMaintenanceResponse::Ok(links) = decoded else {
            panic!("expected a successful response for {}", path.display());
        };

        // The projection returns the pre-signed `Link` URLs as an unwrapped
        // Vec<String>, one per wire `value[]` entry.
        let expected = wire["value"]
            .as_array()
            .expect("fixture value array")
            .iter()
            .map(|entry| entry["Link"].as_str().expect("fixture Link").to_owned())
            .collect::<Vec<_>>();
        assert_eq!(links, expected);
        assert_eq!(links.len(), 1);
        assert!(links[0].starts_with(
            "https://mtmfacilitiesmaintenance.s3.ap-southeast-1.amazonaws.com/NS1.json"
        ));

        // The wire objects also carry `TimeStamp`, which the public `Vec<String>`
        // return deliberately does not surface.
        assert!(wire["value"][0]["TimeStamp"].is_string());
    }
}
