use lta::operations::get_facilities_maintenance::get_facilities_maintenance_parts;
use lta::{Api, GetFacilitiesMaintenanceInput};

#[test]
fn facilities_maintenance_v2_request_omits_skip_by_default() {
    let request = get_facilities_maintenance_parts(GetFacilitiesMaintenanceInput::new())
        .expect("build request parts");
    assert_eq!(request.method, http::Method::GET);
    assert_eq!(request.uri, "/v2/FacilitiesMaintenance");

    let request = Api::new()
        .base_url("https://example.test/ltaodataservice/")
        .account_key("test-key")
        .facility()
        .get_facilities_maintenance()
        .request()
        .expect("build authenticated request");
    assert_eq!(
        request.uri(),
        "https://example.test/ltaodataservice/v2/FacilitiesMaintenance"
    );
    assert_eq!(request.headers()["AccountKey"], "test-key");
}

#[test]
fn facilities_maintenance_v2_request_serializes_skip() {
    let request = get_facilities_maintenance_parts(GetFacilitiesMaintenanceInput::new().skip(500))
        .expect("build request parts");
    assert_eq!(request.uri, "/v2/FacilitiesMaintenance?%24skip=500");

    let request = Api::new()
        .base_url("")
        .facility()
        .get_facilities_maintenance()
        .skip(1_000)
        .request()
        .expect("build request");
    assert_eq!(request.uri(), "/v2/FacilitiesMaintenance?%24skip=1000");
}
