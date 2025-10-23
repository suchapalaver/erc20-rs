//! ERC20 token with EIP-3009 transfer with authorization support.
//!
//! This module provides a wrapper for ERC20 tokens that implement EIP-3009,
//! enabling gasless transfers and authorization-based token operations.

use alloy_contract::Error as ContractError;
use alloy_network::Ethereum;
use alloy_primitives::{Address, FixedBytes, Signature, U256};
use alloy_provider::{PendingTransactionBuilder, Provider};
use alloy_sol_types::sol;

sol!(
    #[allow(clippy::too_many_arguments)]
    #[allow(missing_docs)]
    #[sol(rpc)]
    ERC20WithEip3009Contract,
    "abi/erc20_with_eip3009.json"
);

use ERC20WithEip3009Contract::ERC20WithEip3009ContractInstance;

/// ERC20 token with EIP-3009 support.
///
/// This struct provides access to EIP-3009 authorization-based transfer functions,
/// in addition to standard ERC20 functionality.
#[derive(Clone, Debug)]
pub struct Erc20WithEip3009<P: Provider<Ethereum>> {
    instance: ERC20WithEip3009ContractInstance<P>,
}

impl<P: Provider<Ethereum>> Erc20WithEip3009<P> {
    /// Creates a new `Erc20WithEip3009` instance for a token contract.
    ///
    /// # Arguments
    ///
    /// * `address` - The deployed token contract address
    /// * `provider` - The Ethereum provider
    ///
    /// # Example
    ///
    /// ```ignore
    /// use erc20_rs::Erc20WithEip3009;
    /// use alloy_provider::ProviderBuilder;
    ///
    /// let provider = ProviderBuilder::new().on_http(rpc_url);
    /// let token = Erc20WithEip3009::new(usdc_address, provider);
    /// ```
    pub fn new(address: Address, provider: P) -> Self {
        Self {
            instance: ERC20WithEip3009ContractInstance::new(address, provider),
        }
    }

    /// Gets the EIP-712 domain separator for this token.
    ///
    /// This is required for creating EIP-712 typed data signatures.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let domain_separator = token.domain_separator().await?;
    /// ```
    pub async fn domain_separator(&self) -> Result<FixedBytes<32>, ContractError> {
        self.instance.DOMAIN_SEPARATOR().call().await
    }

    /// Gets the type hash for `transferWithAuthorization`.
    pub async fn transfer_with_authorization_typehash(
        &self,
    ) -> Result<FixedBytes<32>, ContractError> {
        self.instance
            .TRANSFER_WITH_AUTHORIZATION_TYPEHASH()
            .call()
            .await
    }

    /// Gets the type hash for `receiveWithAuthorization`.
    pub async fn receive_with_authorization_typehash(
        &self,
    ) -> Result<FixedBytes<32>, ContractError> {
        self.instance
            .RECEIVE_WITH_AUTHORIZATION_TYPEHASH()
            .call()
            .await
    }

    /// Gets the type hash for `cancelAuthorization`.
    pub async fn cancel_authorization_typehash(&self) -> Result<FixedBytes<32>, ContractError> {
        self.instance.CANCEL_AUTHORIZATION_TYPEHASH().call().await
    }

    // ============ View Functions ============

    /// Gets the balance of an account.
    pub async fn balance_of(&self, address: Address) -> Result<U256, ContractError> {
        self.instance.balanceOf(address).call().await
    }

    /// Gets the allowance granted by owner to spender.
    pub async fn allowance(&self, owner: Address, spender: Address) -> Result<U256, ContractError> {
        self.instance.allowance(owner, spender).call().await
    }

    /// Gets the token decimals.
    pub async fn decimals(&self) -> Result<u8, ContractError> {
        self.instance.decimals().call().await
    }

    /// Gets the token name.
    pub async fn name(&self) -> Result<String, ContractError> {
        self.instance.name().call().await
    }

    /// Gets the token symbol.
    pub async fn symbol(&self) -> Result<String, ContractError> {
        self.instance.symbol().call().await
    }

    /// Gets the total supply.
    pub async fn total_supply(&self) -> Result<U256, ContractError> {
        self.instance.totalSupply().call().await
    }

    /// Gets the current nonce for an account (for EIP-2612 permit).
    pub async fn nonces(&self, owner: Address) -> Result<U256, ContractError> {
        self.instance.nonces(owner).call().await
    }

    /// Checks whether an authorization has been used.
    ///
    /// # Arguments
    ///
    /// * `authorizer` - The address that created the authorization
    /// * `nonce` - The nonce of the authorization
    ///
    /// # Returns
    ///
    /// * `true` if the authorization has been used or canceled
    /// * `false` if the authorization is still available
    pub async fn authorization_state(
        &self,
        authorizer: Address,
        nonce: FixedBytes<32>,
    ) -> Result<bool, ContractError> {
        self.instance
            .authorizationState(authorizer, nonce)
            .call()
            .await
    }

    // ============ State-Changing Functions ============

    /// Standard ERC20 approve function.
    pub async fn approve(
        &self,
        owner: Address,
        spender: Address,
        amount: U256,
    ) -> Result<PendingTransactionBuilder<Ethereum>, ContractError> {
        self.instance
            .approve(spender, amount)
            .from(owner)
            .send()
            .await
    }

    /// Standard ERC20 transfer function.
    pub async fn transfer(
        &self,
        from: Address,
        to: Address,
        amount: U256,
    ) -> Result<PendingTransactionBuilder<Ethereum>, ContractError> {
        self.instance.transfer(to, amount).from(from).send().await
    }

    /// Standard ERC20 transferFrom function.
    pub async fn transfer_from(
        &self,
        sender: Address,
        from: Address,
        to: Address,
        amount: U256,
    ) -> Result<PendingTransactionBuilder<Ethereum>, ContractError> {
        self.instance
            .transferFrom(from, to, amount)
            .from(sender)
            .send()
            .await
    }

    // ============ EIP-3009 Functions ============

    /// Executes a transfer with an authorization signature.
    ///
    /// This allows anyone to submit a transfer on behalf of the token holder
    /// using a signed authorization. The submitter pays for gas.
    ///
    /// # Arguments
    ///
    /// * `from` - Token holder authorizing the transfer
    /// * `to` - Recipient of the tokens
    /// * `value` - Amount to transfer
    /// * `valid_after` - Unix timestamp after which the authorization is valid
    /// * `valid_before` - Unix timestamp before which the authorization is valid
    /// * `nonce` - Unique 32-byte nonce for this authorization
    /// * `signature` - ECDSA signature from the `from` address
    ///
    /// # Security Warning
    ///
    /// This function is susceptible to front-running. If the recipient needs protection,
    /// use `receive_with_authorization` instead.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use erc20_rs::signing::{sign_transfer_authorization, generate_nonce};
    /// use erc20_rs::types::TransferAuthorizationParams;
    ///
    /// // Alice creates and signs authorization
    /// let params = TransferAuthorizationParams::with_duration(
    ///     alice_address,
    ///     bob_address,
    ///     U256::from(1_000_000), // 1 USDC (6 decimals)
    ///     3600, // Valid for 1 hour
    ///     generate_nonce(),
    /// );
    /// let signature = sign_transfer_authorization(&params, domain_separator, &alice_signer).await?;
    ///
    /// // Bob (or anyone) submits the transaction
    /// let tx = token.transfer_with_authorization(
    ///     params.from,
    ///     params.to,
    ///     params.value,
    ///     params.valid_after,
    ///     params.valid_before,
    ///     params.nonce,
    ///     signature,
    /// ).await?;
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub async fn transfer_with_authorization(
        &self,
        from: Address,
        to: Address,
        value: U256,
        valid_after: u64,
        valid_before: u64,
        nonce: FixedBytes<32>,
        signature: Signature,
    ) -> Result<PendingTransactionBuilder<Ethereum>, ContractError> {
        // Extract v, r, s from signature
        // Alloy Signature v() returns a bool (Parity), convert to u8 (27 or 28)
        let v = if signature.v() { 28u8 } else { 27u8 };
        let r = signature.r().into();
        let s = signature.s().into();

        self.instance
            .transferWithAuthorization(
                from,
                to,
                value,
                U256::from(valid_after),
                U256::from(valid_before),
                nonce,
                v,
                r,
                s,
            )
            .send()
            .await
    }

    /// Executes a receive with an authorization signature.
    ///
    /// Similar to `transfer_with_authorization`, but provides front-running protection
    /// by requiring that `msg.sender` matches the `to` address. Only the recipient
    /// can submit this transaction.
    ///
    /// # Arguments
    ///
    /// * `from` - Token holder authorizing the transfer
    /// * `to` - Recipient of the tokens (must be `msg.sender`)
    /// * `value` - Amount to transfer
    /// * `valid_after` - Unix timestamp after which the authorization is valid
    /// * `valid_before` - Unix timestamp before which the authorization is valid
    /// * `nonce` - Unique 32-byte nonce for this authorization
    /// * `signature` - ECDSA signature from the `from` address
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Alice creates authorization for Bob to receive tokens
    /// let params = TransferAuthorizationParams::with_duration(
    ///     alice_address,
    ///     bob_address,
    ///     U256::from(1_000_000),
    ///     3600,
    ///     generate_nonce(),
    /// );
    /// let signature = sign_receive_authorization(&params, domain_separator, &alice_signer).await?;
    ///
    /// // Only Bob can submit this transaction (front-run safe)
    /// let tx = token.receive_with_authorization(
    ///     params.from,
    ///     params.to,
    ///     params.value,
    ///     params.valid_after,
    ///     params.valid_before,
    ///     params.nonce,
    ///     signature,
    /// ).await?;
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub async fn receive_with_authorization(
        &self,
        from: Address,
        to: Address,
        value: U256,
        valid_after: u64,
        valid_before: u64,
        nonce: FixedBytes<32>,
        signature: Signature,
    ) -> Result<PendingTransactionBuilder<Ethereum>, ContractError> {
        // Extract v, r, s from signature
        // Alloy Signature v() returns a bool (Parity), convert to u8 (27 or 28)
        let v = if signature.v() { 28u8 } else { 27u8 };
        let r = signature.r().into();
        let s = signature.s().into();

        self.instance
            .receiveWithAuthorization(
                from,
                to,
                value,
                U256::from(valid_after),
                U256::from(valid_before),
                nonce,
                v,
                r,
                s,
            )
            .from(to) // Recipient must be the sender
            .send()
            .await
    }

    /// Cancels an authorization before it has been used.
    ///
    /// This prevents the authorization from being used in the future.
    ///
    /// # Arguments
    ///
    /// * `authorizer` - The address that created the authorization
    /// * `nonce` - The nonce of the authorization to cancel
    /// * `signature` - ECDSA signature from the authorizer
    ///
    /// # Example
    ///
    /// ```ignore
    /// use erc20_rs::signing::{sign_cancel_authorization};
    /// use erc20_rs::types::CancelAuthorizationParams;
    ///
    /// let params = CancelAuthorizationParams::new(alice_address, nonce_to_cancel);
    /// let signature = sign_cancel_authorization(&params, domain_separator, &alice_signer).await?;
    ///
    /// let tx = token.cancel_authorization(
    ///     params.authorizer,
    ///     params.nonce,
    ///     signature,
    /// ).await?;
    /// ```
    pub async fn cancel_authorization(
        &self,
        authorizer: Address,
        nonce: FixedBytes<32>,
        signature: Signature,
    ) -> Result<PendingTransactionBuilder<Ethereum>, ContractError> {
        // Extract v, r, s from signature
        // Alloy Signature v() returns a bool (Parity), convert to u8 (27 or 28)
        let v = if signature.v() { 28u8 } else { 27u8 };
        let r = signature.r().into();
        let s = signature.s().into();

        self.instance
            .cancelAuthorization(authorizer, nonce, v, r, s)
            .send()
            .await
    }
}
