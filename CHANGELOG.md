# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2025-11-16

### Added

- **Complete ERC20 functionality**: Forked from [`alloy-erc20`](https://github.com/leruaa/alloy-erc20) 1.0.0 and added write operation support
- **Public `instance` field** on `LazyToken` to enable direct access to underlying ERC20 contract
- **Write operations** available via `instance` field:
  - `transfer(to, amount)` - Transfer tokens to recipient
  - `approve(spender, amount)` - Approve spender allowance
  - `transferFrom(from, to, amount)` - Transfer using allowance
- **All alloy-erc20 features** preserved:
  - `LazyToken` - Token instance with lazy-loaded metadata caching
  - `Token` - Simple struct with address, symbol, decimals
  - `Erc20ProviderExt` - Provider trait extensions for ERC20 operations
  - `TokenStore` trait with `BasicTokenStore` and `LruTokenStore` implementations
  - BigDecimal support for human-readable token amounts
  - Multi-chain token registry
- **Comprehensive documentation**:
  - README with comparison table and examples
  - Migration guide from erc20-rs 0.2.x
  - Inline documentation for all public APIs
  - Working code examples
- **Dual licensing**: Apache-2.0 OR MIT (to respect both erc20-rs and alloy-erc20 licenses)
- **CI/CD**: GitHub Actions workflow for testing, clippy, formatting, docs, and security audits
- **Test suite**: Unit tests for core functionality

### Changed

- Renamed package from `erc20-rs` to `alloy-erc20-full`
- Updated to Alloy 1.0.41 (unified `alloy` crate)
- Improved API clarity: write operations require provider with configured signer/wallet

### Fixed

- Clippy warnings in vendored alloy-erc20 code
- Example code updated for new crate name

### Migration from erc20-rs 0.2.x

See [MIGRATION_FROM_ERC20_RS.md](MIGRATION_FROM_ERC20_RS.md) for detailed migration guide.

**Quick summary:**

- Change dependency from `erc20-rs` to `alloy-erc20-full`
- Rename `Erc20` to `LazyToken`
- Access write operations via `.instance` field
- Configure provider with wallet for transaction signing

### Migration from alloy-erc20 1.0.0

Simply change imports from `alloy_erc20` to `alloy_erc20_full`. All existing code continues to work.

**New capability:** Write operations via `.instance.method()`.

## Attribution

This crate is a fork of [`alloy-erc20`](https://github.com/leruaa/alloy-erc20) version 1.0.0 by @leruaa, extended to support write operations while maintaining full compatibility.

We aim to contribute write operation support back to the upstream `alloy-erc20` project.

[1.0.0]: https://github.com/suchapalaver/erc20-rs/releases/tag/v1.0.0
