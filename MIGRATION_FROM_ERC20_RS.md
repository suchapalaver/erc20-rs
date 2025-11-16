# Migration Guide: erc20-rs 0.2.x → alloy-erc20-full 1.0

This guide helps you migrate from `erc20-rs` 0.2.x to `alloy-erc20-full` 1.0.

## Executive Summary

**Why migrate?**

`erc20-rs` was a minimal wrapper around Alloy's ERC20 functionality.  Research showed that:

1. `alloy-erc20` provides better read-only operations with caching and helpers
2. `alloy-erc20` lacks write operations (transfer, approve, transferFrom)
3. **`alloy-erc20-full` combines the best of both**: complete functionality with better features

**What's better in alloy-erc20-full?**

- ✅ Metadata caching (name, symbol, decimals) - reduces RPC calls
- ✅ BigDecimal conversion for human-readable balances
- ✅ TokenStore for managing multiple tokens across chains
- ✅ Provider extension pattern for ergonomic API
- ✅ Complete write operation support
- ✅ Active upstream with more features planned

## Quick Comparison

| Feature | erc20-rs 0.2.x | alloy-erc20-full 1.0 |
|---------|----------------|----------------------|
| balance_of() | ✅ | ✅ (with caching) |
| allowance() | ✅ | ✅ |
| decimals() | ✅ | ✅ (cached!) |
| name() | ❌ | ✅ (cached!) |
| symbol() | ❌ | ✅ (cached!) |
| totalSupply() | ❌ | ✅ |
| transfer() | ✅ | ✅ (via .instance) |
| approve() | ✅ | ✅ (via .instance) |
| transferFrom() | ❌ | ✅ (via .instance) |
| BigDecimal support | ❌ | ✅ |
| TokenStore | ❌ | ✅ |
| Provider extensions | ❌ | ✅ |

## Step-by-Step Migration

### 1. Update Dependencies

**Old `Cargo.toml`:**

```toml
[dependencies]
erc20-rs = "0.2"
alloy-contract = "1.0.41"
alloy-primitives = "1.4.1"
alloy-provider = "1.0.41"
# ... other alloy crates
```

**New `Cargo.toml`:**

```toml
[dependencies]
alloy-erc20-full = "1.0"
alloy = { version = "1.0", features = ["full"] }  # Unified alloy crate
```

### 2. Update Imports

**Old:**

```rust
use erc20_rs::Erc20;
use alloy_primitives::{Address, U256};
use alloy_provider::Provider;
```

**New:**

```rust
use alloy_erc20_full::LazyToken;
use alloy::primitives::{Address, U256};
use alloy::providers::Provider;
```

### 3. Update Struct Names

**Old:**

```rust
let erc20 = Erc20::new(token_address, provider);
```

**New:**

```rust
let token = LazyToken::new(token_address, provider);
```

### 4. Migrate Read Operations

#### balanceOf

**Old:**

```rust
let balance: U256 = erc20.balance_of(user_address).await?;
```

**New (identical):**

```rust
let balance: U256 = token.balance_of(user_address).await?;
```

#### allowance

**Old:**

```rust
let allowance: U256 = erc20.allowance(owner, spender).await?;
```

**New (identical):**

```rust
let allowance: U256 = token.allowance(owner, spender).await?;
```

#### decimals

**Old:**

```rust
let decimals: u8 = erc20.decimals().await?;
```

**New (now cached!):**

```rust
let decimals: &u8 = token.decimals().await?;  // Note: returns &u8, not u8
// If you need owned value:
let decimals: u8 = *token.decimals().await?;
```

#### New capabilities

```rust
// These didn't exist in erc20-rs:
let name: &String = token.name().await?;
let symbol: &String = token.symbol().await?;
let supply: U256 = token.total_supply().await?;

// Get balance as human-readable BigDecimal
let balance_decimal = token.get_balance(balance).await?;
println!("Balance: {balance_decimal}");  // e.g., "1234.567890"
```

### 5. Migrate Write Operations

#### transfer

**Old:**

```rust
use alloy_primitives::Address;

let tx_builder = erc20.transfer(from_address, to_address, amount).await?;
// Then you'd call .send() on the builder
```

**New:**

```rust
// Note: erc20-rs had a confusing API that took a "from" parameter but didn't use it.
// alloy-erc20-full is clearer: the signer is in the provider.

let receipt = token
    .instance
    .transfer(to_address, amount)
    .send()
    .await?
    .watch()
    .await?;

println!("Sent transaction: {:?}", receipt.transaction_hash);
```

#### approve

**Old:**

```rust
let tx_builder = erc20.approve(owner_address, spender_address, amount).await?;
```

**New:**

```rust
// Note: erc20-rs took an "owner" parameter, but this was misleading.
// The owner is always the signer in the provider.

let receipt = token
    .instance
    .approve(spender_address, amount)
    .send()
    .await?
    .watch()
    .await?;
```

#### transferFrom (NEW!)

**Old:**

```rust
// Not supported in erc20-rs 0.2.x
```

**New:**

```rust
let receipt = token
    .instance
    .transferFrom(from_address, to_address, amount)
    .send()
    .await?
    .watch()
    .await?;
```

### 6. Provider Setup

The biggest difference: `erc20-rs` had a confusing signing pattern. `alloy-erc20-full` uses standard Alloy patterns.

#### Old Pattern (Read-Only)

```rust
use alloy_provider::ProviderBuilder;

let provider = ProviderBuilder::new()
    .on_http(rpc_url.parse()?);

let erc20 = Erc20::new(token_address, provider);
```

#### New Pattern (Read-Only) - Same

```rust
use alloy::providers::ProviderBuilder;

let provider = ProviderBuilder::new()
    .on_http(rpc_url.parse()?);

let token = LazyToken::new(token_address, provider);
```

#### Old Pattern (With Signing) - Confusing

```rust
// erc20-rs had a confusing API where you'd pass addresses:
erc20.approve(owner_addr, spender_addr, amount).await?;
erc20.transfer(from_addr, to_addr, amount).await?;
// But it didn't actually handle signing!
```

#### New Pattern (With Signing) - Clear

```rust
use alloy::network::EthereumWallet;
use alloy::providers::ProviderBuilder;
use alloy::signers::local::PrivateKeySigner;

// Set up wallet/signer
let signer: PrivateKeySigner = std::env::var("PRIVATE_KEY")?.parse()?;
let wallet = EthereumWallet::from(signer);

// Create provider with wallet
let provider = ProviderBuilder::new()
    .with_recommended_fillers()  // Important for transaction building
    .wallet(wallet)
    .on_http(rpc_url.parse()?);

let token = LazyToken::new(token_address, provider);

// Now write operations work - the wallet signs automatically
token.instance.transfer(to, amount).send().await?;
```

### 7. Error Handling

**Old:**

```rust
use alloy_contract::Error;

match erc20.balance_of(addr).await {
    Ok(balance) => println!("Balance: {balance}"),
    Err(e) => eprintln!("Error: {e}"),
}
```

**New:**

```rust
use alloy::contract::Error;  // Slightly different path

match token.balance_of(addr).await {
    Ok(balance) => println!("Balance: {balance}"),
    Err(e) => eprintln!("Error: {e}"),
}

// Or use the crate's own error type:
use alloy_erc20_full::Error;  // For provider extension methods
```

## Complete Example Migration

### Before (erc20-rs 0.2.x)

```rust
use erc20_rs::Erc20;
use alloy_primitives::{Address, U256, address};
use alloy_provider::ProviderBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rpc_url = "https://eth.llamarpc.com";
    let provider = ProviderBuilder::new()
        .on_http(rpc_url.parse()?);

    let dai_address = address!("6B175474E89094C44Da98b954EedeAC495271d0F");
    let erc20 = Erc20::new(dai_address, provider);

    // Read operations
    let user = address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
    let balance = erc20.balance_of(user).await?;
    let decimals = erc20.decimals().await?;

    println!("Balance: {} (decimals: {})", balance, decimals);

    // Write operation (confusing API)
    let to = address!("0x1234...");
    let amount = U256::from(1000000000000000000u64);
    let _tx = erc20.transfer(user, to, amount).await?;

    Ok(())
}
```

### After (alloy-erc20-full 1.0)

```rust
use alloy_erc20_full::LazyToken;
use alloy::primitives::{Address, U256, address};
use alloy::providers::ProviderBuilder;
use alloy::network::EthereumWallet;
use alloy::signers::local::PrivateKeySigner;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup provider (with wallet for write operations)
    let signer: PrivateKeySigner = std::env::var("PRIVATE_KEY")?.parse()?;
    let wallet = EthereumWallet::from(signer);

    let rpc_url = "https://eth.llamarpc.com";
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_http(rpc_url.parse()?);

    let dai_address = address!("6B175474E89094C44Da98b954EedeAC495271d0F");
    let token = LazyToken::new(dai_address, provider);

    // Read operations (enhanced!)
    let user = address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
    let balance = token.balance_of(user).await?;
    let decimals = *token.decimals().await?;  // Cached!
    let symbol = token.symbol().await?;  // NEW! Cached!

    // Convert to human-readable decimal
    let balance_decimal = token.get_balance(balance).await?;  // NEW!

    println!("{symbol} balance: {balance_decimal} (decimals: {decimals})");

    // Write operation (clear API)
    let to = address!("0x1234...");
    let amount = U256::from(1000000000000000000u64);
    let receipt = token
        .instance
        .transfer(to, amount)
        .send()
        .await?
        .watch()
        .await?;

    println!("Transfer confirmed: {:?}", receipt.transaction_hash);

    Ok(())
}
```

## Advanced Features in alloy-erc20-full

### TokenStore for Managing Multiple Tokens

```rust
use alloy_erc20_full::{BasicTokenStore, Erc20ProviderExt, TokenId};

let mut store = BasicTokenStore::new();

// Fetch and cache DAI
let dai = provider
    .get_token(address!("6B175474E89094C44Da98b954EedeAC495271d0F"), &mut store)
    .await?;

// Later, retrieve from cache by symbol
let dai_cached = store.get(1, TokenId::Symbol("DAI".to_string())).unwrap();
println!("DAI has {} decimals", dai_cached.decimals);
```

### Provider Extension Methods

```rust
use alloy_erc20_full::Erc20ProviderExt;

// Retrieve token info directly from provider
let token_info = provider.retrieve_token(token_address).await?;
println!("Token: {} ({})", token_info.symbol, token_info.decimals);

// Get balance as BigDecimal in one call
let balance = provider.balance_of(token_address, user_address).await?;
println!("Balance: {balance}");
```

## Common Gotchas

### 1. Decimals returns `&u8` not `u8`

```rust
// OLD
let decimals: u8 = erc20.decimals().await?;

// NEW - returns reference (cached value)
let decimals: &u8 = token.decimals().await?;
// Or dereference:
let decimals: u8 = *token.decimals().await?;
```

### 2. Write operations use `.instance` field

```rust
// OLD
erc20.transfer(from, to, amount).await?;

// NEW
token.instance.transfer(to, amount).send().await?;
```

### 3. Provider must have wallet for writes

```rust
// This WON'T work for write operations:
let provider = ProviderBuilder::new().on_http(url);

// This WILL work:
let provider = ProviderBuilder::new()
    .with_recommended_fillers()
    .wallet(wallet)  // ← Required for writes!
    .on_http(url);
```

### 4. Transaction pattern is different

```rust
// OLD - returns PendingTransactionBuilder
let tx = erc20.transfer(from, to, amount).await?;
// Then manually send...

// NEW - fluent API
let receipt = token.instance
    .transfer(to, amount)
    .send().await?       // Send transaction
    .watch().await?;     // Wait for confirmation

// Or just get transaction hash:
let pending = token.instance.transfer(to, amount).send().await?;
println!("TX hash: {:?}", *pending.tx_hash());
```

## Testing Your Migration

After migrating, verify these operations:

```rust
#[tokio::test]
async fn test_read_operations() {
    let provider = ProviderBuilder::new().on_http(rpc_url);
    let token = LazyToken::new(token_address, provider);

    // These should all work:
    let symbol = token.symbol().await.unwrap();
    let decimals = token.decimals().await.unwrap();
    let balance = token.balance_of(user_address).await.unwrap();
    let supply = token.total_supply().await.unwrap();

    assert!(*decimals > 0);
}

#[tokio::test]
async fn test_write_operations() {
    let signer: PrivateKeySigner = test_private_key().parse().unwrap();
    let wallet = EthereumWallet::from(signer);
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_http(test_rpc_url());

    let token = LazyToken::new(token_address, provider);

    // These should work with proper wallet:
    let receipt = token.instance
        .approve(spender, U256::MAX)
        .send().await.unwrap()
        .watch().await.unwrap();

    assert!(receipt.status());
}
```

## Summary Checklist

- [ ] Update `Cargo.toml` dependencies
- [ ] Change imports from `erc20_rs` to `alloy_erc20_full`
- [ ] Rename `Erc20` to `LazyToken`
- [ ] Update read operations (watch for `&u8` from `decimals()`)
- [ ] Update write operations to use `.instance.method()`
- [ ] Setup provider with wallet for write operations
- [ ] Update error handling imports
- [ ] Test all operations
- [ ] Enjoy better features (caching, BigDecimal, TokenStore, etc.)!

## Getting Help

- [alloy-erc20-full README](README.md)
- [alloy-erc20 upstream](https://github.com/leruaa/alloy-erc20)
- [Alloy documentation](https://alloy.rs)
- [Open an issue](https://github.com/suchapalaver/erc20-rs/issues)

## Future Plans

We plan to contribute write operation support back to the upstream `alloy-erc20` project. This will help the entire Rust+Ethereum ecosystem. If/when that happens, we'll provide migration instructions for moving from `alloy-erc20-full` back to `alloy-erc20`.
