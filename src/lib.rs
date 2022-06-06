#![doc = include_str!("../README.md")]

/// Helper macro to general API URL at compile time
#[macro_export]
macro_rules! api_url {
    ($e: expr) => {
        concat!("http://datamall2.mytransport.sg/ltaodataservice", $e)
    };
}

pub use crate::r#async::prelude::*;
pub use crate::r#async::LTAClient;
pub use lta_models as models;
use reqwest::StatusCode;
use thiserror::Error;

/// Imports for important structs
pub mod prelude {
    pub use crate::{Bus, Crowd, Facility, Geo, Taxi, Traffic, Train};
}

use crate::models::crowd::passenger_vol::VolType;
pub use reqwest;

/// Internal Async module
pub mod r#async;

/// Internal Blocking module
#[cfg(feature = "blocking")]
pub mod blocking;

/// Type alias for `Result<T, LTAError>`
pub type LTAResult<T> = Result<T, LTAError>;

/// LTAError type, all request using lta-rs returns `Result<T, LTAError>`
#[derive(Error, Debug)]
pub enum LTAError {
    /// Internal error within the client backend, open a PR if this happens
    #[error("Internal error within the client backend, open a PR if this happens!")]
    BackendError(#[from] reqwest::Error),
    
    /// API key is most likely empty
    #[error("Invalid API Key!")]
    InvalidAPIKey,

    /// You have reached the server limit, try again later
    #[error("Server rate limit reached!")]
    RateLimitReached,

    /// Response body can't be parsed to a valid enum
    #[error("Unknown enum variant!")]
    UnknownEnumVariant,

    /// Make sure that your API key is correct and valid
    #[error("HTTP Header Unauthorized")]
    Unauthorized,

    /// HTTP NOTFOUND
    #[error("HTTP Header NotFound")]
    NotFound,

    /// Failed to parse body of response, probably malformed
    #[error("Failed to parse body of response, probably malformed")]
    FailedToParseBody,

    /// Undocumented status code, open an issue if this happens
    #[error("Undocumented status code, open an issue if this happens")]
    UnhandledStatusCode(StatusCode, String),

    /// Custom
    #[error("Custom error: `{0}`")]
    Custom(String),
}

/// A `Client` to make requests with
/// The `Client` holds a connection pool internally, so it is advised that you create one and reuse it
pub trait Client: Sized {
    /// Any backend Client
    type InternalClient;

    /// Any type that can build requests
    type RB;

    /// General constructor for `Self`
    fn new(api_key: impl Into<String>, client: Self::InternalClient) -> Self;

    /// This method not assign the `api_key` in struct if the provided key is empty or whitespaces
    /// Instead, assign `None`
    fn with_api_key(api_key: impl Into<String>) -> LTAResult<Self>;

    /// Returns `Self::RB`
    fn req_builder(&self, url: &str) -> Self::RB;
}

/// Bus type that implements APIs. Can be either blocking or async
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Bus;

/// Crowd type that implements APIs. Can be either blocking or async
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Crowd;

/// Taxi type that implements APIs. Can be either blocking or async
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Taxi;

/// Traffic type that implements APIs. Can be either blocking or async
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Traffic;

/// Train type that implements APIs. Can be either blocking or async
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Train;

/// Geo type that implements APIs. Can be either blocking or async
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Geo;

/// Facility type that implements APIs. Can be either blocking or async
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Facility;

/// util to map enum to url
pub(crate) const fn vol_type_to_url(vol_type: VolType) -> LTAResult<&'static str> {
    use crate::models::crowd::passenger_vol;

    let url = match vol_type {
        VolType::BusStops => passenger_vol::URL_BY_BUS_STOPS,
        VolType::OdBusStop => passenger_vol::URL_BY_OD_BUS_STOPS,
        VolType::Train => passenger_vol::URL_BY_TRAIN,
        VolType::OdTrain => passenger_vol::URL_BY_OD_TRAIN,
        _ => return Err(LTAError::UnknownEnumVariant),
    };

    Ok(url)
}
