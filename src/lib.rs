//! <p align="center">
//!     <img width="333" height="117" src="https://raw.githubusercontent.com/BudiNverse/lta-rs/master/logo.png">
//! </p>
//! <p align="center">
//!     <a href="https://github.com/BudiNverse/lta-rs">
//!         <img src="https://img.shields.io/badge/-lta--rs-blueviolet.svg"/>
//!     </a>
//!     <a href="https://github.com/BudiNverse/lta-rs">
//!         <img src="https://img.shields.io/github/license/BudiNverse/lta-rs"/>
//!     </a>
//!     <a href="https://crates.io/crates/lta">
//!         <img src="https://img.shields.io/crates/v/lta"/>
//!     </a>
//!     <a href="https://dev.azure.com/budisyahiddin/lta-rs/_build?definitionId=6">
//!         <img src="https://dev.azure.com/budisyahiddin/lta-rs/_apis/build/status/BudiNverse.lta-rs?branchName=master&jobName=Job&configuration=Job%20stable"/>
//!     </a>
//!     <a href="https://github.com/BudiNverse/lta-rs">
//!         <img src="https://img.shields.io/badge/rust-1.3.9-blueviolet.svg"/>
//!     </a>
//!     <a href="https://github.com/BudiNverse/lta-rs">
//!         <img src="https://img.shields.io/crates/d/lta"/>
//!     </a>
//! </p>
//!
//!
//! # lta
//! lta-rs is a lta datamall client library written in pure safe rust. lta-rs is used to interact with the lta-datamall
//!
//! ## Design Decisions
//! - Made sure that Rust structs are as close to the original response as possible to make sure that people can reference the original docs if there are any issues
//! - Simple and no additional baggage. Only the client is included. E.g If anyone wants to add concurrency, they have to do it on their own
//! - Predictable API usage
//!
//! ## Usage
//! Put this in you `Cargo.toml`
//! Features available: `async`, `blocking`. If you only need blocking requests, choose blocking vice versa.
//! ```toml
//! [dependencies]
//! lta = { version = "0.4.0", features = ["async"] }
//! ```
//!
//! Initialise API key
//! ```rust
//! use lta::{LTAResult, LTAClient, Client, Traffic, TrafficRequests};
//!
//! #[tokio::main]
//! async fn main() -> LTAResult<()> {
//! let api_key = std::env::var("API_KEY").unwrap();
//!     let client = LTAClient::with_api_key(api_key)?;
//!     let erp_rates = Traffic::get_erp_rates(&client, None).await?;
//!     println!("{:?}", erp_rates);
//!     Ok(())
//! }
//! ```

#[macro_export]
macro_rules! api_url {
    ($e: expr) => {
        concat!("http://datamall2.mytransport.sg/ltaodataservice", $e)
    };
}

pub use crate::r#async::prelude::*;
pub use crate::r#async::LTAClient;
pub use lta_models as models;

pub mod prelude {
    pub use crate::{Bus, Crowd, Taxi, Traffic, Train};
}

pub use reqwest;
pub mod r#async;

#[cfg(feature = "blocking")]
pub mod blocking;

pub type LTAResult<T> = Result<T, LTAError>;

#[derive(Debug)]
pub enum LTAError {
    BackendError(reqwest::Error),
    InvalidAPIKey,
    Custom(String),
    RateLimitReached,
    UnknownEnumVariant,
}

/// A `Client` to make requests with
/// The `Client` holds a connection pool internally, so it is advised that you create one and reuse it
pub trait Client: Sized {
    /// Any backend Client
    type InternalClient;

    /// Any type that can build requests
    type RB;

    /// General constructor for `Self`
    fn new<S: Into<String>>(api_key: S, client: Self::InternalClient) -> Self;

    /// This method not assign the `api_key` in struct if the provided key is empty or whitespaces
    /// Instead, assign `None`
    fn with_api_key<S: Into<String>>(api_key: S) -> LTAResult<Self>;

    /// Returns `Self::RB`
    fn req_builder(&self, url: &str) -> Self::RB;
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Bus;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Crowd;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Taxi;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Traffic;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Train;
