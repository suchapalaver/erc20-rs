# ⚠️ DEPRECATED - Use alloy-erc20-full Instead

This crate (`erc20-rs`) has been **superseded**
by [`alloy-erc20-full`](<https://crates.io/crates/alloy-erc20-full>).

## Why the Change?

Research showed that the excellent [`alloy-erc20`](<https://github.com/leruaa/alloy-erc20>)
crate already provides superior read-only ERC20
functionality with caching,
BigDecimal support, and TokenStore. However, it
lacks write operations.

**alloy-erc20-full** is a fork of `alloy-erc20`
that adds write operation support while preserving
all of its excellent features.
