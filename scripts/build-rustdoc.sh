#!/usr/bin/env bash
set -euo pipefail

readonly rust_version="1.95.0"

# Cloudflare Pages does not include Rust in its build image.
curl --proto '=https' --tlsv1.2 --fail --silent --show-error \
  https://sh.rustup.rs \
  | sh -s -- -y --no-modify-path --profile minimal --default-toolchain "$rust_version"

export PATH="${CARGO_HOME:-$HOME/.cargo}/bin:$PATH"
export CARGO_INCREMENTAL=0

cargo +"$rust_version" doc --all-features --no-deps

printf '%s\n' \
  '<!doctype html>' \
  '<meta charset="utf-8">' \
  '<meta http-equiv="refresh" content="0; url=lta/index.html">' \
  '<link rel="canonical" href="lta/index.html">' \
  '<title>lta documentation</title>' \
  > target/doc/index.html
