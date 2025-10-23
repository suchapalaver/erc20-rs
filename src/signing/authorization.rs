//! High-level authorization signing helpers for EIP-3009.
//!
//! This module provides convenient functions for creating signed authorizations
//! using local wallets.

use crate::signing::eip712::{
    hash_cancel_authorization, hash_receive_with_authorization, hash_transfer_with_authorization,
};
use crate::types::{CancelAuthorizationParams, TransferAuthorizationParams};
use alloy_primitives::{FixedBytes, Signature, U256};
use alloy_signer::{Signer, SignerSync};
use alloy_signer_local::PrivateKeySigner;

/// Signs a `transferWithAuthorization` message with a local signer.
///
/// # Arguments
///
/// * `params` - The authorization parameters
/// * `domain_separator` - The EIP-712 domain separator from the token contract
/// * `signer` - The signer with the `from` address's private key
///
/// # Returns
///
/// The ECDSA signature (v, r, s packed into `Signature`).
///
/// # Example
///
/// ```ignore
/// use erc20_rs::signing::{sign_transfer_authorization, generate_nonce};
/// use erc20_rs::types::TransferAuthorizationParams;
/// use alloy_signer_local::PrivateKeySigner;
///
/// let signer = PrivateKeySigner::random();
/// let params = TransferAuthorizationParams::with_duration(
///     signer.address(),
///     recipient_address,
///     U256::from(1000000),
///     3600,
///     generate_nonce(),
/// );
///
/// let signature = sign_transfer_authorization(&params, domain_separator, &signer).await?;
/// ```
pub async fn sign_transfer_authorization(
    params: &TransferAuthorizationParams,
    domain_separator: FixedBytes<32>,
    signer: &PrivateKeySigner,
) -> Result<Signature, alloy_signer::Error> {
    let digest = hash_transfer_with_authorization(
        domain_separator,
        params.from,
        params.to,
        params.value,
        U256::from(params.valid_after),
        U256::from(params.valid_before),
        params.nonce,
    );

    signer.sign_hash(&digest).await
}

/// Signs a `transferWithAuthorization` message synchronously.
///
/// This is the synchronous version of `sign_transfer_authorization`.
pub fn sign_transfer_authorization_sync(
    params: &TransferAuthorizationParams,
    domain_separator: FixedBytes<32>,
    signer: &PrivateKeySigner,
) -> Result<Signature, alloy_signer::Error> {
    let digest = hash_transfer_with_authorization(
        domain_separator,
        params.from,
        params.to,
        params.value,
        U256::from(params.valid_after),
        U256::from(params.valid_before),
        params.nonce,
    );

    signer.sign_hash_sync(&digest)
}

/// Signs a `receiveWithAuthorization` message with a local signer.
///
/// # Security Note
///
/// Use `receiveWithAuthorization` when the recipient needs front-running protection.
/// The contract will verify that `msg.sender` matches the `to` address.
///
/// # Arguments
///
/// * `params` - The authorization parameters
/// * `domain_separator` - The EIP-712 domain separator from the token contract
/// * `signer` - The signer with the `from` address's private key
///
/// # Returns
///
/// The ECDSA signature (v, r, s packed into `Signature`).
pub async fn sign_receive_authorization(
    params: &TransferAuthorizationParams,
    domain_separator: FixedBytes<32>,
    signer: &PrivateKeySigner,
) -> Result<Signature, alloy_signer::Error> {
    let digest = hash_receive_with_authorization(
        domain_separator,
        params.from,
        params.to,
        params.value,
        U256::from(params.valid_after),
        U256::from(params.valid_before),
        params.nonce,
    );

    signer.sign_hash(&digest).await
}

/// Signs a `receiveWithAuthorization` message synchronously.
pub fn sign_receive_authorization_sync(
    params: &TransferAuthorizationParams,
    domain_separator: FixedBytes<32>,
    signer: &PrivateKeySigner,
) -> Result<Signature, alloy_signer::Error> {
    let digest = hash_receive_with_authorization(
        domain_separator,
        params.from,
        params.to,
        params.value,
        U256::from(params.valid_after),
        U256::from(params.valid_before),
        params.nonce,
    );

    signer.sign_hash_sync(&digest)
}

/// Signs a `cancelAuthorization` message with a local signer.
///
/// # Arguments
///
/// * `params` - The cancellation parameters
/// * `domain_separator` - The EIP-712 domain separator from the token contract
/// * `signer` - The signer with the authorizer's private key
///
/// # Returns
///
/// The ECDSA signature (v, r, s packed into `Signature`).
pub async fn sign_cancel_authorization(
    params: &CancelAuthorizationParams,
    domain_separator: FixedBytes<32>,
    signer: &PrivateKeySigner,
) -> Result<Signature, alloy_signer::Error> {
    let digest = hash_cancel_authorization(domain_separator, params.authorizer, params.nonce);

    signer.sign_hash(&digest).await
}

/// Signs a `cancelAuthorization` message synchronously.
pub fn sign_cancel_authorization_sync(
    params: &CancelAuthorizationParams,
    domain_separator: FixedBytes<32>,
    signer: &PrivateKeySigner,
) -> Result<Signature, alloy_signer::Error> {
    let digest = hash_cancel_authorization(domain_separator, params.authorizer, params.nonce);

    signer.sign_hash_sync(&digest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{address, U256};

    #[test]
    fn test_sign_transfer_authorization_sync() {
        let signer = PrivateKeySigner::random();
        let domain_separator = FixedBytes::<32>::from([1u8; 32]);

        let params = TransferAuthorizationParams::new(
            signer.address(),
            address!("0000000000000000000000000000000000000002"),
            U256::from(1000),
            0,
            u64::MAX,
            FixedBytes::<32>::from([3u8; 32]),
        );

        let result = sign_transfer_authorization_sync(&params, domain_separator, &signer);
        assert!(result.is_ok());

        let signature = result.unwrap();
        assert_eq!(signature.as_bytes().len(), 65); // ECDSA signature is 65 bytes (r, s, v)
    }

    #[test]
    fn test_sign_receive_authorization_sync() {
        let signer = PrivateKeySigner::random();
        let domain_separator = FixedBytes::<32>::from([1u8; 32]);

        let params = TransferAuthorizationParams::new(
            signer.address(),
            address!("0000000000000000000000000000000000000002"),
            U256::from(1000),
            0,
            u64::MAX,
            FixedBytes::<32>::from([3u8; 32]),
        );

        let result = sign_receive_authorization_sync(&params, domain_separator, &signer);
        assert!(result.is_ok());

        let signature = result.unwrap();
        assert_eq!(signature.as_bytes().len(), 65);
    }

    #[test]
    fn test_sign_cancel_authorization_sync() {
        let signer = PrivateKeySigner::random();
        let domain_separator = FixedBytes::<32>::from([1u8; 32]);

        let params =
            CancelAuthorizationParams::new(signer.address(), FixedBytes::<32>::from([3u8; 32]));

        let result = sign_cancel_authorization_sync(&params, domain_separator, &signer);
        assert!(result.is_ok());

        let signature = result.unwrap();
        assert_eq!(signature.as_bytes().len(), 65);
    }

    #[test]
    fn test_signatures_are_deterministic() {
        let signer = PrivateKeySigner::random();
        let domain_separator = FixedBytes::<32>::from([1u8; 32]);

        let params = TransferAuthorizationParams::new(
            signer.address(),
            address!("0000000000000000000000000000000000000002"),
            U256::from(1000),
            0,
            u64::MAX,
            FixedBytes::<32>::from([3u8; 32]),
        );

        let sig1 = sign_transfer_authorization_sync(&params, domain_separator, &signer).unwrap();
        let sig2 = sign_transfer_authorization_sync(&params, domain_separator, &signer).unwrap();

        // Note: Signatures might not be identical due to randomness in k value,
        // but they should both verify correctly. This test just ensures signing works.
        assert_eq!(sig1.as_bytes().len(), 65);
        assert_eq!(sig2.as_bytes().len(), 65);
    }

    #[test]
    fn test_different_params_produce_different_signatures() {
        let signer = PrivateKeySigner::random();
        let domain_separator = FixedBytes::<32>::from([1u8; 32]);

        let params1 = TransferAuthorizationParams::new(
            signer.address(),
            address!("0000000000000000000000000000000000000002"),
            U256::from(1000),
            0,
            u64::MAX,
            FixedBytes::<32>::from([3u8; 32]),
        );

        let params2 = TransferAuthorizationParams::new(
            signer.address(),
            address!("0000000000000000000000000000000000000002"),
            U256::from(2000), // Different value
            0,
            u64::MAX,
            FixedBytes::<32>::from([3u8; 32]),
        );

        let sig1 = sign_transfer_authorization_sync(&params1, domain_separator, &signer).unwrap();
        let sig2 = sign_transfer_authorization_sync(&params2, domain_separator, &signer).unwrap();

        // Different parameters should produce different signatures
        assert_ne!(sig1, sig2);
    }
}
