<div align="center">
  <h1><code>fireblocks-sdk</code></h1>
  <a href="https://docs.rs/fireblocks-sdk/">
    <img src="https://docs.rs/fireblocks-sdk/badge.svg" height="25">
  </a>
  <a href="https://github.com/dougEfresh/fireblocks-sdk-rs/actions">
    <img src="https://github.com/dougEfresh/fireblocks-sdk-rs/workflows/Continuous%20integration/badge.svg" height="25">
  </a>
  <a href="https://deps.rs/repo/github/dougEfresh/fireblocks-sdk-rs">
    <img src="https://deps.rs/repo/github/dougEfresh/fireblocks-sdk-rs/status.svg" height="25">
  </a>
  <a href="https://codecov.io/github/dougEfresh/fireblocks-sdk-rs" > 
   <img src="https://codecov.io/github/dougEfresh/fireblocks-sdk-rs/graph/badge.svg?token=dILa1k9tlW" height="25"/> 
 </a>
  <a href="https://crates.io/crates/fireblocks-sdk">
    <img src="https://img.shields.io/crates/v/fireblocks-sdk.svg" height="25">
  </a>
</div>


# Overview

`fireblocks_sdk` is an async library for the Fireblocks [API](https://docs.fireblocks.com/api/swagger-ui/#)

!!!! Note this is community driven project and not affiliated with [Fireblocks](https://fireblocks.io) !!!!!

# Getting Started

```shell
cargo install fireblocks-sdk
```

See developer [portal](https://developers.fireblocks.com/docs/introduction) and sign up for a [sandbox](https://developers.fireblocks.com/docs/sandbox-quickstart) account

# Quick Start

```rust
use fireblocks_sdk::ClientBuilder;
use fireblocks_sdk::apis::vaults_api::GetPagedVaultAccountsParams;
use crate::fireblocks_sdk::apis::Api;
use std::time::Duration;

async fn vaults() -> anyhow::Result<()> {
  let api_key = std::env::var("FIREBLOCKS_API_KEY")?;
  let secret = std::env::var("FIREBLOCKS_SECRET")?;
  let client = ClientBuilder::new(&api_key, &secret.into_bytes())
    .with_sandbox()
    .with_timeout(Duration::from_secs(10))
    .with_connect_timeout(Duration::from_secs(5))
    .build()?;
  // Auto generate ApiClient 
  let api_client = client.apis();
  let params = GetPagedVaultAccountsParams::builder()
            .limit(50.0)
            .build();
  let vault_accounts = api_client.vaults_api().get_paged_vault_accounts(params).await?;
  println!("vault accounts: {:#?}", vault_accounts);
  Ok(())
}
```

# APIs

The [client](./src/client.rs) is a small wrapper to the auto-generate [APIs](./src/apis/mod.rs) using openapi generator.

```rust

use crate::fireblocks_sdk::apis::Api;
use fireblocks_sdk::Client;

fn demo(client: Client) {
  // Access to generated API client
  let api_client = client.apis();
  // External Wallet Api (whitlisted)
  let external_wallet_api = api_client.whitelisted_external_wallets_api();
}
```

## Bon builder

This is [bon](https://crates.io/crates/bon) crate for construction parameters to API endpoints (both query and body)

## Caveats 

The openapi-generator decided that all integers are floats. Annoying yes, but it is what it is.

# Development

Create a .env file

```shell
cp .env-sameple .env
```

Edit .env and configure your API and secret key. You also need to create some whitlisted (see [test](./tests/wallets.rs) for details)

Run tests:
```shell
cargo test
```

Run a single test:
```shell
cargo test --test wallets
```

---

# Docs 

Code was generatered by Fireblocks openapi [spec](https://raw.githubusercontent.com/fireblocks/fireblocks-openapi-spec/refs/heads/main/api-spec-v2.yaml) using [openapi-generator](./Makefile) with this [config](./generator/config.yaml)

See [docs](./docs/README.md)
