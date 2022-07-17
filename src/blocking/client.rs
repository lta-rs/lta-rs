//! Client for interacting with LTA API

/// A `Client` to make requests with
/// The `Client` holds a connection pool internally, so it is advised that you create one and reuse it
/// There are some instance where you might need to customise your client due to certain limitations.
///
/// The `Client` trait has a general constructor method and you should use the `reqwest` re-export
/// to build you own customised client from the ground up.
///
#[derive(Debug, Clone)]
pub struct LTAClient<T> {
    pub api_key: String,
    pub client: T,
}