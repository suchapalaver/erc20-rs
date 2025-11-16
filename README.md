# alloy-erc20-full

Complete ERC20 SDK for [Alloy](https://github.com/alloy-rs/alloy) with both **read and write operations**.

Forked from [`alloy-erc20`](https://github.com/leruaa/alloy-erc20) to add write operation support (transfer, approve, transferFrom).

[![Crates.io](https://img.shields.io/crates/v/alloy-erc20-full.svg)](https://crates.io/crates/alloy-erc20-full)
[![Docs.rs](https://docs.rs/alloy-erc20-full/badge.svg)](https://docs.rs/alloy-erc20-full)
[![Rust Version](https://img.shields.io/badge/rust-2021-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/License-Apache%202.0%20OR%20MIT-blue.svg)](https://opensource.org/licenses/Apache-2.0)

## ⚠️ DEPRECATED - Use alloy-erc20-full Instead

This crate (`erc20-rs`) has been **superseded**
by [`alloy-erc20-full`](<https://crates.io/crates/alloy-erc20-full>).

### Why the Change?

Research showed that the excellent [`alloy-erc20`](<https://github.com/leruaa/alloy-erc20>)
crate already provides superior read-only ERC20
functionality with caching,
BigDecimal support, and TokenStore. However, it
lacks write operations.

**alloy-erc20-full** is a fork of `alloy-erc20`
that adds write operation
support while preserving all of its excellent
features.

### Migration

See the [Migration Guide](<https://github.com/suchapalaver/erc20-rs/blob/main/MIGRATION_FROM_ERC20_RS.md>).

Links:

- New Crate: <https://crates.io/crates/alloy-erc20-full>
- Repository: <https://github.com/suchapalaver/erc20-rs> (same repo, different crate)
- Migration Guide: <https://github.com/suchapalaver/erc20-rs/blob/main/MIGRATION_FROM_ERC20_RS.md>

## Why alloy-erc20-full?

| Feature | Raw Alloy `sol!` | `alloy-erc20` | `alloy-erc20-full` |
|---------|------------------|---------------|-------------------|
| **Read Operations** | ✅ Manual | ✅ Easy helpers | ✅ Easy helpers + caching |
| **Write Operations** | ✅ Manual | ❌ Not supported | ✅ Supported |
| **Metadata Caching** | ❌ | ✅ LazyToken | ✅ LazyToken |
| **TokenStore** | ❌ | ✅ | ✅ |
| **BigDecimal Support** | ❌ | ✅ | ✅ |
| **Provider Extensions** | ❌ | ✅ | ✅ |

**Use this crate if you need:**

- Complete ERC20 functionality (read + write)
- Convenient helpers with caching
- BigDecimal balance conversion
- Token metadata management

**Use `alloy-erc20` if you only need:**

- Read-only operations
- Token metadata queries

**Use raw Alloy if you want:**

- Maximum control
- Minimal dependencies

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
alloy-erc20-full = "1.0"
alloy = { version = "1.0", features = ["full"] }
```

## Quick Start

### Read Operations (Query Token Data)

```rust
use alloy::primitives::{address, U256};
use alloy::providers::ProviderBuilder;
use alloy_erc20_full::LazyToken;

#[tokio::main]
async fn main() {
    let rpc_url = "https://eth.llamarpc.com";
    let provider = ProviderBuilder::new()
        .on_http(rpc_url.parse().unwrap());

    // Create token instance (DAI)
    let dai = LazyToken::new(
        address!("6B175474E89094C44Da98b954EedeAC495271d0F"),
        provider,
    );

    // Read operations - metadata is cached after first call
    let symbol = dai.symbol().await.unwrap();
    let decimals = dai.decimals().await.unwrap();
    let total_supply = dai.total_supply().await.unwrap();

    // Get balance for an address
    let balance = dai.balance_of(
        address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045")
    ).await.unwrap();

    // Convert to BigDecimal (human-readable)
    let balance_decimal = dai.get_balance(balance).await.unwrap();

    println!("{symbol} balance: {balance_decimal}");
}
```

### Write Operations (Transfer, Approve)

**Important:** Provider must be configured with a signer/wallet for write operations.

```rust
use alloy::network::EthereumWallet;
use alloy::primitives::{address, U256};
use alloy::providers::ProviderBuilder;
use alloy::signers::local::PrivateKeySigner;
use alloy_erc20_full::LazyToken;

#[tokio::main]
async fn main() {
    // Setup provider with wallet
    let signer: PrivateKeySigner = "your-private-key".parse().unwrap();
    let wallet = EthereumWallet::from(signer);

    let rpc_url = "https://eth.llamarpc.com";
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_http(rpc_url.parse().unwrap());

    let dai = LazyToken::new(
        address!("6B175474E89094C44Da98b954EedeAC495271d0F"),
        provider,
    );

    // Transfer tokens
    let receipt = dai
        .instance
        .transfer(
            address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045"), // to
            U256::from(1000000000000000000u64), // 1.0 DAI
        )
        .send()
        .await
        .unwrap()
        .watch()
        .await
        .unwrap();

    println!("Transfer complete: {:?}", receipt.transaction_hash);

    // Approve spender
    let receipt = dai
        .instance
        .approve(
            address!("1111111254EEB25477B68fb85Ed929f73A960582"), // 1inch router
            U256::MAX, // unlimited approval
        )
        .send()
        .await
        .unwrap()
        .watch()
        .await
        .unwrap();

    println!("Approval complete: {:?}", receipt.transaction_hash);

    // Transfer from (using allowance)
    let receipt = dai
        .instance
        .transferFrom(
            address!("владелец"),
            address!("получатель"),
            U256::from(1000000000000000000u64),
        )
        .send()
        .await
        .unwrap()
        .watch()
        .await
        .unwrap();
}
```

## Features

### LazyToken - Cached Token Instance

```rust
use alloy_erc20_full::LazyToken;

// Metadata (name, symbol, decimals) is cached after first query
let token = LazyToken::new(address, provider);

// First call queries the network
let symbol = token.symbol().await?; // Network call

// Subsequent calls use cache
let symbol_again = token.symbol().await?; // From cache!

// Always queries network (not cached)
let balance = token.balance_of(user).await?;
let supply = token.total_supply().await?;
```

### Provider Extensions

```rust
use alloy_erc20_full::Erc20ProviderExt;

// Any Alloy provider automatically gets ERC20 methods
let token_info = provider.retrieve_token(token_address).await?;
println!("{} ({})", token_info.symbol, token_info.decimals);

// Get balance as BigDecimal
let balance = provider.balance_of(token_address, user_address).await?;
```

### TokenStore - Multi-Chain Token Registry

```rust
use alloy_erc20_full::{BasicTokenStore, Erc20ProviderExt, TokenId};

let mut store = BasicTokenStore::new();

// Retrieve and cache token
let dai = provider.get_token(
    address!("6B175474E89094C44Da98b954EedeAC495271d0F"),
    &mut store,
).await?;

// Later, retrieve from store by symbol or address
let dai_from_store = store.get(1, TokenId::Symbol("DAI".to_string()));
```

## Architecture

This crate is a fork of `alloy-erc20` with the following additions:

1. **Public `instance` field** on `LazyToken` - Provides direct access to the underlying Alloy contract for write operations
2. **Complete ERC20 support** - All standard ERC20 methods (read + write)
3. **Maintained compatibility** - All original `alloy-erc20` features work identically

### Core Types

- **`LazyToken<P, N>`** - Token instance with lazy-loaded metadata caching
- **`Token`** - Simple struct with address, symbol, decimals
- **`Erc20ProviderExt`** - Trait extending Alloy providers with ERC20 helpers
- **`TokenStore`** - Trait for caching tokens (with `BasicTokenStore` and `LruTokenStore` impls)

## Migration Guides

### From `erc20-rs` 0.2.x

See [MIGRATION_FROM_ERC20_RS.md](MIGRATION_FROM_ERC20_RS.md) for detailed guide.

**Quick comparison:**

```rust
// OLD: erc20-rs 0.2.x
use erc20_rs::Erc20;
let erc20 = Erc20::new(address, provider);
let balance = erc20.balance_of(user).await?;
let tx = erc20.transfer(from, to, amount).await?;

// NEW: alloy-erc20-full 1.0
use alloy_erc20_full::LazyToken;
let token = LazyToken::new(address, provider);
let balance = token.balance_of(user).await?;
let receipt = token.instance.transfer(to, amount).send().await?.watch().await?;
```

### From `alloy-erc20`

`alloy-erc20-full` is a superset of `alloy-erc20`. All code using `alloy-erc20` works identically:

```rust
// Just change the crate name in Cargo.toml
// alloy-erc20 = "1.0"  ← old
alloy-erc20-full = "1.0"  // ← new

// And update imports
// use alloy_erc20::LazyToken;  ← old
use alloy_erc20_full::LazyToken;  // ← new

// All existing code works as-is!
```

**New capability:** Write operations via `.instance` field.

## Examples

See [`examples/`](examples/) directory:

- `lazy.rs` - LazyToken with caching
- `provider_ext.rs` - Provider extension methods
- `basic_store.rs` - TokenStore usage

Run with:

```bash
ETH_MAINNET_RPC=https://eth.llamarpc.com cargo run --example lazy
```

## Features

### Default

- Basic functionality with `BasicTokenStore`

### Optional

- `lru-store` - Adds `LruTokenStore` with LRU eviction policy
- `known-tokens` - Pre-populated token lists for mainnet and Arbitrum

```toml
[dependencies]
alloy-erc20-full = { version = "1.0", features = ["lru-store", "known-tokens"] }
```

## Testing

```bash
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

## Minimum Supported Rust Version (MSRV)

Rust 1.75+

## License

Licensed under Apache-2.0 OR MIT (to respect both original `erc20-rs` and `alloy-erc20` licenses).

## Attribution

This crate is a fork of [`alloy-erc20`](https://github.com/leruaa/alloy-erc20) by @leruaa, extended to support write operations.

## Contributing

Contributions welcome! This crate aims to eventually contribute write operation support back to the upstream `alloy-erc20` project.

## Changelog

### 1.0.0 (2025-11)

- Forked from `alloy-erc20` 1.0.0
- Added write operation support by exposing contract `instance` field
- Complete ERC20 functionality (read + write)
- Maintained full compatibility with `alloy-erc20` API
