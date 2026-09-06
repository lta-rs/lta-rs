<p align="center">
  <img width="350" height="350" src="./logo.png">
</p>

## Development

Secrets are managed with [SecretSpec](https://secretspec.dev/); the manifest in
`secretspec.toml` declares the LTA DataMall `AccountKey` without storing any values.

1. Store your key once (saved to the macOS Keychain via the `keyring` provider):

   ```sh
   secretspec set LTA_ACCOUNT_KEY
   ```

2. Run tests with secrets injected:

   ```sh
   secretspec run -- cargo test
   ```

Without a stored key the live download test
(`bus_stops_downloads_and_decodes_live_fixture`) skips automatically.

### Collecting fixtures

Fixtures under `tests/fixtures/` are captured from the live API with the
`save_fixtures` example. Every run writes **new** numbered files
(`<name>_<N>.json`) next to the existing ones — nothing is ever overwritten,
so captures collect up over time and each numbered file is an identifier for
a particular captured response.

```sh
# list all capture ids
secretspec run -- cargo run --example save_fixtures -- --list

# capture one endpoint
secretspec run -- cargo run --example save_fixtures -- --only taxi_availability

# capture everything (page 0 of each endpoint)
secretspec run -- cargo run --example save_fixtures

# walk $skip pagination to the end and collect every page
secretspec run -- cargo run --example save_fixtures -- --all-pages
```

Requests are paced (500 ms apart by default, `--delay-ms` to change) and
429/5xx responses are retried with backoff. The tool exits non-zero if any
endpoint fails, so it can be wired into CI later.

