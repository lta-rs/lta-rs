use std::fs;

use assert_float_eq::assert_float_absolute_eq;
use lta::operations::get_traffic_images::{
    decode_get_traffic_images_response, get_traffic_images_parts,
};
use lta::{Api, GetTrafficImagesInput, GetTrafficImagesResponse, TrafficImage};

#[test]
fn traffic_images_request_preserves_optional_skip() {
    let request =
        get_traffic_images_parts(GetTrafficImagesInput::new()).expect("build request parts");
    assert_eq!(request.method, http::Method::GET);
    assert_eq!(request.uri, "/Traffic-Imagesv2");

    let request = get_traffic_images_parts(GetTrafficImagesInput::new().skip(500))
        .expect("build paged request parts");
    assert_eq!(request.uri, "/Traffic-Imagesv2?%24skip=500");

    let request = Api::new()
        .base_url("https://example.test/ltaodataservice/")
        .account_key("test-key")
        .traffic()
        .get_images()
        .skip(500)
        .request()
        .expect("build authenticated request");
    assert_eq!(
        request.uri(),
        "https://example.test/ltaodataservice/Traffic-Imagesv2?%24skip=500"
    );
    assert_eq!(request.headers()["AccountKey"], "test-key");
}

#[test]
fn traffic_images_decodes_vendored_fixture() {
    let raw = fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/traffic_images.json"
    ))
    .expect("read vendored fixture");
    let response = satay_runtime::ResponseParts {
        status: http::StatusCode::OK,
        headers: http::HeaderMap::new(),
        body: raw.as_bytes(),
    };

    let decoded = decode_get_traffic_images_response(response).expect("decode fixture");
    let GetTrafficImagesResponse::Ok(images) = decoded else {
        panic!("expected successful Traffic Images response");
    };

    assert_eq!(images.len(), 90);
    for image in &images {
        assert!(image.camera_id > 0);
        assert!(image.lat.is_finite());
        assert!(image.long.is_finite());
        assert!(!image.image_link.is_empty());
    }

    let first = &images[0];
    assert_eq!(first.camera_id, 1001);
    assert_float_absolute_eq!(first.lat, 1.295_313_32);
    assert_float_absolute_eq!(first.long, 103.871_146);
    assert!(first.image_link.starts_with(
        "https://dm-traffic-camera-itsc.s3.ap-southeast-1.amazonaws.com/2023-04-05/23-05/1001_"
    ));
}

#[test]
fn traffic_images_response_projects_wire_fields() {
    let response = satay_runtime::ResponseParts {
        status: http::StatusCode::OK,
        headers: http::HeaderMap::new(),
        body: br#"{
            "odata.metadata": "https://datamall2.mytransport.sg/ltaodataservice/$metadata#Traffic-Imagesv2",
            "value": [{
                "CameraID": "1701",
                "Latitude": 1.323604823,
                "Longitude": 103.8587802,
                "ImageLink": "https://example.test/1701.jpg"
            }]
        }"#,
    };

    let decoded = decode_get_traffic_images_response(response).expect("decode projected response");
    let GetTrafficImagesResponse::Ok(images) = decoded else {
        panic!("expected successful Traffic Images response");
    };

    assert_eq!(images.len(), 1);
    let image = &images[0];
    assert_eq!(image.camera_id, 1701);
    assert_float_absolute_eq!(image.lat, 1.323_604_823);
    assert_float_absolute_eq!(image.long, 103.858_780_2);
    assert_eq!(image.image_link, "https://example.test/1701.jpg");
}

#[test]
fn traffic_image_serializes_to_canonical_wire_shape() {
    let image = TrafficImage {
        camera_id: 1701,
        lat: 1.323_604_823,
        long: 103.858_780_2,
        image_link: "https://example.test/1701.jpg".into(),
    };

    let serialized = serde_json::to_value(image).expect("serialize traffic image");
    let object = serialized.as_object().expect("traffic image JSON object");

    assert_eq!(object["CameraID"], serde_json::json!("1701"));
    assert_eq!(object["Latitude"], serde_json::json!(1.323_604_823));
    assert_eq!(object["Longitude"], serde_json::json!(103.858_780_2));
    assert_eq!(
        object["ImageLink"],
        serde_json::json!("https://example.test/1701.jpg")
    );
    for key in ["CameraID", "Latitude", "Longitude", "ImageLink"] {
        assert!(object.contains_key(key));
    }
    for key in ["camera_id", "lat", "long", "image_link"] {
        assert!(!object.contains_key(key));
    }
}
