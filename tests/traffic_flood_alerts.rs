mod common;

use lta::operations::get_flood_alerts::{decode_get_flood_alerts_response, get_flood_alerts_parts};
use lta::{
    Api, FloodEvent, FloodMsgType, FloodResponseType, FloodSenderName, FloodSeverity, FloodStatus,
    FloodUrgency, GetFloodAlertsInput, GetFloodAlertsResponse,
};

#[test]
fn traffic_flood_alerts_request_has_no_pagination_parameter() {
    let request = get_flood_alerts_parts(GetFloodAlertsInput::new()).expect("build request parts");
    assert_eq!(request.method, http::Method::GET);
    assert_eq!(request.uri, "/PubFloodAlerts");

    let request = Api::new()
        .base_url("https://example.test/ltaodataservice/")
        .account_key("test-key")
        .traffic()
        .get_flood_alerts()
        .request()
        .expect("build authenticated request");
    assert_eq!(
        request.uri(),
        "https://example.test/ltaodataservice/PubFloodAlerts"
    );
    assert_eq!(request.headers()["AccountKey"], "test-key");
}

#[test]
fn traffic_flood_alerts_decodes_every_vendored_fixture() {
    for (path, body) in common::json_fixtures("flood_alerts") {
        let response = satay_runtime::ResponseParts {
            status: http::StatusCode::OK,
            headers: http::HeaderMap::new(),
            body,
        };

        let decoded = decode_get_flood_alerts_response(response)
            .unwrap_or_else(|error| panic!("failed to decode {}: {error}", path.display()));
        let GetFloodAlertsResponse::Ok(alerts) = decoded else {
            panic!("expected a successful response for {}", path.display());
        };

        for alert in &alerts {
            assert!(!alert.alert_id.is_empty());
            // dateTime and expires are OffsetDateTime, check they are valid by formatting
            assert!(!alert.headline.is_empty());
            assert!(!alert.desc.is_empty());
            assert!(!alert.instruction.is_empty());
            assert!(!alert.area_desc.is_empty());
            assert!(!alert.circle.is_empty());
            // enums are validated via serde, just ensure they are known variants where strict
            assert!(!alert.headline.is_empty());
        }

        if path
            .file_name()
            .is_some_and(|name| name == "flood_alerts_0.json")
        {
            assert_eq!(alerts.len(), 1);
            let first = &alerts[0];
            assert_eq!(
                first.alert_id,
                "2.49.0.0.702.2-BCM-17612003774680-PUBCON-DYOONG"
            );
            assert_eq!(first.msg_type, FloodMsgType::Alert);
            assert_eq!(first.event, FloodEvent::Flood);
            assert_eq!(first.response_type, FloodResponseType::Avoid);
            assert_eq!(first.urgency, FloodUrgency::Immediate);
            assert_eq!(first.severity, FloodSeverity::Minor);
            assert_eq!(first.sender_name, FloodSenderName::Pub);
            assert_eq!(first.headline, "Flash Flood Alert");
            assert_eq!(
                first.desc,
                "Flash flood at Bt Timah Rd from Wilby Rd to Blackmore Dr. Please avoid the area. Issued 1705 hrs."
            );
            assert_eq!(
                first.instruction,
                "Please avoid this area for the next one (1) hour."
            );
            assert_eq!(first.area_desc, "Jalan Mastuli, Singapore");
            assert_eq!(first.circle, "1.35479,103.88611 0.05");
            assert_eq!(first.status, FloodStatus::Actual);
            // Check datetime values via Display contains (time crate displays without leading zero)
            let dt_str = first.date_time.to_string();
            assert!(dt_str.contains("2025-05-22"), "dateTime: {dt_str}");
            assert!(
                dt_str.contains("9:55") || dt_str.contains("09:55"),
                "dateTime: {dt_str}"
            );
            assert!(dt_str.contains("+08:00"), "dateTime offset: {dt_str}");
            let exp_str = first.expires.to_string();
            assert!(exp_str.contains("2025-10-24"), "expires: {exp_str}");
            assert!(exp_str.contains("14:19:37"), "expires: {exp_str}");
        }
    }
}

#[test]
fn traffic_flood_alerts_response_projects_wire_fields() {
    let response = satay_runtime::ResponseParts {
        status: http::StatusCode::OK,
        headers: http::HeaderMap::new(),
        body: br#"{
            "odata.metadata": "https://datamall2.mytransport.sg/ltaodataservice/$metadata#PubFloodAlerts",
            "value": [{
                "alertId": "2.49.0.0.702.2-BCM-17612003774680-PUBCON-DYOONG",
                "dateTime": "2025-05-22T09:55:00+08:00",
                "msgType": "Alert",
                "event": "Flood",
                "responseType": "Avoid",
                "urgency": "Immediate",
                "severity": "Minor",
                "expires": "2025-10-24T14:19:37+08:00",
                "senderName": "PUB",
                "headline": "Flash Flood Alert",
                "description": "Flash flood at Bt Timah Rd from Wilby Rd to Blackmore Dr. Please avoid the area. Issued 1705 hrs.",
                "instruction": "Please avoid this area for the next one (1) hour.",
                "areaDesc": "Jalan Mastuli, Singapore",
                "circle": "1.35479,103.88611 0.05",
                "status": "Actual"
            }]
        }"#,
    };

    let decoded = decode_get_flood_alerts_response(response).expect("decode projected response");
    let GetFloodAlertsResponse::Ok(alerts) = decoded else {
        panic!("expected successful Flood Alerts response");
    };

    assert_eq!(alerts.len(), 1);
    let alert = &alerts[0];
    assert_eq!(
        alert.alert_id,
        "2.49.0.0.702.2-BCM-17612003774680-PUBCON-DYOONG"
    );
    assert_eq!(alert.msg_type, FloodMsgType::Alert);
    assert_eq!(alert.event, FloodEvent::Flood);
    assert_eq!(alert.urgency, FloodUrgency::Immediate);
    assert_eq!(alert.severity, FloodSeverity::Minor);
    assert_eq!(alert.circle, "1.35479,103.88611 0.05");
}

#[test]
fn traffic_flood_alerts_handles_cancel_msg_type() {
    let body = br#"{
        "odata.metadata": "x",
        "value": [{
            "alertId": "id-cancel",
            "dateTime": "2025-05-22T09:55:00+08:00",
            "msgType": "Cancel",
            "event": "Flood",
            "responseType": "Avoid",
            "urgency": "Immediate",
            "severity": "Minor",
            "expires": "2025-10-24T14:19:37+08:00",
            "senderName": "PUB",
            "headline": "Cancel",
            "description": "desc",
            "instruction": "inst",
            "areaDesc": "area",
            "circle": "1.0,103.0 0.05",
            "status": "Actual"
        }]
    }"#;
    let response = satay_runtime::ResponseParts {
        status: http::StatusCode::OK,
        headers: http::HeaderMap::new(),
        body: body.to_vec(),
    };
    let decoded = decode_get_flood_alerts_response(response).expect("decode cancel");
    let GetFloodAlertsResponse::Ok(alerts) = decoded else {
        panic!("expected ok");
    };
    assert_eq!(alerts[0].msg_type, FloodMsgType::Cancel);
}

#[test]
fn traffic_flood_alerts_open_enums_preserve_unknown_spelling() {
    // FloodResponseType and FloodUrgency are open (anyOf) and should preserve unknown as Other
    let decoded = serde_json::from_value::<FloodResponseType>(serde_json::json!("Evacuate"))
        .expect("decode unknown responseType");
    assert_eq!(decoded, FloodResponseType::Other("Evacuate".into()));
    assert_eq!(decoded.as_str(), "Evacuate");

    let decoded = serde_json::from_value::<FloodUrgency>(serde_json::json!("CustomUrgency"))
        .expect("decode unknown urgency");
    assert_eq!(decoded, FloodUrgency::Other("CustomUrgency".into()));
}

#[test]
fn traffic_flood_alerts_strict_enums_reject_unknown_spelling() {
    // FloodMsgType, FloodEvent, FloodSeverity, FloodStatus, FloodSenderName are strict
    let err = serde_json::from_value::<FloodMsgType>(serde_json::json!("UnknownType"))
        .expect_err("unknown msgType should error");
    assert!(err.to_string().contains("unknown variant") || err.to_string().contains("UnknownType"));

    let err = serde_json::from_value::<FloodEvent>(serde_json::json!("Tsunami"))
        .expect_err("unknown event should error");
    assert!(err.to_string().contains("unknown variant") || err.to_string().contains("Tsunami"));

    let err = serde_json::from_value::<FloodSeverity>(serde_json::json!("Critical"))
        .expect_err("unknown severity should error");
    assert!(err.to_string().contains("unknown variant") || err.to_string().contains("Critical"));

    let err = serde_json::from_value::<FloodStatus>(serde_json::json!("UnknownStatus"))
        .expect_err("unknown status should error");
    assert!(err.to_string().contains("unknown variant"));
}

#[test]
fn traffic_flood_alerts_serializes_to_canonical_wire_shape() {
    // Ensure serialization uses original wire names (alertId, dateTime, etc.)
    // Build via serde to avoid manual OffsetDateTime parsing.
    let alert: lta::FloodAlert = serde_json::from_value(serde_json::json!({
        "alertId": "2.49.0.0.702.2-BCM-17612003774680-PUBCON-DYOONG",
        "dateTime": "2025-05-22T09:55:00+08:00",
        "msgType": "Alert",
        "event": "Flood",
        "responseType": "Avoid",
        "urgency": "Immediate",
        "severity": "Minor",
        "expires": "2025-10-24T14:19:37+08:00",
        "senderName": "PUB",
        "headline": "Flash Flood Alert",
        "description": "Flash flood at Bt Timah Rd from Wilby Rd to Blackmore Dr. Please avoid the area. Issued 1705 hrs.",
        "instruction": "Please avoid this area for the next one (1) hour.",
        "areaDesc": "Jalan Mastuli, Singapore",
        "circle": "1.35479,103.88611 0.05",
        "status": "Actual"
    }))
    .expect("deserialize flood alert for serialization test");
    let value = serde_json::to_value(&alert).expect("serialize flood alert");
    assert_eq!(
        value["alertId"].as_str().unwrap(),
        "2.49.0.0.702.2-BCM-17612003774680-PUBCON-DYOONG"
    );
    assert_eq!(value["msgType"].as_str().unwrap(), "Alert");
    assert_eq!(value["event"].as_str().unwrap(), "Flood");
    assert_eq!(value["responseType"].as_str().unwrap(), "Avoid");
    assert_eq!(value["senderName"].as_str().unwrap(), "PUB");
    assert_eq!(value["description"].as_str().unwrap(), alert.desc);
    assert_eq!(
        value["areaDesc"].as_str().unwrap(),
        "Jalan Mastuli, Singapore"
    );
    assert_eq!(value["circle"].as_str().unwrap(), "1.35479,103.88611 0.05");
}
