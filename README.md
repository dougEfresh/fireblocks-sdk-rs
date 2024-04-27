<div align="center">
  <h1><code>fireblocks-sdk</code></h1>
  <a href="https://docs.rs/fireblocks-sdk/">
    <img src="https://docs.rs/fireblocks-sdk/badge.svg">
  </a>
  <a href="https://github.com/dougEfresh/fireblocks-sdk-rs/actions">
    <img src="https://github.com/dougEfresh/fireblocks-sdk-rs/workflows/Continuous%20integration/badge.svg">
  </a>
  <a href="https://deps.rs/repo/github/dougEfresh/fireblocks-sdk-rs">
    <img src="https://deps.rs/repo/github/dougEfresh/fireblocks-sdk-rs/status.svg" >
  </a>
  <a href="https://codecov.io/github/dougEfresh/fireblocks-sdk-rs" > 
   <img src="https://codecov.io/github/dougEfresh/fireblocks-sdk-rs/graph/badge.svg?token=dILa1k9tlW"/> 
 </a>
  <a href="https://crates.io/crates/fireblocks-sdk">
    <img src="https://img.shields.io/crates/v/fireblocks-sdk.svg">
  </a>
</div>


# Overview

`fireblocks_sdk` is an async library for the Fireblocks [API](https://docs.fireblocks.com/api/swagger-ui/#)

!!!! Note this is community driven project and not affiliated with [Fireblocks](https://fireblocks.io) !!!!! 

# Getting Started 

See developer [portal](https://developers.fireblocks.com/docs/introduction) and sign up for a [sandbox](https://developers.fireblocks.com/docs/sandbox-quickstart) account

# Quick Start

```rust
use fireblocks_sdk::{ClientBuilder, PagingVaultRequestBuilder};
use std::time::Duration;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
  let api_key = std::env::var("FIREBLOCKS_API_KEY")?;
  let secret = std::env::var("FIREBLOCKS_SECRET")?;
  let client = ClientBuilder::new(&api_key, &secret.into_bytes())
    .with_timeout(Duration::from_secs(10))
    .with_connect_timeout(Duration::from_secs(5))
    .build()?;
  let params = PagingVaultRequestBuilder::new().limit(10).build()?;
  let (vault_accounts, request_id) = client.vaults(params).await?;
  println!("Got requestId: {request_id}");
  println!("vault accounts: {:#?}", vault_accounts.accounts);
  Ok(())
}
```

# Development

Create a .env file
```shell
cp .env-sameple .env
```
Edit .env and configure your API and secret key

Run tests:
```shell
cargo test
```
---
