//! ## async
//! Currently uses `futures-preview = "0.3.0-alpha.17"`
//! API calling convention is exactly the same as the sync version
//!
//! ## Example
//! ```rust
//! use lta::r#async::lta_client::LTAClient;
//! use lta::r#async::{bus::get_arrival, traffic::get_erp_rates};
//! use lta::utils::commons::Client;
//! use tokio::prelude::Future;
//! use lta::traffic::erp_rates::ErpRate;
//! use lta::bus::bus_arrival::BusArrivalResp;
//! use futures::{TryStreamExt, FutureExt};
//!
//! fn async_example(client: &LTAClient) -> impl Future<Item = (), Error = ()> {
//!     type Req = (Vec<ErpRate>, BusArrivalResp);
//!     let fut = get_erp_rates(client);
//!     let fut2 = get_arrival(client, 83139, "15");
//!     fut.join(fut2)
//!         .map(|(a,b): Req| {
//!             println!("{:?}", a);
//!             println!("{:?}", b);
//!     })
//!     .map_err(|e| println!("Request failed ${:?}", e))
//! }
//!
//! fn run() {
//!     use std::env;
//!     
//!     let api_key = env::var("API_KEY").unwrap();
//!     let client = &LTAClient::with_api_key(api_key);
//!     let fut = async_example(client);
//!     tokio::run(fut);
//! }
//! ```
//!
//!

pub use futures;
pub use tokio;

pub mod bus;
pub mod crowd;
pub mod lta_client;
pub mod taxi;
pub mod traffic;
pub mod train;
