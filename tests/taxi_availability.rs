mod common;

use lta::GetTaxiAvailabilityResponse;
use lta::operations::get_taxi_availability::decode_get_taxi_availability_response;

#[test]
fn taxi_availability_decodes_every_vendored_fixture() {
    for (path, body) in common::json_fixtures("taxi_availability") {
        let response = satay_runtime::ResponseParts {
            status: http::StatusCode::OK,
            headers: http::HeaderMap::new(),
            body,
        };
        let decoded = decode_get_taxi_availability_response(response)
            .unwrap_or_else(|error| panic!("failed to decode {}: {error}", path.display()));
        let GetTaxiAvailabilityResponse::Ok(taxis) = decoded else {
            panic!("expected a successful response for {}", path.display());
        };

        for taxi in taxis {
            assert!(
                (*taxi.lat).is_finite(),
                "invalid latitude in {}",
                path.display()
            );
            assert!(
                (*taxi.long).is_finite(),
                "invalid longitude in {}",
                path.display()
            );
        }
    }
}
