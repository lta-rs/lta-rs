mod common;

use assert_float_eq::assert_float_absolute_eq;
use lta::operations::get_variable_message_signs::{
    decode_get_variable_message_signs_response, get_variable_message_signs_parts,
};
use lta::{
    Api, EquipmentId, GetVariableMessageSignsInput, GetVariableMessageSignsResponse, Latitude,
    Longitude, Vms,
};

#[test]
fn traffic_vms_request_preserves_optional_skip() {
    let request = get_variable_message_signs_parts(GetVariableMessageSignsInput::new())
        .expect("build request parts");
    assert_eq!(request.method, http::Method::GET);
    assert_eq!(request.uri, "/VMS");

    let request = get_variable_message_signs_parts(GetVariableMessageSignsInput::new().skip(500))
        .expect("build paged request parts");
    assert_eq!(request.uri, "/VMS?%24skip=500");

    let request = Api::new()
        .base_url("https://example.test/ltaodataservice/")
        .account_key("test-key")
        .traffic()
        .get_variable_message_signs()
        .skip(500)
        .request()
        .expect("build authenticated request");
    assert_eq!(
        request.uri(),
        "https://example.test/ltaodataservice/VMS?%24skip=500"
    );
    assert_eq!(request.headers()["AccountKey"], "test-key");
}

#[test]
fn traffic_vms_decodes_every_vendored_fixture() {
    for (path, body) in common::json_fixtures("vms_emas") {
        let response = satay_runtime::ResponseParts {
            status: http::StatusCode::OK,
            headers: http::HeaderMap::new(),
            body: body.as_ref(),
        };

        let decoded = decode_get_variable_message_signs_response(response)
            .unwrap_or_else(|error| panic!("failed to decode {}: {error}", path.display()));

        let GetVariableMessageSignsResponse::<Box<_>>::Ok(signs) = decoded else {
            panic!("expected a successful response for {}", path.display());
        };

        let mut with_message = 0;
        for sign in &signs {
            assert!(!sign.equipment_id.as_ref().is_empty());
            assert!((*sign.lat).is_finite());
            assert!((*sign.long).is_finite());
            if !sign.msg.is_empty() {
                with_message += 1;
            }
        }

        if path
            .file_name()
            .is_some_and(|name| name == "vms_emas_0.json")
        {
            assert_eq!(signs.len(), 12);
            assert_eq!(with_message, 9);

            let first = &signs[0];
            assert_eq!(first.equipment_id.as_ref(), "EVMS_RQ10");
            assert_float_absolute_eq!(*first.lat, 1.324_836_348_160_294);
            assert_float_absolute_eq!(*first.long, 103.872_033_883_177_38);
            assert_eq!(first.msg.as_ref(), "");

            let message_sign = &signs[5];
            assert_eq!(message_sign.equipment_id.as_ref(), "VMS_0011");
            assert_eq!(message_sign.msg.as_ref(), "FOLLOW TRAFFIC SIGNS,");
        }
    }
}

#[test]
fn traffic_vms_response_projects_and_parses_wire_fields() {
    let response = satay_runtime::ResponseParts {
        status: http::StatusCode::OK,
        headers: http::HeaderMap::new(),
        body: &br#"{
            "odata.metadata": "http://datamall2.mytransport.sg/ltaodataservice/$metadata#VMS",
            "value": [{
                "EquipmentID": "EVMS_RQ10",
                "Latitude": 1.324836348160294,
                "Longitude": 103.87203388317738,
                "Message": "PLS TURN ON,LOCAL RADIO,IN TUNNEL"
            }, {
                "EquipmentID": "AVMS_0006",
                "Latitude": 1.3117156532174947,
                "Longitude": 103.87520430818762,
                "Message": ""
            }]
        }"#[..],
    };

    let decoded =
        decode_get_variable_message_signs_response(response).expect("decode projected response");

    let GetVariableMessageSignsResponse::<Box<_>>::Ok(signs) = decoded else {
        panic!("expected successful Variable Message Signs response");
    };

    assert_eq!(signs.len(), 2);
    assert_eq!(signs[0].equipment_id.as_ref(), "EVMS_RQ10");
    assert_float_absolute_eq!(*signs[0].lat, 1.324_836_348_160_294);
    assert_float_absolute_eq!(*signs[0].long, 103.872_033_883_177_38);
    assert_eq!(signs[0].msg.as_ref(), "PLS TURN ON,LOCAL RADIO,IN TUNNEL");
    assert_eq!(signs[1].equipment_id.as_ref(), "AVMS_0006");
    assert_eq!(signs[1].msg.as_ref(), "");
}

#[test]
fn traffic_vms_serializes_to_canonical_wire_shape() {
    let sign = Vms::<Box<_>> {
        equipment_id: EquipmentId::try_from("EVMS_RQ10").expect("valid equipment id"),
        lat: Latitude::try_from(1.324_836_348_160_294).expect("valid latitude"),
        long: Longitude::try_from(103.872_033_883_177_38).expect("valid longitude"),
        msg: "PLS TURN ON,LOCAL RADIO,IN TUNNEL".into(),
    };

    let serialized = serde_json::to_value(sign).expect("serialize variable message sign");
    let object = serialized
        .as_object()
        .expect("variable message sign JSON object");

    assert_eq!(object.len(), 4);
    assert_eq!(object["EquipmentID"], serde_json::json!("EVMS_RQ10"));
    assert_eq!(object["Latitude"], serde_json::json!(1.324_836_348_160_294));
    assert_eq!(
        object["Longitude"],
        serde_json::json!(103.872_033_883_177_38)
    );
    assert_eq!(
        object["Message"],
        serde_json::json!("PLS TURN ON,LOCAL RADIO,IN TUNNEL")
    );
}

#[test]
fn traffic_vms_validates_and_preserves_equipment_ids() {
    assert_eq!(
        EquipmentId::try_from("EVMS_RQ10")
            .expect("valid equipment id")
            .as_ref(),
        "EVMS_RQ10"
    );
    for valid in ["AVMS_0006", "TID_0005", "VMS_0011"] {
        assert!(EquipmentId::try_from(valid).is_ok(), "{valid} rejected");
    }

    for malformed in [
        "vms_0011",  // lowercase prefix
        "VMS_11",    // short suffix
        "VMS_00110", // long suffix
        "VMS0011",   // missing underscore
        "VMS_001",   // short suffix with digit
        "VM_0011",   // short prefix
        "VMS__0011", // empty sign class
        "_0011",     // missing prefix
        "VMS_0011_", // trailing separator
    ] {
        assert!(
            EquipmentId::try_from(malformed).is_err(),
            "{malformed} accepted as an equipment id"
        );
    }
}
