//! # DEPRECATED
//!
//! This crate has been superseded by [`alloy-erc20-full`](https://crates.io/crates/alloy-erc20-full).
//!
//! Please migrate to `alloy-erc20-full` for:
//! - All features of the original alloy-erc20
//! - Write operation support (transfer, approve, transferFrom)
//! - Better documentation and examples
//!
//! See the [migration guide](https://github.com/suchapalaver/erc20-rs/blob/main/MIGRATION_FROM_ERC20_RS.md).

#![deprecated(
    since = "0.3.0",
    note = "Use alloy-erc20-full instead:
    https://crates.io/crates/alloy-erc20-full"
)]

// Re-export alloy-erc20-full for compatibility
pub use alloy_erc20_full::*;
