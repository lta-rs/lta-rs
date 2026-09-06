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
