mod common;

use std::collections::BTreeSet;

use assert_float_eq::assert_float_absolute_eq;
use lta::operations::get_ev_charging_points::{
    decode_get_ev_charging_points_response, get_ev_charging_points_parts,
};
use lta::{
    Api, EvChargingPointStatus, EvConnectorStatus, EvPlugTypeKind, EvPowerRating, EvPriceType,
    GetEvChargingPointsInput, GetEvChargingPointsResponse, PostalCode,
};

#[test]
fn ev_charging_points_request_requires_postal_code() {
    let postal_code = PostalCode::try_from("650346").expect("valid postal code");
    let request = get_ev_charging_points_parts(GetEvChargingPointsInput::new(postal_code))
        .expect("build request parts");
    assert_eq!(request.method, http::Method::GET);
    assert_eq!(request.uri, "/EVChargingPoints?PostalCode=650346");

    let request = Api::new()
        .base_url("https://example.test/ltaodataservice/")
        .account_key("test-key")
        .ev()
        .get_charging_points(PostalCode::try_from("650346").expect("valid"))
        .request()
        .expect("build authenticated request");
    assert_eq!(
        request.uri(),
        "https://example.test/ltaodataservice/EVChargingPoints?PostalCode=650346"
    );
    assert_eq!(request.headers()["AccountKey"], "test-key");
}

#[test]
fn ev_charging_points_request_preserves_optional_skip() {
    let postal_code = PostalCode::try_from("650346").expect("valid postal code");
    // Live API accepts $skip but ignores it for postal-code queries; client must still send it exactly.
    let request =
        get_ev_charging_points_parts(GetEvChargingPointsInput::new(postal_code.clone()).skip(500))
            .expect("build paged request parts");
    assert_eq!(
        request.uri,
        "/EVChargingPoints?PostalCode=650346&%24skip=500"
    );

    let request = Api::new()
        .base_url("https://example.test/ltaodataservice/")
        .account_key("test-key")
        .ev()
        .get_charging_points(postal_code)
        .skip(500)
        .request()
        .expect("build authenticated request");
    assert_eq!(
        request.uri(),
        "https://example.test/ltaodataservice/EVChargingPoints?PostalCode=650346&%24skip=500"
    );
    assert_eq!(request.headers()["AccountKey"], "test-key");
}

#[test]
fn ev_charging_points_postal_code_is_six_digits_with_leading_zeroes() {
    // Leading zeroes are preserved (e.g. 018956) because PostalCode is a validated string newtype.
    let postal_code = PostalCode::try_from("018956").expect("valid postal code with leading zero");
    assert_eq!(postal_code.as_ref(), "018956");

    for invalid in ["123", "12345", "1234567", "abcdef", "65034a", ""] {
        assert!(
            PostalCode::try_from(invalid).is_err(),
            "{invalid} accepted as a postal code"
        );
    }
}

#[test]
fn ev_charging_points_response_projects_value_wrapper() {
    let response = satay_runtime::ResponseParts {
        status: http::StatusCode::OK,
        headers: http::HeaderMap::new(),
        body: br#"{
            "value": {"evLocationsData": [{
                "address": "346 BUKIT BATOK STREET 34 SINGAPORE 650346",
                "name": "BLK 337-353/355-356 BUKIT BATOK STREET 34",
                "longitude": 103.75001,
                "latitude": 1.363105,
                "locationId": "750010650346",
                "status": "",
                "chargingPoints": [{
                    "status": "1",
                    "operatingHours": "",
                    "operator": "CHARGE+ PTE. LTD.",
                    "position": "L1 255 & 256",
                    "name": "BLK 337-353/355-356 BUKIT BATOK STREET 34",
                    "id": "",
                    "plugTypes": [{
                        "plugType": "Type 2",
                        "powerRating": "AC",
                        "chargingSpeed": "7.4",
                        "price": "0.6490",
                        "priceType": "kWh",
                        "evIds": [
                            {"id": "", "evCpId": "R109701B-001", "status": "0"},
                            {"id": "", "evCpId": "R109701B-002", "status": "1"}
                        ]
                    }]
                }]
            }]}
        }"#
        .to_vec(),
    };

    let decoded =
        decode_get_ev_charging_points_response(response).expect("decode projected response");
    let GetEvChargingPointsResponse::Ok(value) = decoded else {
        panic!("expected successful EV Charging Points response");
    };

    assert_eq!(value.ev_locations_data.len(), 1);
    let location = &value.ev_locations_data[0];
    assert_eq!(
        location.address,
        "346 BUKIT BATOK STREET 34 SINGAPORE 650346"
    );
    assert_float_absolute_eq!(*location.long, 103.75001);
    assert_float_absolute_eq!(*location.lat, 1.363_105);
    assert_eq!(location.location_id.as_ref(), "750010650346");
    assert_eq!(location.charging_points.len(), 1);
    let point = &location.charging_points[0];
    assert_eq!(point.status, EvChargingPointStatus::Available);
    let plug = &point.plug_types[0];
    assert_eq!(plug.plug_type, EvPlugTypeKind::Type2);
    assert_eq!(plug.power_rating, EvPowerRating::Ac);
    assert_float_absolute_eq!(plug.charging_speed, 7.4);
    assert_float_absolute_eq!(plug.price.expect("price present"), 0.6490);
    assert_eq!(plug.price_type, EvPriceType::Kwh);
    assert_eq!(plug.ev_ids.len(), 2);
    assert_eq!(plug.ev_ids[0].status, EvConnectorStatus::Occupied);
    assert_eq!(plug.ev_ids[1].status, EvConnectorStatus::Available);
}

#[test]
fn ev_charging_points_decodes_every_vendored_fixture() {
    let mut seen_cp_status = BTreeSet::new();
    let mut seen_ev_status = BTreeSet::new();
    let mut seen_plug = BTreeSet::new();
    let mut seen_price_type = BTreeSet::new();
    let mut seen_empty_locations = false;
    let mut seen_empty_price = false;
    let mut seen_free_price_type = false;

    for (path, body) in common::json_fixtures("ev_charging_points") {
        let wire = serde_json::from_slice::<serde_json::Value>(&body)
            .unwrap_or_else(|error| panic!("failed to parse {}: {error}", path.display()));

        // Guide/fixture difference: live /EVChargingPoints omits odata.metadata
        // (unlike every other DataMall endpoint) and uses correct `longitude`.
        // The guide documents `longtitude`; the /EVCBatch file uses it.
        assert!(
            wire.get("odata.metadata").is_none(),
            "unexpected odata.metadata in {}",
            path.display()
        );
        let value = wire.get("value").expect("fixture value object");
        assert!(
            value.is_object(),
            "value must be object in {}",
            path.display()
        );
        let locations = value
            .get("evLocationsData")
            .expect("fixture evLocationsData");
        let locations = locations.as_array().expect("evLocationsData array");

        for location in locations {
            assert!(
                location.get("longitude").is_some(),
                "missing wire-canonical longitude in {}",
                path.display()
            );
            assert!(
                location.get("longtitude").is_none(),
                "guide-spelling longtitude must not appear in /EVChargingPoints fixtures ({})",
                path.display()
            );
        }

        let response = satay_runtime::ResponseParts {
            status: http::StatusCode::OK,
            headers: http::HeaderMap::new(),
            body,
        };
        let decoded = decode_get_ev_charging_points_response(response)
            .unwrap_or_else(|error| panic!("failed to decode {}: {error}", path.display()));
        let GetEvChargingPointsResponse::Ok(value) = decoded else {
            panic!("expected a successful response for {}", path.display());
        };

        if value.ev_locations_data.is_empty() {
            seen_empty_locations = true;
            assert_eq!(
                locations.len(),
                0,
                "empty fixture must have no locations in {}",
                path.display()
            );
            continue;
        }

        assert_eq!(value.ev_locations_data.len(), locations.len());

        for location in &value.ev_locations_data {
            assert!(!location.address.is_empty());
            assert!(!location.name.is_empty());
            assert!(
                (*location.long).is_finite(),
                "invalid longitude in {}",
                path.display()
            );
            assert!(
                (*location.lat).is_finite(),
                "invalid latitude in {}",
                path.display()
            );
            assert!(
                (-180.0..=180.0).contains(&*location.long),
                "longitude out of range in {}",
                path.display()
            );
            assert!(
                (-90.0..=90.0).contains(&*location.lat),
                "latitude out of range in {}",
                path.display()
            );
            assert_eq!(location.location_id.as_ref().len(), 12);
            assert!(
                location
                    .location_id
                    .as_ref()
                    .bytes()
                    .all(|b| b.is_ascii_digit()),
                "non-numeric locationId in {}",
                path.display()
            );
            assert!(!location.charging_points.is_empty());

            for point in &location.charging_points {
                seen_cp_status.insert(point.status.as_str().to_owned());
                assert!(!point.operator.is_empty());
                assert!(!point.position.is_empty());
                assert!(!point.plug_types.is_empty());

                for plug in &point.plug_types {
                    seen_plug.insert(plug.plug_type.as_str().to_owned());
                    seen_price_type.insert(plug.price_type.as_str().to_owned());
                    assert!(plug.charging_speed.is_finite());
                    assert!(plug.charging_speed > 0.0);
                    match plug.price {
                        Some(price) => {
                            assert!(price.is_finite());
                            assert!(price >= 0.0);
                            if plug.price_type == EvPriceType::Free {
                                seen_free_price_type = true;
                            }
                        }
                        None => {
                            seen_empty_price = true;
                        }
                    }
                    assert!(!plug.ev_ids.is_empty());
                    for ev in &plug.ev_ids {
                        seen_ev_status.insert(ev.status.as_str().to_owned());
                        assert!(!ev.ev_cp_id.as_ref().is_empty());
                    }
                }
            }
        }
    }

    // Stronger type safety from 2753-location batch analysis + live edge cases:
    // CP status covers occupied/available/not-available, ev status covers
    // occupied/available/empty, plug covers Type 2/Combo 2 (CHAdeMO in batch),
    // price covers numeric/empty, priceType covers kWh/empty/free.
    assert!(
        seen_cp_status.contains("0"),
        "no occupied charging point fixture"
    );
    assert!(
        seen_cp_status.contains("1"),
        "no available charging point fixture"
    );
    assert!(
        seen_cp_status.contains("100"),
        "no unavailable charging point fixture"
    );
    assert!(
        seen_ev_status.contains("0"),
        "no occupied connector fixture"
    );
    assert!(
        seen_ev_status.contains("1"),
        "no available connector fixture"
    );
    assert!(
        seen_ev_status.contains(""),
        "no empty connector-status fixture"
    );
    assert!(seen_empty_locations, "no empty evLocationsData fixture");
    assert!(seen_empty_price, "no empty-price sentinel fixture");
    assert!(seen_free_price_type, "no free priceType fixture");
    assert!(seen_plug.contains("Type 2"));
    assert!(seen_plug.contains("Combo 2"));
    assert!(seen_price_type.contains("kWh"));
}
