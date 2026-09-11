mod common;

use assert_float_eq::assert_float_absolute_eq;
use lta::operations::get_bike_parking::{decode_get_bike_parking_response, get_bike_parking_parts};
use lta::{
    Api, BicycleParking, GetBikeParkingInput, GetBikeParkingResponse, Latitude, Longitude, RackType,
};

#[test]
fn bike_parking_request_requires_lat_long_and_defaults_dist() {
    let lat = Latitude::try_from(1.3521).expect("valid latitude");
    let long = Longitude::try_from(103.8198).expect("valid longitude");

    let request =
        get_bike_parking_parts(GetBikeParkingInput::new(lat, long)).expect("build request parts");
    assert_eq!(request.method, http::Method::GET);
    assert_eq!(
        request.uri,
        "/BicycleParkingv2?Lat=1.3521&Long=103.8198&Dist=0.5"
    );

    let request = get_bike_parking_parts(GetBikeParkingInput::new(lat, long).dist(2.5))
        .expect("build request parts with explicit dist");
    assert_eq!(
        request.uri,
        "/BicycleParkingv2?Lat=1.3521&Long=103.8198&Dist=2.5"
    );

    let request = Api::new()
        .base_url("https://example.test/ltaodataservice/")
        .account_key("test-key")
        .traffic()
        .get_bike_parking(lat, long)
        .dist(2.5)
        .request()
        .expect("build authenticated request");
    assert_eq!(
        request.uri(),
        "https://example.test/ltaodataservice/BicycleParkingv2?Lat=1.3521&Long=103.8198&Dist=2.5"
    );
    assert_eq!(request.headers()["AccountKey"], "test-key");
}

#[test]
fn bike_parking_decodes_every_vendored_fixture() {
    for (path, body) in common::json_fixtures("bike_parking") {
        let response = satay_runtime::ResponseParts {
            status: http::StatusCode::OK,
            headers: http::HeaderMap::new(),
            body: body.as_ref(),
        };

        let decoded = decode_get_bike_parking_response(response)
            .unwrap_or_else(|error| panic!("failed to decode {}: {error}", path.display()));

        let GetBikeParkingResponse::<Box<_>>::Ok(parking) = decoded else {
            panic!("expected a successful response for {}", path.display());
        };

        let mut sheltered = 0;
        let mut other = 0;
        let mut yellow_box = 0;
        let mut yellow_box_private = 0;
        let mut racks_mrt = 0;
        let mut racks_bus_stop = 0;
        let mut racks_ura = 0;
        let mut racks_ava = 0;
        let mut racks_ite = 0;
        let mut racks_jtc = 0;
        let mut racks_pa = 0;
        let mut racks_nparks = 0;
        let mut racks_hdb = 0;
        let mut racks_nlb = 0;
        let mut racks_nea = 0;

        for location in &parking {
            assert!(!location.desc.is_empty());
            assert!((*location.lat).is_finite());
            assert!((*location.long).is_finite());

            if location.shelter_indicator {
                sheltered += 1;
            }

            match location.rack_type {
                RackType::<Box<_>>::YellowBox => yellow_box += 1,
                RackType::<Box<_>>::YellowBoxPrivate => yellow_box_private += 1,
                RackType::<Box<_>>::RacksMrt => racks_mrt += 1,
                RackType::<Box<_>>::RacksBusStop => racks_bus_stop += 1,
                RackType::<Box<_>>::RacksUra => racks_ura += 1,
                RackType::<Box<_>>::RacksAva => racks_ava += 1,
                RackType::<Box<_>>::RacksIte => racks_ite += 1,
                RackType::<Box<_>>::RacksJtc => racks_jtc += 1,
                RackType::<Box<_>>::RacksPa => racks_pa += 1,
                RackType::<Box<_>>::RacksNParks => racks_nparks += 1,
                RackType::<Box<_>>::RacksHdb => racks_hdb += 1,
                RackType::<Box<_>>::RacksNlb => racks_nlb += 1,
                RackType::<Box<_>>::RacksNea => racks_nea += 1,
                RackType::<Box<_>>::Other(_) => other += 1,
            }
        }

        if path
            .file_name()
            .is_some_and(|name| name == "bike_parking_0.json")
        {
            assert_eq!(parking.len(), 2759);
            assert_eq!(sheltered, 2481);
            assert_eq!(other, 0);
            assert_eq!(
                [
                    (RackType::<Box<_>>::YellowBox, yellow_box),
                    (RackType::<Box<_>>::YellowBoxPrivate, yellow_box_private),
                    (RackType::<Box<_>>::RacksMrt, racks_mrt),
                    (RackType::<Box<_>>::RacksBusStop, racks_bus_stop),
                    (RackType::<Box<_>>::RacksUra, racks_ura),
                    (RackType::<Box<_>>::RacksAva, racks_ava),
                    (RackType::<Box<_>>::RacksIte, racks_ite),
                    (RackType::<Box<_>>::RacksJtc, racks_jtc),
                    (RackType::<Box<_>>::RacksPa, racks_pa),
                    (RackType::<Box<_>>::RacksNParks, racks_nparks),
                    (RackType::<Box<_>>::RacksHdb, racks_hdb),
                    (RackType::<Box<_>>::RacksNlb, racks_nlb),
                    (RackType::<Box<_>>::RacksNea, racks_nea),
                ],
                [
                    (RackType::YellowBox, 876),
                    (RackType::YellowBoxPrivate, 1),
                    (RackType::RacksMrt, 33),
                    (RackType::RacksBusStop, 1),
                    (RackType::RacksUra, 4),
                    (RackType::RacksAva, 1),
                    (RackType::RacksIte, 1),
                    (RackType::RacksJtc, 1),
                    (RackType::RacksPa, 5),
                    (RackType::RacksNParks, 8),
                    (RackType::RacksHdb, 1826),
                    (RackType::RacksNlb, 1),
                    (RackType::RacksNea, 1),
                ]
            );
        }
    }
}

#[test]
fn bike_parking_response_projects_and_parses_wire_fields() {
    let response = satay_runtime::ResponseParts {
        status: http::StatusCode::OK,
        headers: http::HeaderMap::new(),
        body: &br#"{
            "odata.metadata": "https://datamall2.mytransport.sg/ltaodataservice/$metadata#BicycleParkingv2",
            "value": [{
                "Description": "Bukit Panjang Road Block 403_YB",
                "Latitude": 1.3806436929483808,
                "Longitude": 103.76771974367475,
                "RackType": "Yellow Box",
                "RackCount": 16,
                "ShelterIndicator": "Y"
            }, {
                "Description": "Bus Stop 28259",
                "Latitude": 1.3340881696956965,
                "Longitude": 103.73705894621091,
                "RackType": "Racks_HDB",
                "RackCount": 10,
                "ShelterIndicator": "N"
            }]
        }"#[..],
    };

    let decoded = decode_get_bike_parking_response(response).expect("decode projected response");
    let GetBikeParkingResponse::<Box<_>>::Ok(parking) = decoded else {
        panic!("expected successful Bicycle Parking response");
    };

    assert_eq!(parking.len(), 2);
    assert_eq!(parking[0].desc.as_ref(), "Bukit Panjang Road Block 403_YB");
    assert_eq!(parking[0].rack_type, RackType::YellowBox);
    assert_eq!(parking[0].rack_count, 16);
    assert!(parking[0].shelter_indicator);
    assert_float_absolute_eq!(*parking[0].lat, 1.380_643_692_948_380_8);
    assert_float_absolute_eq!(*parking[0].long, 103.767_719_743_674_75);
    assert_eq!(parking[1].desc.as_ref(), "Bus Stop 28259");
    assert_eq!(parking[1].rack_type, RackType::RacksHdb);
    assert_eq!(parking[1].rack_count, 10);
    assert!(!parking[1].shelter_indicator);
}

fn parking_with_shelter_indicator(shelter_indicator: &str) -> serde_json::Value {
    serde_json::json!({
        "Description": "Bukit Panjang Road Block 403_YB",
        "Latitude": 1.380_643_692_948_380_8,
        "Longitude": 103.767_719_743_674_75,
        "RackType": "Yellow Box",
        "RackCount": 16,
        "ShelterIndicator": shelter_indicator
    })
}

// lta_models' `from_str_to_bool` maps only "Y"/"Yes" to true and every other
// string to false. The configured mapping reproduces that fallback exactly via
// `unknown-as: false`, with "N"/"No"/"0"/"false"/"" also listed as false;
// DataMall only emits "Y", "N", and "" for shelter_indicator.
#[test]
fn bike_parking_shelter_indicator_maps_configured_string_values() {
    for (wire, expected) in [("Y", true), ("N", false), ("", false)] {
        let decoded =
            serde_json::from_value::<BicycleParking>(parking_with_shelter_indicator(wire))
                .unwrap_or_else(|error| {
                    panic!("failed to decode shelter_indicator {wire:?}: {error}")
                });
        assert_eq!(
            decoded.shelter_indicator, expected,
            "shelter_indicator {wire:?}"
        );
    }
}

#[test]
fn bike_parking_map_every_rack_type() {
    for (wire, expected) in [
        ("Yellow Box", RackType::YellowBox),
        ("Yellow Box_Private", RackType::YellowBoxPrivate),
        ("Racks_MRT", RackType::RacksMrt),
        ("Racks_Bus Stop", RackType::RacksBusStop),
        ("Racks_URA", RackType::RacksUra),
        ("Racks_AVA", RackType::RacksAva),
        ("Racks_ITE", RackType::RacksIte),
        ("Racks_JTC", RackType::RacksJtc),
        ("Racks_PA", RackType::RacksPa),
        ("Racks_NParks", RackType::RacksNParks),
        ("Racks_HDB", RackType::RacksHdb),
        ("Racks_NLB", RackType::RacksNlb),
        ("Racks_NEA", RackType::RacksNea),
    ] {
        let decoded = serde_json::from_value::<RackType>(serde_json::json!(wire))
            .unwrap_or_else(|error| panic!("failed to decode rack type {wire}: {error}"));
        assert_eq!(decoded, expected);
        assert_eq!(decoded.as_str(), wire, "rack type {wire} round-trips");
    }
}

// Deviation from lta_models: its #[serde(other)] variant collapses every
// unrecognized value to the unit RackType::Unknown. Satay open string enums
// preserve the unrecognized wire value in RackType::Other(String). The variant
// names also use satay's upper-camel acronym casing (RacksMrt, RacksHdb) like
// the accepted StationCode deviation, while wire serialization stays identical.
#[test]
fn bike_parking_preserve_unknown_rack_type_spellings() {
    let decoded = serde_json::from_value::<RackType>(serde_json::json!("Racks_XYZ"))
        .expect("decode unknown rack type");
    assert_eq!(decoded, RackType::Other("Racks_XYZ".into()));
    assert_eq!(decoded.as_str(), "Racks_XYZ");
}

#[test]
fn bike_parking_serializes_to_canonical_wire_shape() {
    let parking = BicycleParking {
        desc: "Bukit Panjang Road Block 403_YB".to_string(),
        lat: Latitude::try_from(1.380_643_692_948_380_8).expect("valid latitude"),
        long: Longitude::try_from(103.767_719_743_674_75).expect("valid longitude"),
        rack_type: RackType::YellowBox,
        rack_count: 16,
        shelter_indicator: true,
    };

    let object = serde_json::to_value(parking)
        .expect("serialize bicycle parking")
        .as_object()
        .expect("bicycle parking JSON object")
        .clone();
    assert_eq!(object.len(), 6);
    assert_eq!(
        object["Description"],
        serde_json::json!("Bukit Panjang Road Block 403_YB")
    );
    assert_eq!(
        object["Latitude"],
        serde_json::json!(1.380_643_692_948_380_8)
    );
    assert_eq!(
        object["Longitude"],
        serde_json::json!(103.767_719_743_674_75)
    );
    assert_eq!(object["RackType"], serde_json::json!("Yellow Box"));
    assert_eq!(object["RackCount"], serde_json::json!(16));
    assert_eq!(object["ShelterIndicator"], serde_json::json!("Y"));

    let parking = BicycleParking {
        desc: "Bus Stop 28259".to_string(),
        lat: Latitude::try_from(1.334_088_169_695_696_5).expect("valid latitude"),
        long: Longitude::try_from(103.737_058_946_210_91).expect("valid longitude"),
        rack_type: RackType::Other("Racks_XYZ".to_string()),
        rack_count: 10,
        shelter_indicator: false,
    };

    let object = serde_json::to_value(parking)
        .expect("serialize bicycle parking")
        .as_object()
        .expect("bicycle parking JSON object")
        .clone();
    assert_eq!(object["RackType"], serde_json::json!("Racks_XYZ"));
    assert_eq!(object["ShelterIndicator"], serde_json::json!("N"));
}
#[test]
fn bike_parking_validates_coordinate_bounds() {
    assert!(Latitude::try_from(-90.0).is_ok());
    assert!(Latitude::try_from(90.0).is_ok());
    for invalid in [-90.1, 90.1, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(
            Latitude::try_from(invalid).is_err(),
            "{invalid} accepted as a latitude"
        );
    }

    assert!(Longitude::try_from(-180.0).is_ok());
    assert!(Longitude::try_from(180.0).is_ok());
    for invalid in [-180.1, 180.1] {
        assert!(
            Longitude::try_from(invalid).is_err(),
            "{invalid} accepted as a longitude"
        );
    }
}
