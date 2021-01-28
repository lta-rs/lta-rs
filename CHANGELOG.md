### Changelog
Version 0.5.0 **[ Breaking Changes ]**
- Added GeoSpatialWholeIsland API
- Added FacilitiesMaintenance API
- Made library async-first, ie by default, it all requests are marked `async`
- Changed from openssl to rustls
- Both blocking and async APIs are implemented using traits rather than free standing functions
- Moved to Github CI to simplify CI process
- Combined `lta-models` and `lta-utils-commons` crate to a single [lta-models](https://github.com/lta-rs/lta-models) crate
- Performance wise, async APIs are slightly slower (no benchmarks) due to the usage of async-traits crate which boxes `Future`
- Moved projects to the `lta-rs` organisation

Version 0.4.0 **[ Breaking Changes ]**
- Added Taxi Stand API
- Added number of skip for all APIs that requires it

Version 0.3.0 **[ Breaking Changes ]**
- `std::future` for async implementations
- Optional features `async`, `blocking` 
- Changed to Azure Pipelines for windows compilation
- General code clean up

Version 0.3.0-async-preview-5
- Changed internals of deserialisation

Version 0.3.0-async-preview-4 **[ Breaking Changes ]**
- Re-exports to ensure compatibility (chrono)
- APIs that broke `bus::get_bus_arrival`, `traffic::get_bike_parking`

Version 0.3.0-async-preview-3 **[ Breaking Changes ]**
- Removed some re-exports to avoid confusion
- Removed `futures-preview = "0.3.0-alpha.17"`
- Removed `tokio` as dependency and make it dev-dependency
- Added `futures = "0.1.28"`

Version 0.3.0-async-preview-2 **[ Breaking Changes ]**
- Re-exports to ensure compatibility (reqwest)
- Removed `futures-preview = "0.3.0-alpha.17"`
- Examples for all API, with the exception of `async`

Version 0.3.0-async-preview-1 **[ Breaking Changes ]**
- Client trait, now has 2 clients, one with async capabilities
- Currently using `futures-preview = "0.3.0-alpha.17"` and `tokio = "0.1.22"` 

Version 0.2.3
- Hotfix for broken `lta::bus::get_bus_stops` which will panic due to typo in serde rename


Version 0.2.2 **[ Broken get_bus_stops, yanked from crates.io ]**
- Updated `LTAClient::with_api_key` to create a LTAClient

Version 0.2.1
- Updated dependencies to latest version as of `21 July 2019`

Version 0.2 **[ Breaking Changes ]**
- Changed all API to take in `&LTAClient` rather than using a global `LTAClient`

Version 0.1
- All endpoints that are available from lta datamall website
- Configuration using API