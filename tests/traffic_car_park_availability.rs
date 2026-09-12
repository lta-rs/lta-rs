mod common;

use assert_float_eq::assert_float_absolute_eq;
use lta::operations::get_car_park_availability::{
    decode_get_car_park_availability_response, get_car_park_availability_parts,
};
use lta::{
    Api, CarPark, CarParkAgency, CarParkLotType, GetCarParkAvailabilityInput,
    GetCarParkAvailabilityResponse,
};

#[test]
fn car_park_availability_request_preserves_optional_skip() {
    let request = get_car_park_availability_parts(GetCarParkAvailabilityInput::new())
        .expect("build request parts");
    assert_eq!(request.method, http::Method::GET);
    assert_eq!(request.uri, "/CarParkAvailabilityv2");

    let request = get_car_park_availability_parts(GetCarParkAvailabilityInput::new().skip(500))
        .expect("build paged request parts");
    assert_eq!(request.uri, "/CarParkAvailabilityv2?%24skip=500");

    let request = Api::new()
        .base_url("https://example.test/ltaodataservice/")
        .account_key("test-key")
        .traffic()
        .get_car_park_availability()
        .skip(500)
        .request()
        .expect("build authenticated request");
    assert_eq!(
        request.uri(),
        "https://example.test/ltaodataservice/CarParkAvailabilityv2?%24skip=500"
    );
    assert_eq!(request.headers()["AccountKey"], "test-key");
}

#[test]
fn car_park_availability_decodes_every_vendored_fixture() {
    for (path, body) in common::json_fixtures("car_park_availability") {
        let response = satay_runtime::ResponseParts {
            status: http::StatusCode::OK,
            headers: http::HeaderMap::new(),
            body: body.as_ref(),
        };

        let decoded = decode_get_car_park_availability_response(response)
            .unwrap_or_else(|error| panic!("failed to decode {}: {error}", path.display()));

        let GetCarParkAvailabilityResponse::<Box<_>>::Ok(parks) = decoded else {
            panic!("expected a successful response for {}", path.display());
        };

        assert!(!parks.is_empty(), "expected records in {}", path.display());

        for park in &parks {
            assert!(
                !park.carpark_id.is_empty(),
                "carpark_id in {}",
                path.display()
            );
            assert!(!park.dev.is_empty(), "dev in {}", path.display());
            if let Some(coords) = &park.coords {
                assert!((*coords.lat).is_finite());
                assert!((*coords.long).is_finite());
            }
        }

        if path
            .file_name()
            .is_some_and(|name| name == "car_park_availability_0.json")
        {
            assert_eq!(parks.len(), 500);

            let mut lot_c = 0;
            let mut lot_l = 0;
            let mut lot_y = 0;
            let mut lot_h = 0;
            let mut lot_s = 0;
            let mut agency_hdb = 0;
            let mut agency_ura = 0;
            let mut agency_lta = 0;
            let mut coords_none = 0;

            for park in &parks {
                match &park.lot_type {
                    CarParkLotType::C => lot_c += 1,
                    CarParkLotType::L => lot_l += 1,
                    CarParkLotType::Y => lot_y += 1,
                    CarParkLotType::H => lot_h += 1,
                    CarParkLotType::S => lot_s += 1,
                }
                match &park.agency {
                    CarParkAgency::Hdb => agency_hdb += 1,
                    CarParkAgency::Ura => agency_ura += 1,
                    CarParkAgency::Lta => agency_lta += 1,
                }
                if park.coords.is_none() {
                    coords_none += 1;
                }
            }

            assert_eq!((lot_c, lot_l, lot_y, lot_h, lot_s), (426, 12, 49, 13, 0));
            assert_eq!((agency_hdb, agency_ura, agency_lta), (367, 90, 43));
            // 9 empty Location strings plus one 4-component Location
            // (TELOK AYER STREET) decode as None via lossy coordinates.
            assert_eq!(coords_none, 10);

            let first = &parks[0];
            assert_eq!(first.carpark_id.as_ref(), "1");
            assert_eq!(first.area.as_ref(), "Marina");
            assert_eq!(first.dev.as_ref(), "Suntec City");
            let coords = first.coords.as_ref().expect("first has coords");
            assert_float_absolute_eq!(*coords.lat, 1.29375);
            assert_float_absolute_eq!(*coords.long, 103.85718);
            assert_eq!(first.avail_lots, 463);
            assert_eq!(first.lot_type, CarParkLotType::C);
            assert_eq!(first.agency, CarParkAgency::Lta);
        }
    }
}

#[test]
fn car_park_availability_response_projects_and_parses_wire_fields() {
    let response = satay_runtime::ResponseParts {
        status: http::StatusCode::OK,
        headers: http::HeaderMap::new(),
        body: &br#"{
            "odata.metadata": "https://datamall2.mytransport.sg/ltaodataservice/$metadata#CarParkAvailability",
            "value": [{
                "CarParkID": "1",
                "Area": "Marina",
                "Development": "Suntec City",
                "Location": "1.29375 103.85718",
                "AvailableLots": 463,
                "LotType": "C",
                "Agency": "LTA"
            }, {
                "CarParkID": "A0007",
                "Area": "",
                "Development": "ANGULLIA PARK OFF STREET",
                "Location": "1.3053280313212745 103.82957069514487",
                "AvailableLots": 0,
                "LotType": "Y",
                "Agency": "URA"
            }]
        }"#[..],
    };

    let decoded =
        decode_get_car_park_availability_response(response).expect("decode projected response");
    let GetCarParkAvailabilityResponse::<Box<_>>::Ok(parks) = decoded else {
        panic!("expected successful Car Park Availability response");
    };

    assert_eq!(parks.len(), 2);

    let first = &parks[0];
    assert_eq!(first.carpark_id.as_ref(), "1");
    assert_eq!(first.area.as_ref(), "Marina");
    assert_eq!(first.dev.as_ref(), "Suntec City");
    let coords = first.coords.as_ref().expect("first has coords");
    assert_float_absolute_eq!(*coords.lat, 1.29375);
    assert_float_absolute_eq!(*coords.long, 103.85718);
    assert_eq!(first.avail_lots, 463);
    assert_eq!(first.lot_type, CarParkLotType::C);
    assert_eq!(first.agency, CarParkAgency::Lta);

    let second = &parks[1];
    assert_eq!(second.carpark_id.as_ref(), "A0007");
    assert_eq!(second.area.as_ref(), "");
    assert_eq!(second.dev.as_ref(), "ANGULLIA PARK OFF STREET");
    let coords = second.coords.as_ref().expect("second has coords");
    assert_float_absolute_eq!(*coords.lat, 1.305_328_031_321_274_5);
    assert_float_absolute_eq!(*coords.long, 103.829_570_695_144_87);
    assert_eq!(second.avail_lots, 0);
    assert_eq!(second.lot_type, CarParkLotType::Y);
    assert_eq!(second.agency, CarParkAgency::Ura);
}

fn car_park_with_location(location: &str) -> serde_json::Value {
    serde_json::json!({
        "CarParkID": "1",
        "Area": "Marina",
        "Development": "Suntec City",
        "Location": location,
        "AvailableLots": 463,
        "LotType": "C",
        "Agency": "LTA"
    })
}

// Reference `from_str_to_coords` returns None for empty or regex-mismatched
// Location strings. The coordinates codec reproduces that via
// `treat-error-as-none: true`: any string that is not exactly two
// space-delimited finite floats (including "" and 4-component values)
// decodes as None instead of erroring.
#[test]
fn car_park_availability_location_empty_and_invalid_decode_as_none() {
    for location in [
        "",
        "broken",
        "1.29375",
        "1.29375 103.85718 1.0",
        "1.279101488551342 103.84691237884489 1.2830672511607033 103.84904433544153",
        "NaN 103.85718",
        "1.29375 inf",
    ] {
        let decoded = serde_json::from_value::<CarPark>(car_park_with_location(location))
            .unwrap_or_else(|error| panic!("failed to decode location {location:?}: {error}"));
        assert_eq!(
            decoded.coords, None,
            "location {location:?} should decode as None"
        );
    }

    let decoded = serde_json::from_value::<CarPark>(car_park_with_location("1.29375 103.85718"))
        .expect("valid location decodes");
    let coords = decoded.coords.expect("valid location has coords");
    assert_float_absolute_eq!(*coords.lat, 1.29375);
    assert_float_absolute_eq!(*coords.long, 103.85718);

    // Leading/trailing whitespace is trimmed by the pair codec.
    let decoded = serde_json::from_value::<CarPark>(car_park_with_location(" 1.29375 103.85718 "))
        .expect("padded location decodes");
    assert!(decoded.coords.is_some());
}

// Reference LotType/Agency use #[serde(other)] Unknown for unrecognized
// values. Satay open string enums preserve the wire value in Other(String);
// wire serialization stays identical. Agency variant names use satay's
// upper-camel casing (Hdb/Ura/Lta) like the accepted StationCode/RackType
// deviation, while LotType single letters match the reference exactly.
#[test]
fn car_park_availability_preserve_unknown_lot_type_and_agency() {
    let decoded = serde_json::from_value::<CarParkLotType>(serde_json::json!("C"))
        .expect("decode known lot type");
    assert_eq!(decoded, CarParkLotType::C);
    assert_eq!(decoded.as_str(), "C");

    for wire in ["C", "L", "Y", "H"] {
        let decoded = serde_json::from_value::<CarParkLotType>(serde_json::json!(wire))
            .unwrap_or_else(|error| panic!("failed to decode lot type {wire}: {error}"));
        assert_eq!(decoded.as_str(), wire, "lot type {wire} round-trips");
    }

    for (wire, expected) in [
        ("HDB", CarParkAgency::Hdb),
        ("URA", CarParkAgency::Ura),
        ("LTA", CarParkAgency::Lta),
    ] {
        let decoded = serde_json::from_value::<CarParkAgency>(serde_json::json!(wire))
            .unwrap_or_else(|error| panic!("failed to decode agency {wire}: {error}"));
        assert_eq!(decoded, expected);
        assert_eq!(decoded.as_str(), wire, "agency {wire} round-trips");
    }
}
