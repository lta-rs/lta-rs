#![allow(async_fn_in_trait)]
#![doc = include_str!("../LIBDOC.md")]

#[cfg(feature = "non-blocking-traits")]
pub use crate::r#async::prelude::*;

#[cfg(feature = "non-blocking-traits")]
pub use crate::r#async::LTAClient;

use http::status::StatusCode;
pub use lta_models as models;
use thiserror::Error;

/// Imports for important structs
pub mod prelude {
    pub use crate::{Bus, Crowd, Facility, Geo, Taxi, Traffic, Train};
}

use crate::models::crowd::passenger_vol::VolType;
use concat_string::concat_string;

#[cfg(any(feature = "reqwest-async", feature = "reqwest-blocking"))]
pub use reqwest;

#[cfg(feature = "ureq-blocking")]
pub use ureq;

/// Internal Async module
#[cfg(feature = "non-blocking-traits")]
pub mod r#async;

/// Internal module containing traits for blocking impl
#[cfg(feature = "blocking-traits")]
pub mod blocking;

#[cfg(feature = "reqwest-blocking")]
pub mod reqwest_blocking;

#[cfg(feature = "ureq-blocking")]
pub mod ureq_blocking;

#[cfg(feature = "reqwest-async")]
pub mod reqwest_async;

/// Type alias for `Result<T, LTAError>`
pub type LTAResult<T> = Result<T, LTAError>;

/// LTAError type, all request using lta-rs returns `Result<T, LTAError>`
#[derive(Error, Debug)]
pub enum LTAError {
    /// Internal error within the client backend, open a PR if this happens
    #[error("Internal error within the client backend, open a PR if this happens!")]
    BackendError(Box<dyn std::error::Error + Send + Sync>),

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

    #[error("HTTP Internal Server Error")]
    InternalServerError,

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
    fn new(
        api_key: impl Into<String>,
        client: Self::InternalClient,
        base_url: impl Into<String>,
    ) -> Self;

    /// This method not assign the `api_key` in struct if the provided key is empty or whitespaces
    /// Instead, assign `None`
    fn with_api_key(api_key: impl Into<String>, base_url: impl Into<String>) -> LTAResult<Self>;

    /// Returns `Self::RB`
    fn req_builder(&self, url: &str) -> Self::RB;

    /// Returns the base URL that is set by user
    fn base_url(&self) -> &str;
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
pub(crate) fn vol_type_to_url(base_url: &str, vol_type: VolType) -> LTAResult<String> {
    let url = match vol_type {
        VolType::BusStops => "/PV/Bus",
        VolType::OdBusStop => "/PV/ODBus",
        VolType::Train => "/PV/Train",
        VolType::OdTrain => "/PV/ODTrain",
        _ => return Err(LTAError::UnknownEnumVariant),
    };

    Ok(concat_string!(base_url, url))
}
