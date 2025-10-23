//! Rust SDK for ERC20 tokens with EIP-3009 support.
//!
//! This crate provides type-safe Rust bindings for ERC20 tokens, including
//! support for EIP-3009 transfer with authorization (gasless transfers).
//!
//! # Examples
//!
//! ## Basic ERC20 Usage
//!
//! ```ignore
//! use erc20_rs::Erc20;
//! use alloy_provider::ProviderBuilder;
//!
//! let provider = ProviderBuilder::new().on_http(rpc_url);
//! let token = Erc20::new(token_address, provider);
//!
//! let balance = token.balance_of(user_address).await?;
//! ```
//!
//! ## EIP-3009 Authorization Transfer
//!
//! ```ignore
//! use erc20_rs::{Erc20WithEip3009, signing, types::TransferAuthorizationParams};
//! use alloy_signer_local::PrivateKeySigner;
//!
//! let token = Erc20WithEip3009::new(usdc_address, provider);
//! let domain_separator = token.domain_separator().await?;
//!
//! // Create authorization
//! let params = TransferAuthorizationParams::with_duration(
//!     alice_address,
//!     bob_address,
//!     U256::from(1_000_000), // 1 USDC
//!     3600, // Valid for 1 hour
//!     signing::generate_nonce(),
//! );
//!
//! // Sign authorization
//! let signature = signing::sign_transfer_authorization(&params, domain_separator, &alice_signer).await?;
//!
//! // Submit transaction (can be done by anyone)
//! let tx = token.transfer_with_authorization(
//!     params.from,
//!     params.to,
//!     params.value,
//!     params.valid_after,
//!     params.valid_before,
//!     params.nonce,
//!     signature,
//! ).await?;
//! ```

use alloy_network::Ethereum;
use alloy_primitives::{Address, U256};
use alloy_provider::{PendingTransactionBuilder, Provider};
use alloy_sol_types::sol;

use crate::ERC20::ERC20Instance;

pub mod eip3009;
pub mod signing;
pub mod types;

pub use eip3009::Erc20WithEip3009;

/// The ERC20 contract.
#[derive(Clone, Debug)]
pub struct Erc20<P: Provider<Ethereum>> {
    instance: ERC20Instance<P>,
}

impl<P: Provider<Ethereum>> Erc20<P> {
    pub fn new(address: Address, provider: P) -> Self {
        Self {
            instance: ERC20Instance::new(address, provider),
        }
    }

    pub async fn balance_of(&self, address: Address) -> Result<U256, alloy_contract::Error> {
        self.instance.balanceOf(address).call().await
    }

    pub async fn allowance(
        &self,
        owner: Address,
        spender: Address,
    ) -> Result<U256, alloy_contract::Error> {
        self.instance.allowance(owner, spender).call().await
    }

    pub async fn decimals(&self) -> Result<u8, alloy_contract::Error> {
        self.instance.decimals().call().await
    }

    pub async fn approve(
        &self,
        owner: Address,
        spender: Address,
        amount: U256,
    ) -> Result<PendingTransactionBuilder<Ethereum>, alloy_contract::Error> {
        self.instance
            .approve(spender, amount)
            .from(owner)
            .send()
            .await
    }

    pub async fn transfer(
        &self,
        from: Address,
        to: Address,
        amount: U256,
    ) -> Result<PendingTransactionBuilder<Ethereum>, alloy_contract::Error> {
        self.instance.transfer(to, amount).from(from).send().await
    }
}

sol!(
    #[allow(clippy::too_many_arguments)]
    #[allow(missing_docs)]
    #[sol(rpc)]
    ERC20,
    "abi/erc20.json"
);
