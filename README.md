<p align="center">
  <img width="666" height="234" src="./logo.png">
</p>
<p align="center">
  <a href="https://github.com/BudiNverse/lta-rs">
    <img src="https://img.shields.io/badge/-lta--rs-blueviolet.svg?style=flat-square"/>
  </a>
  <a href="https://github.com/BudiNverse/lta-rs">
    <img src="https://img.shields.io/github/license/BudiNverse/lta-rs?style=flat-square"/>
  </a>
  <a href="https://crates.io/crates/lta">
    <img src="https://img.shields.io/crates/v/lta?style=flat-square"/>
  </a>
  <a href="https://travis-ci.com/BudiNverse/lta-rs">
    <img src="https://img.shields.io/travis/com/BudiNverse/lta-rs?style=flat-square"/>
  </a>
  <a href="https://github.com/BudiNverse/lta-rs">
    <img src="https://img.shields.io/badge/rust-1.3.6-blueviolet.svg?style=flat-square"/>
  </a>
  <a href="https://github.com/BudiNverse/lta-rs">
    <img src="https://img.shields.io/crates/d/lta?style=flat-square"/>
  </a>
</p>

# lta-rs
> 🚍 Singapore LTA Datamall Rust Client written in pure rust with support for asynchronous requests. lta-rs is used to interact with the [lta-datamall](https://www.mytransport.sg/content/mytransport/home/dataMall.html)

## lta-rs in action

### Cargo.toml setup
There are various versions available. If you omit `branch = "version_no"`, you are taking it from master branch
The library is also available on crates.io
```toml
[dependencies]
lta = "0.3.0-async-preview-3"
```

### API key setup
You can get your API key from [here](https://www.mytransport.sg/content/mytransport/home/dataMall/request-for-api.html)

```rust
extern crate lta;

use lta::lta_client::*;

fn main() {
    let api_key = "MY_API_KEY";
    let client = LTAClient::with_api_key(api_key);
}
```

### Examples

Getting bus timings
```rust
use lta::prelude::*;
use lta::lta_client::LTAClient;
use lta::bus::get_arrival;
use lta::Result;

fn get_bus_arrival() -> Result<()> {
    let api_key = std::env::var("API_KEY").unwrap();
    let client = LTAClient::with_api_key(api_key);
    let arrivals: BusArrivalResp = get_arrival(&client, 83139, "15")?;
    println!("{:?}", arrivals);
    Ok(())
}
```

Getting anything else
```rust
// All the APIs in this library are designed to be used like this
// `module::get_something`
// All of them return lta::utils::Result<Vec<T>>
// The example below is bus::get_bus_services()
// and traffic::get_erp_rates()
// Do note that the API calling convention is similar across all the APIs except for
// bus::get_arrival
// prefer lta::prelude::* over glob imports
use lta::prelude::*;
use lta::lta_client::LTAClient;
use lta::traffic::get_erp_rates;
use lta::bus::get_bus_services;
use lta::Result;

fn bus_services() -> Result<()> {
    let api_key = std::env::var("API_KEY").unwrap();
    let client = LTAClient::with_api_key(api_key);
    let bus_services: Vec<BusService> = get_bus_services(&client)?;
    println!("{:?}", bus_services);
    Ok(())
}

fn get_erp() -> Result<()> {
    let api_key = std::env::var("API_KEY").unwrap();
    let client = LTAClient::with_api_key(api_key);
    let erp_rates: Vec<ErpRate> = get_erp_rates(&client)?;
    println!("{:?}", erp_rates);
    Ok(())
}
```

### Async Example
```rust
use lta::r#async::{
    prelude::*,
    lta_client::LTAClient,
    bus::get_arrival,
    traffic::get_erp_rates
};
use std::env::var;
use tokio::run;

fn async_example(client: &LTAClient) -> impl Future<Item = (), Error = ()> {
    type Req = (Vec<ErpRate>, BusArrivalResp);
    let fut = get_erp_rates(client);
    let fut2 = get_arrival(client, 83139, "15");
    fut.join(fut2)
        .map(|(a,b): Req| {
            println!("{:?}", a);
            println!("{:?}", b);
    })
    .map_err(|e| println!("Request failed ${:?}", e))
}

fn multiple_async_requests() {
    let api_key = var("API_KEY").unwrap();
    let client = &LTAClient::with_api_key(api_key);
    let fut = async_example(client);
    run(fut);
}
```

### Custom Client
There are some instance where you might need to customise the client more due to certain limitations.
```rust
use std::time::Duration;
use lta::reqwest::ClientBuilder;
use lta::lta_client::LTAClient;
use lta::utils::commons::Client;

fn my_custom_client() -> LTAClient {
    let client = ClientBuilder::new()
        .gzip(true)
        .connect_timeout(Some(Duration::new(420,0)))
        .build()
        .unwrap();

    LTAClient::new(Some("api_key".to_string()), client)     
}
 ```

### Concurrent requests without `Futures`
```rust
use std::sync::Arc;
use std::thread::spawn;
use lta::lta_client::LTAClient;
use lta::utils::commons::Client;

fn concurrent() {
    let api_key = env::var("API_KEY").unwrap();
    let c1 = Arc::new(LTAClient::with_api_key(api_key));
    let c2 = c1.clone();

    let child = spawn(move || {
        let res = get_carpark_avail(&c1).unwrap();
        println!("{:?}", res)
    });

    let vms = traffic::get_vms_emas(&c2).unwrap();
    println!("{:?}", vms);

    child.join();
}
```

### General advice
- Reuse `LTAClient` as it holds a connection pool internally
- Reduce the number of times you call the API, take a look at the `Update Freq` in the documentation andprevent
yourself from getting blacklisted. Use a caching mechanism. 
- Prefer `async` APIs over writing your own implementation for concurrent requests.

### Getting help
- You can get help via github issues. I will try my best to respond to your queries :smile:

### Design decisions
- Made sure that Rust structs are as close to the original response as possible to make sure that people can reference the original docs if there are any issues 
- Simple and no additional baggage. Only the client is included. E.g If anyone wants to add concurrency, they have to do it on their own
- Predictable API usage

### Changelog
Version 0.1
- All endpoints that are available from lta datamall website
- Configuration using API

Version 0.2 **[ Breaking Changes ]**
- Changed all API to take in `&LTAClient` rather than using a global `LTAClient`

Version 0.2.1
- Updated dependencies to latest version as of `21 July 2019`

Version 0.2.2 **[ Broken get_bus_stops, yanked from crates.io ]**
- Updated `LTAClient::with_api_key` to create a LTAClient

Version 0.2.3
- Hotfix for broken `lta::bus::get_bus_stops` which will panic due to typo in serde rename

Version 0.3.0-async-preview-1 **[ Breaking Changes ]**
- Client trait, now has 2 clients, one with async capabilities
- Currently using `futures-preview = "0.3.0-alpha.17"` and `tokio = "0.1.22"` 

Version 0.3.0-async-preview-2 **[ Breaking Changes ]**
- Re-exports to ensure computability
- Removed `futures-preview = "0.3.0-alpha.17"`
- Examples for all API, with the exception of `async`

Version 0.3.0-async-preview-3 **[ Breaking Changes ]**
- Removed some re-exports to avoid confusion
- Removed `futures-preview = "0.3.0-alpha.17"`
- Removed `tokio` as dependency and make it dev-dependency
- Added `futures = "0.1.28"`

### Todo (excluding bugs from issues)
- [x] Proper date types using chrono library
- [x] Utils cleanup
- [x] Host on crates.io
- [ ] Static website to showcase project
- [x] Documentation
- [x] More idiomatic Rust code
- [x] Asynchronous requests 
- [x] Travis CI
- [x] Documentation for async
- [ ] `std::future`
- [x] Customisable `Client`
- [ ] Better testing, reduce API spam and cache data for testing
- [ ] Deserialization benchmark

### License
lta-rs is licensed under MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

### Frequently Asked Questions

> Is this library being actively developed?

Yes. However, development will slow down from mid August 2019 onwards due to my NS commitments.

> What are the APIs available?

Take a look at the offical LTA docs.

> Where do I get the official docs from lta?

You can get them [here](https://www.mytransport.sg/content/dam/datamall/datasets/LTA_DataMall_API_User_Guide.pdf)

> Why are some of the data types different from the lta documentation?

Some of the data types returned are not ideal such as returning `lat` and `lang` as `string` rather than `number`. Some of the types are also converted to enums to reduce the number of stringly typed stuff

> My application panicked.

Check if your API key is valid, if it is and your application still panics because of this library, create a github issue

> Why is the most fully featured LTA client library implemented in a language not many people use?

Friendship ended with Kotlin. Now Rust is my best friend ❤️.

> Is this project affiliated to LTA or any government body?

No.

> What is the plan to move to `std::future`?

Currently waiting for dependencies to move to `std::future`. However, this might take some time and different libraries might 
update at different times, so I am currently experimenting on making the APIs exposed to users use `std::future` while the internal implementation
depends on the `compat` layer provided by `futures-preview`.

All the async stuff is currently on preview and will be released for `0.3.0`. I do not want to rush the implementation of async APIs to 
ensure that the ergonomics of them are user friendly. Considering that a lot of libraries are currently moving to `std::future`,
this can be very confusing to beginners that want to take a look into futures.  

Development of this happen on `master` branch. 

### Common Technical Questions
- [EOF while parsing a value](https://github.com/BudiNverse/lta-rs/issues/1)
- [API key not init!](https://github.com/BudiNverse/lta-rs/issues/2)
- [No such known host](https://github.com/BudiNverse/lta-rs/issues/3)