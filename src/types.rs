//! Common types for EIP-3009 transfer with authorization.

use alloy_primitives::{Address, FixedBytes, U256};

/// Parameters for creating a transfer authorization.
///
/// These parameters are used to construct EIP-712 typed data for
/// `transferWithAuthorization` and `receiveWithAuthorization` signatures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransferAuthorizationParams {
    /// The address authorizing the transfer (token holder).
    pub from: Address,
    /// The payee address receiving the tokens.
    pub to: Address,
    /// The amount of tokens to transfer.
    pub value: U256,
    /// Unix timestamp after which the authorization is valid.
    ///
    /// Use 0 for immediate validity.
    pub valid_after: u64,
    /// Unix timestamp before which the authorization is valid.
    ///
    /// This creates a time window for the authorization to be used.
    pub valid_before: u64,
    /// A unique 32-byte nonce to prevent replay attacks.
    ///
    /// Must be random and unique per authorization. Use `generate_nonce()` to create.
    pub nonce: FixedBytes<32>,
}

impl TransferAuthorizationParams {
    /// Creates a new `TransferAuthorizationParams` with specified parameters.
    pub fn new(
        from: Address,
        to: Address,
        value: U256,
        valid_after: u64,
        valid_before: u64,
        nonce: FixedBytes<32>,
    ) -> Self {
        Self {
            from,
            to,
            value,
            valid_after,
            valid_before,
            nonce,
        }
    }

    /// Creates a new authorization with a time window starting now.
    ///
    /// # Arguments
    ///
    /// * `from` - Token holder authorizing the transfer
    /// * `to` - Recipient of the tokens
    /// * `value` - Amount to transfer
    /// * `duration_seconds` - How long the authorization should be valid (e.g., 3600 for 1 hour)
    ///
    /// # Example
    ///
    /// ```ignore
    /// use erc20_rs::types::TransferAuthorizationParams;
    /// use erc20_rs::signing::generate_nonce;
    ///
    /// let params = TransferAuthorizationParams::with_duration(
    ///     alice_address,
    ///     bob_address,
    ///     U256::from(1000000), // 1 USDC (6 decimals)
    ///     3600, // Valid for 1 hour
    ///     generate_nonce(),
    /// );
    /// ```
    pub fn with_duration(
        from: Address,
        to: Address,
        value: U256,
        duration_seconds: u64,
        nonce: FixedBytes<32>,
    ) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        Self {
            from,
            to,
            value,
            valid_after: now,
            valid_before: now + duration_seconds,
            nonce,
        }
    }

    /// Creates a new authorization that is valid immediately and indefinitely.
    ///
    /// # Security Warning
    ///
    /// This creates an authorization with no time bounds (valid_after = 0, valid_before = max).
    /// This is generally **not recommended** for production use as it provides no time-based
    /// protection. Prefer using `with_duration()` instead.
    pub fn without_time_bounds(
        from: Address,
        to: Address,
        value: U256,
        nonce: FixedBytes<32>,
    ) -> Self {
        Self {
            from,
            to,
            value,
            valid_after: 0,
            valid_before: u64::MAX,
            nonce,
        }
    }
}

/// Parameters for canceling an authorization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelAuthorizationParams {
    /// The address that originally authorized the operation.
    pub authorizer: Address,
    /// The nonce of the authorization to cancel.
    pub nonce: FixedBytes<32>,
}

impl CancelAuthorizationParams {
    /// Creates a new `CancelAuthorizationParams`.
    pub fn new(authorizer: Address, nonce: FixedBytes<32>) -> Self {
        Self { authorizer, nonce }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::address;

    #[test]
    fn test_transfer_authorization_params_new() {
        let from = address!("0000000000000000000000000000000000000001");
        let to = address!("0000000000000000000000000000000000000002");
        let value = U256::from(1000);
        let nonce = FixedBytes::<32>::from([1u8; 32]);

        let params = TransferAuthorizationParams::new(from, to, value, 100, 200, nonce);

        assert_eq!(params.from, from);
        assert_eq!(params.to, to);
        assert_eq!(params.value, value);
        assert_eq!(params.valid_after, 100);
        assert_eq!(params.valid_before, 200);
        assert_eq!(params.nonce, nonce);
    }

    #[test]
    fn test_transfer_authorization_params_with_duration() {
        let from = address!("0000000000000000000000000000000000000001");
        let to = address!("0000000000000000000000000000000000000002");
        let value = U256::from(1000);
        let nonce = FixedBytes::<32>::from([1u8; 32]);

        let params = TransferAuthorizationParams::with_duration(from, to, value, 3600, nonce);

        assert_eq!(params.from, from);
        assert_eq!(params.to, to);
        assert_eq!(params.value, value);
        assert!(params.valid_before > params.valid_after);
        assert_eq!(params.valid_before - params.valid_after, 3600);
    }

    #[test]
    fn test_transfer_authorization_params_without_time_bounds() {
        let from = address!("0000000000000000000000000000000000000001");
        let to = address!("0000000000000000000000000000000000000002");
        let value = U256::from(1000);
        let nonce = FixedBytes::<32>::from([1u8; 32]);

        let params = TransferAuthorizationParams::without_time_bounds(from, to, value, nonce);

        assert_eq!(params.from, from);
        assert_eq!(params.to, to);
        assert_eq!(params.value, value);
        assert_eq!(params.valid_after, 0);
        assert_eq!(params.valid_before, u64::MAX);
    }

    #[test]
    fn test_cancel_authorization_params_new() {
        let authorizer = address!("0000000000000000000000000000000000000001");
        let nonce = FixedBytes::<32>::from([1u8; 32]);

        let params = CancelAuthorizationParams::new(authorizer, nonce);

        assert_eq!(params.authorizer, authorizer);
        assert_eq!(params.nonce, nonce);
    }
}
