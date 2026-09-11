mod common;

use assert_float_eq::assert_float_absolute_eq;
use lta::operations::get_traffic_images::{
    decode_get_traffic_images_response, get_traffic_images_parts,
};
use lta::{
    Api, CameraId, GetTrafficImagesInput, GetTrafficImagesResponse, Latitude, Longitude,
    TrafficImage,
};
use reqwest::Url;

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
fn traffic_images_decodes_every_vendored_fixture() {
    for (path, body) in common::json_fixtures("traffic_images") {
        let response = satay_runtime::ResponseParts {
            status: http::StatusCode::OK,
            headers: http::HeaderMap::new(),
            body: body.as_ref(),
        };

        let decoded = decode_get_traffic_images_response(response)
            .unwrap_or_else(|error| panic!("failed to decode {}: {error}", path.display()));

        let GetTrafficImagesResponse::Ok(images) = decoded else {
            panic!("expected a successful response for {}", path.display());
        };

        for image in &images {
            assert_eq!(image.camera_id.as_ref().len(), 4);
            assert!((*image.lat).is_finite());
            assert!((*image.long).is_finite());
        }

        if path
            .file_name()
            .is_some_and(|name| name == "traffic_images_0.json")
        {
            assert_eq!(images.len(), 90);
            let first = &images[0];
            assert_eq!(first.camera_id.as_ref(), "1001");
            assert_float_absolute_eq!(*first.lat, 1.295_313_32);
            assert_float_absolute_eq!(*first.long, 103.871_146);
            // assert!(first.image_link.contains(
            //     "https://dm-traffic-camera-itsc.s3.ap-southeast-1.amazonaws.com/2023-04-05/23-05/1001_"
            // ));
        }
    }
}

#[test]
fn traffic_images_response_projects_wire_fields() {
    let response = satay_runtime::ResponseParts {
        status: http::StatusCode::OK,
        headers: http::HeaderMap::new(),
        body: &br#"{
            "odata.metadata": "https://datamall2.mytransport.sg/ltaodataservice/$metadata#Traffic-Imagesv2",
            "value": [{
                "CameraID": "1701",
                "Latitude": 1.323604823,
                "Longitude": 103.8587802,
                "ImageLink": "https://example.test/1701.jpg"
            }]
        }"#[..],
    };

    let decoded = decode_get_traffic_images_response(response).expect("decode projected response");
    let GetTrafficImagesResponse::Ok(images) = decoded else {
        panic!("expected successful Traffic Images response");
    };

    assert_eq!(images.len(), 1);
    let image = &images[0];
    assert_eq!(image.camera_id.as_ref(), "1701");
    assert_float_absolute_eq!(*image.lat, 1.323_604_823);
    assert_float_absolute_eq!(*image.long, 103.858_780_2);
    assert_eq!(
        image.image_link,
        Url::parse("https://example.test/1701.jpg").unwrap()
    );
}

#[test]
fn traffic_image_serializes_to_canonical_wire_shape() {
    let image = TrafficImage {
        camera_id: CameraId::try_from("1701").expect("valid camera id"),
        lat: Latitude::try_from(1.323_604_823).expect("valid latitude"),
        long: Longitude::try_from(103.858_780_2).expect("valid longitude"),
        image_link: Url::parse("https://example.test/1701.jpg").unwrap(),
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

#[test]
fn traffic_image_validates_identifiers_and_coordinate_bounds() {
    for malformed in ["101", "01001", "10A1"] {
        assert!(
            CameraId::try_from(malformed).is_err(),
            "{malformed} accepted as a camera id"
        );
    }

    assert!(Latitude::try_from(-90.0).is_ok());
    assert!(Latitude::try_from(90.0).is_ok());
    assert!(Latitude::try_from(-90.1).is_err());
    assert!(Latitude::try_from(90.1).is_err());
    assert!(Longitude::try_from(-180.0).is_ok());
    assert!(Longitude::try_from(180.0).is_ok());
    assert!(Longitude::try_from(-180.1).is_err());
    assert!(Longitude::try_from(180.1).is_err());
}
