<p align="center">
  <img width="250" height="250" src="./logo.png">
</p>
<p align="center">
  <a href="https://github.com/lta-rs/lta-models/blob/main/LICENSE">
    <img src="https://img.shields.io/github/license/lta-rs/lta-models"/>
  </a>
  <a href="https://docs.rs/lta">
    <img src="https://img.shields.io/badge/docs-docs.rs-blue"/>
  </a>
  <a href="https://lta-rs.github.io/lta-rs/lta/">
    <img src="https://img.shields.io/badge/docs-master--branch-red"/>
  </a>
  <a href="https://github.com/lta-rs/lta-rs/actions">
    <img src="https://img.shields.io/github/workflow/status/lta-rs/lta-rs/Test%20Rust%20project"/>
  </a>
  <a href="https://crates.io/crates/lta">
    <img src="https://img.shields.io/crates/v/lta"/>
  </a>
  <a href="https://github.com/BudiNverse/lta-rs">
    <img src="https://img.shields.io/crates/d/lta"/>
  </a>
</p>

# lta-rs
🚍 Singapore LTA Datamall async first Rust client. lta-rs is used to interact with  [lta-datamall](https://www.mytransport.sg/content/mytransport/home/dataMall.html)

## lta-rs in action

### Cargo.toml setup
```toml
[dependencies]
lta = { version = "0.7.0" }
```

### API key setup
You can get your API key from [here](https://www.mytransport.sg/content/mytransport/home/dataMall/request-for-api.html)

```rust
use lta::{LTAResult, LTAClient, Client, Traffic, TrafficRequests};

#[tokio::main]
async fn main() -> LTAResult<()> {
    let api_key = std::env::var("API_KEY").expect("API_KEY not found!");
    let client = LTAClient::with_api_key(api_key)?;
    let erp_rates = Traffic::get_erp_rates(&client, None).await?;
    println!("{:?}", erp_rates);
    Ok(())
}
```

### Feature flags
| Feature                            | Description                                                                                               |
| ---------------------------------- | --------------------------------------------------------------------------------------------------------- |
| `default` (i.e no features added ) | Uses [`reqwest`](https://github.com/seanmonstar/reqwest) under the hood                                   |
| `reqwest-blocking`                 | Uses [`reqwest::blocking`](https://github.com/seanmonstar/reqwest) under the hood                         |
| `ureq-blocking`                    | Uses [`ureq`](https://github.com/algesten/ureq) under the hood                                            |
| `fastfloat`                        | Enables the [`fastfloat`](https://github.com/aldanor/fast-float-rust) impl for parsing floats (uses SIMD) |
| `non-blocking-traits`              | Exports traits that can be use to impl non-blocking clients                                               |
| `blocking-traits`                  | Exports traits that can be use to impl blocking clients                                                   |

### Feature flags examples
Using `ureq` only
```toml
[dependencies]
lta = { version = "0.7.0", default-features = false, features = ["ureq-blocking"]}
```

Implementing another blocking backend
```toml
[dependencies]
lta = { version = "0.7.0", default-features = false, features = ["blocking-traits"]}
```

Implementing another async backend
```toml
[dependencies]
lta = { version = "0.7.0", default-features = false, features = ["non-blocking-traits"]}
```

### Backend Support 
| Backend          | Status      | Github CI Run :octocat: |
| ---------------- | ----------- | ----------------------- |
| reqwest          | Official ✔  | Yes ✔ ️                  |
| reqwest blocking | Official ✔ ️ | Yes ✔️                   |
| ureq             | Official ✔ ️ | Yes ✔️                   |
| surf             | TBA      ⭕ ️ | No  ⭕                   |

### Examples
| Example                                               | Description                          |
| ----------------------------------------------------- | ------------------------------------ |
| [bus_timing.rs](./examples/bus_timing.rs)             | How to get bus timing (async used)   |
| [reqwest_blocking.rs](./examples/reqwest_blocking.rs) | How to use reqwest blocking feature  |
| [ureq_blocking.rs](./examples/ureq_blocking.rs)       | How to use ureq backend              |
| [custom_client.rs](./examples/custom_client.rs)       | How to create custom backend clients |

### General advice
- Reuse `LTAClient<T>` as it holds a connection pool internally
- Reduce the number of times you call the API, take a look at `Update Freq` in the documentation and prevent
yourself from getting blacklisted. Use a caching mechanism.

### Getting help
- You can get help via GitHub issues. I will try my best to respond to your queries :smile:

### Changelog
> Changelog can be found [here](./CHANGELOG.md)

### Requirements
- Rust compiler 1.56

### Frequently Asked Questions

Q: Is this library being actively developed?

A: Project is currently in maintenance mode. Won't really have any new features. Just bug fixes, minor upgrades etc.

Q: What are the APIs available?

A: All of the APIs are implemented. Take a look at the official LTA docs.

Q: Where do I get the official docs from lta?

A: You can get them [here](https://www.mytransport.sg/content/dam/datamall/datasets/LTA_DataMall_API_User_Guide.pdf)

### License
lta-rs is licensed under MIT license (LICENSE-MIT or <http://opensource.org/licenses/MIT>)

### Donations
> For Singapore based users, you can donate using paylah!

<img src="./paylah.png" width="250">