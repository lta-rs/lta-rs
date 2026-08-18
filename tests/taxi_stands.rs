mod common;

use assert_float_eq::assert_float_absolute_eq;
use lta::operations::get_taxi_stands::{decode_get_taxi_stands_response, get_taxi_stands_parts};
use lta::{
    Api, GetTaxiStandsInput, GetTaxiStandsResponse, Latitude, Longitude, TaxiCode, TaxiStand,
    TaxiStandOwner, TaxiStandType,
};

#[test]
fn taxi_stands_request_preserves_optional_skip() {
    let request = get_taxi_stands_parts(GetTaxiStandsInput::new()).expect("build request parts");
    assert_eq!(request.method, http::Method::GET);
    assert_eq!(request.uri, "/TaxiStands");

    let request = get_taxi_stands_parts(GetTaxiStandsInput::new().skip(500))
        .expect("build paged request parts");
    assert_eq!(request.uri, "/TaxiStands?%24skip=500");

    let request = Api::new()
        .base_url("https://example.test/ltaodataservice/")
        .account_key("test-key")
        .taxi()
        .get_stands()
        .skip(500)
        .request()
        .expect("build authenticated request");
    assert_eq!(
        request.uri(),
        "https://example.test/ltaodataservice/TaxiStands?%24skip=500"
    );
    assert_eq!(request.headers()["AccountKey"], "test-key");
}

#[test]
fn taxi_stands_decodes_every_vendored_fixture() {
    for (path, body) in common::json_fixtures("taxi_stands") {
        let response = satay_runtime::ResponseParts {
            status: http::StatusCode::OK,
            headers: http::HeaderMap::new(),
            body,
        };
        let decoded = decode_get_taxi_stands_response(response)
            .unwrap_or_else(|error| panic!("failed to decode {}: {error}", path.display()));
        let GetTaxiStandsResponse::Ok(stands) = decoded else {
            panic!("expected a successful response for {}", path.display());
        };

        let mut barrier_free = 0;
        let mut lta_owned = 0;
        let mut ccs_owned = 0;
        let mut private_owned = 0;
        let mut stands_count = 0;
        let mut stops_count = 0;
        for stand in &stands {
            assert_eq!(stand.taxi_code.as_ref().len(), 3);
            assert!((*stand.lat).is_finite());
            assert!((*stand.long).is_finite());
            assert!(!stand.name.is_empty());
            assert!(!matches!(stand.owner, TaxiStandOwner::Other(_)));
            assert!(!matches!(stand.stand_type, TaxiStandType::Other(_)));
            if stand.is_barrier_free {
                barrier_free += 1;
            }
            match stand.owner {
                TaxiStandOwner::Lta => lta_owned += 1,
                TaxiStandOwner::Ccs => ccs_owned += 1,
                TaxiStandOwner::Private => private_owned += 1,
                TaxiStandOwner::Other(_) => {}
            }
            match stand.stand_type {
                TaxiStandType::Stand => stands_count += 1,
                TaxiStandType::Stop => stops_count += 1,
                TaxiStandType::Other(_) => {}
            }
        }

        if path
            .file_name()
            .is_some_and(|name| name == "taxi_stands_0.json")
        {
            assert_eq!(stands.len(), 279);
            assert_eq!(barrier_free, 257);
            assert_eq!((lta_owned, ccs_owned, private_owned), (159, 38, 82));
            assert_eq!((stands_count, stops_count), (230, 49));

            let first = &stands[0];
            assert_eq!(first.taxi_code.as_ref(), "A01");
            assert!(first.is_barrier_free);
            assert_eq!(first.owner, TaxiStandOwner::Lta);
            assert_eq!(first.stand_type, TaxiStandType::Stand);
            assert_eq!(first.name, "Orchard Rd along driveway of Lucky Plaza");
            assert_float_absolute_eq!(*first.lat, 1.303_888_888_888_888_9);
            assert_float_absolute_eq!(*first.long, 103.833_611_111_111_11);
        }
    }
}

#[test]
fn taxi_stands_response_projects_and_parses_wire_fields() {
    let response = satay_runtime::ResponseParts {
        status: http::StatusCode::OK,
        headers: http::HeaderMap::new(),
        body: br#"{
            "odata.metadata": "https://datamall2.mytransport.sg/ltaodataservice/$metadata#TaxiStands",
            "value": [{
                "TaxiCode": "A01",
                "Latitude": 1.3038888888888889,
                "Longitude": 103.83361111111111,
                "Bfa": "Yes",
                "Ownership": "LTA",
                "Type": "Stand",
                "Name": "Orchard Rd along driveway of Lucky Plaza"
            }, {
                "TaxiCode": "A13",
                "Latitude": 1.3005555555555557,
                "Longitude": 103.84222222222222,
                "Bfa": "No",
                "Ownership": "Private",
                "Type": "Stop",
                "Name": "Kramat Lane outside Concorde Hotel & Shopping Mall"
            }]
        }"#,
    };

    let decoded = decode_get_taxi_stands_response(response).expect("decode projected response");
    let GetTaxiStandsResponse::Ok(stands) = decoded else {
        panic!("expected successful Taxi Stands response");
    };

    assert_eq!(stands.len(), 2);
    assert_eq!(stands[0].taxi_code.as_ref(), "A01");
    assert!(stands[0].is_barrier_free);
    assert_eq!(stands[0].owner, TaxiStandOwner::Lta);
    assert_eq!(stands[0].stand_type, TaxiStandType::Stand);
    assert_eq!(stands[1].taxi_code.as_ref(), "A13");
    assert!(!stands[1].is_barrier_free);
    assert_eq!(stands[1].owner, TaxiStandOwner::Private);
    assert_eq!(stands[1].stand_type, TaxiStandType::Stop);
}

fn stand_with_is_barrier_free(is_barrier_free: &str) -> serde_json::Value {
    serde_json::json!({
        "TaxiCode": "A01",
        "Latitude": 1.303_888_888_888_888_9,
        "Longitude": 103.833_611_111_111_11,
        "Bfa": is_barrier_free,
        "Ownership": "LTA",
        "Type": "Stand",
        "Name": "Orchard Rd along driveway of Lucky Plaza"
    })
}

// lta_models' `from_str_to_bool` maps only "Y"/"Yes" to true and every other
// string to false. The configured mapping keeps that fallback via `unknown-as:
// false` while additionally accepting "1" and "true" as true and "N"/"No"/"0"/
// "false"/"" as false; DataMall only emits "Y" and "No" for is_barrier_free.
#[test]
fn taxi_stand_is_barrier_free_maps_configured_string_values() {
    for (wire, expected) in [("Yes", true), ("No", false)] {
        let decoded = serde_json::from_value::<TaxiStand>(stand_with_is_barrier_free(wire))
            .unwrap_or_else(|error| panic!("failed to decode is_barrier_free {wire:?}: {error}"));
        assert_eq!(
            decoded.is_barrier_free, expected,
            "is_barrier_free {wire:?}"
        );
    }
}

#[test]
fn taxi_stands_map_every_owner_and_stand_type() {
    for (wire, expected) in [
        ("LTA", TaxiStandOwner::Lta),
        ("CCS", TaxiStandOwner::Ccs),
        ("Private", TaxiStandOwner::Private),
    ] {
        let decoded = serde_json::from_value::<TaxiStandOwner>(serde_json::json!(wire))
            .unwrap_or_else(|error| panic!("failed to decode taxi stand owner {wire}: {error}"));
        assert_eq!(decoded, expected);
    }

    for (wire, expected) in [
        ("Stand", TaxiStandType::Stand),
        ("Stop", TaxiStandType::Stop),
    ] {
        let decoded = serde_json::from_value::<TaxiStandType>(serde_json::json!(wire))
            .unwrap_or_else(|error| panic!("failed to decode taxi stand type {wire}: {error}"));
        assert_eq!(decoded, expected);
    }
}

// Deviation from lta_models: its #[serde(other)] variants collapse every
// unrecognized value to the unit TaxiStandOwner::Unknown / TaxiStandType::Unknown.
// Satay open string enums preserve the unrecognized wire value in
// TaxiStandOwner::Other(String) / TaxiStandType::Other(String). DataMall's
// canonical values map to the same named public variants in both implementations.
#[test]
fn taxi_stands_preserve_unknown_owner_and_type_spellings() {
    let decoded = serde_json::from_value::<TaxiStandOwner>(serde_json::json!("CCS2"))
        .expect("decode unknown taxi stand owner");
    assert_eq!(decoded, TaxiStandOwner::Other("CCS2".into()));

    let decoded = serde_json::from_value::<TaxiStandType>(serde_json::json!("Bay"))
        .expect("decode unknown taxi stand type");
    assert_eq!(decoded, TaxiStandType::Other("Bay".into()));
}

#[test]
fn taxi_stand_serializes_to_canonical_wire_shape() {
    let stand = TaxiStand {
        taxi_code: TaxiCode::try_from("A01").expect("valid taxi code"),
        lat: Latitude::try_from(1.303_888_888_888_888_9).expect("valid latitude"),
        long: Longitude::try_from(103.833_611_111_111_11).expect("valid longitude"),
        is_barrier_free: true,
        owner: TaxiStandOwner::Lta,
        stand_type: TaxiStandType::Stand,
        name: "Orchard Rd along driveway of Lucky Plaza".into(),
    };

    let serialized = serde_json::to_value(stand).expect("serialize taxi stand");
    let object = serialized.as_object().expect("taxi stand JSON object");
    assert_eq!(object.len(), 7);
    assert_eq!(object["TaxiCode"], serde_json::json!("A01"));
    assert_eq!(
        object["Latitude"],
        serde_json::json!(1.303_888_888_888_888_9)
    );
    assert_eq!(
        object["Longitude"],
        serde_json::json!(103.833_611_111_111_11)
    );
    assert_eq!(object["Bfa"], serde_json::json!("Yes"));
    assert_eq!(object["Ownership"], serde_json::json!("LTA"));
    assert_eq!(object["Type"], serde_json::json!("Stand"));
    assert_eq!(
        object["Name"],
        serde_json::json!("Orchard Rd along driveway of Lucky Plaza")
    );

    let stand = TaxiStand {
        taxi_code: TaxiCode::try_from("A13").expect("valid taxi code"),
        lat: Latitude::try_from(1.300_555_555_555_555_7).expect("valid latitude"),
        long: Longitude::try_from(103.842_222_222_222_22).expect("valid longitude"),
        is_barrier_free: false,
        owner: TaxiStandOwner::Other("CCS2".into()),
        stand_type: TaxiStandType::Other("Bay".into()),
        name: "Kramat Lane outside Concorde Hotel & Shopping Mall".into(),
    };

    let object = serde_json::to_value(stand)
        .expect("serialize taxi stand")
        .as_object()
        .expect("taxi stand JSON object")
        .clone();

    assert_eq!(object["Bfa"], serde_json::json!("No"));
    assert_eq!(object["Ownership"], serde_json::json!("CCS2"));
    assert_eq!(object["Type"], serde_json::json!("Bay"));
}

#[test]
fn taxi_stands_validates_and_preserves_fixed_width_taxi_codes() {
    assert_eq!(
        TaxiCode::try_from("A01").expect("valid taxi code").as_ref(),
        "A01"
    );

    for malformed in ["A1", "A001", "AA1", "a01", "1A1", "A01X"] {
        assert!(
            TaxiCode::try_from(malformed).is_err(),
            "{malformed} accepted as a taxi stand code"
        );
    }

    let mut stand = stand_with_is_barrier_free("Yes");
    stand["TaxiCode"] = serde_json::json!("A1");
    assert!(
        serde_json::from_value::<TaxiStand>(stand).is_err(),
        "stand decoded with a malformed taxi code"
    );
}
