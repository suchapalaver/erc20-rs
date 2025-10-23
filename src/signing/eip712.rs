//! EIP-712 typed data hashing utilities for EIP-3009.
//!
//! This module implements the EIP-712 structured data hashing standard
//! as required by EIP-3009 for authorization signatures.

use alloy_primitives::{keccak256, Address, FixedBytes, U256};
use alloy_sol_types::{sol, SolStruct};

// EIP-712 type hashes for EIP-3009 (from USDC implementation)
// These match the constants in the USDC FiatTokenV2_1 contract

/// Type hash for `transferWithAuthorization`.
///
/// ```solidity
/// keccak256("TransferWithAuthorization(address from,address to,uint256 value,uint256 validAfter,uint256 validBefore,bytes32 nonce)")
/// ```
pub const TRANSFER_WITH_AUTHORIZATION_TYPEHASH: FixedBytes<32> = FixedBytes::new([
    0x7c, 0x7c, 0x6c, 0xdb, 0x67, 0xa1, 0x87, 0x43, 0xf4, 0x9e, 0xc6, 0xfa, 0x9b, 0x35, 0xf5, 0x0d,
    0x52, 0xed, 0x05, 0xcb, 0xed, 0x4c, 0xc5, 0x92, 0xe1, 0x3b, 0x44, 0x50, 0x1c, 0x1a, 0x22, 0x67,
]);

/// Type hash for `receiveWithAuthorization`.
///
/// ```solidity
/// keccak256("ReceiveWithAuthorization(address from,address to,uint256 value,uint256 validAfter,uint256 validBefore,bytes32 nonce)")
/// ```
pub const RECEIVE_WITH_AUTHORIZATION_TYPEHASH: FixedBytes<32> = FixedBytes::new([
    0xd0, 0x99, 0xcc, 0x98, 0xef, 0x71, 0x10, 0x7a, 0x61, 0x6c, 0x4f, 0x0f, 0x94, 0x1f, 0x04, 0xc3,
    0x22, 0xd8, 0xe2, 0x54, 0xfe, 0x26, 0xb3, 0xc6, 0x66, 0x8d, 0xb8, 0x7a, 0xae, 0x41, 0x3d, 0xe8,
]);

/// Type hash for `cancelAuthorization`.
///
/// ```solidity
/// keccak256("CancelAuthorization(address authorizer,bytes32 nonce)")
/// ```
pub const CANCEL_AUTHORIZATION_TYPEHASH: FixedBytes<32> = FixedBytes::new([
    0x15, 0x8b, 0x0a, 0x9e, 0xdf, 0x7a, 0x82, 0x8a, 0xad, 0x02, 0xf6, 0x3c, 0xd5, 0x15, 0xc6, 0x8e,
    0xf2, 0xf5, 0x0b, 0xa8, 0x07, 0x39, 0x6f, 0x6d, 0x12, 0x84, 0x28, 0x33, 0xa1, 0x59, 0x74, 0x29,
]);

sol! {
    /// EIP-712 struct for `transferWithAuthorization`.
    #[derive(Debug, PartialEq, Eq)]
    struct TransferWithAuthorization {
        address from;
        address to;
        uint256 value;
        uint256 validAfter;
        uint256 validBefore;
        bytes32 nonce;
    }

    /// EIP-712 struct for `receiveWithAuthorization`.
    #[derive(Debug, PartialEq, Eq)]
    struct ReceiveWithAuthorization {
        address from;
        address to;
        uint256 value;
        uint256 validAfter;
        uint256 validBefore;
        bytes32 nonce;
    }

    /// EIP-712 struct for `cancelAuthorization`.
    #[derive(Debug, PartialEq, Eq)]
    struct CancelAuthorization {
        address authorizer;
        bytes32 nonce;
    }
}

/// Computes the EIP-712 digest for `transferWithAuthorization`.
///
/// This digest can be signed with a private key to create an authorization.
///
/// # Arguments
///
/// * `domain_separator` - The EIP-712 domain separator from the token contract
/// * `from` - Token holder authorizing the transfer
/// * `to` - Recipient address
/// * `value` - Amount to transfer
/// * `valid_after` - Unix timestamp after which the authorization is valid
/// * `valid_before` - Unix timestamp before which the authorization is valid
/// * `nonce` - Unique 32-byte nonce
///
/// # Returns
///
/// The 32-byte digest ready to be signed.
pub fn hash_transfer_with_authorization(
    domain_separator: FixedBytes<32>,
    from: Address,
    to: Address,
    value: U256,
    valid_after: U256,
    valid_before: U256,
    nonce: FixedBytes<32>,
) -> FixedBytes<32> {
    let struct_hash = TransferWithAuthorization {
        from,
        to,
        value,
        validAfter: valid_after,
        validBefore: valid_before,
        nonce,
    }
    .eip712_hash_struct();

    compute_eip712_digest(domain_separator, struct_hash)
}

/// Computes the EIP-712 digest for `receiveWithAuthorization`.
///
/// # Arguments
///
/// * `domain_separator` - The EIP-712 domain separator from the token contract
/// * `from` - Token holder authorizing the transfer
/// * `to` - Recipient address (must match `msg.sender` for front-run protection)
/// * `value` - Amount to transfer
/// * `valid_after` - Unix timestamp after which the authorization is valid
/// * `valid_before` - Unix timestamp before which the authorization is valid
/// * `nonce` - Unique 32-byte nonce
///
/// # Returns
///
/// The 32-byte digest ready to be signed.
pub fn hash_receive_with_authorization(
    domain_separator: FixedBytes<32>,
    from: Address,
    to: Address,
    value: U256,
    valid_after: U256,
    valid_before: U256,
    nonce: FixedBytes<32>,
) -> FixedBytes<32> {
    let struct_hash = ReceiveWithAuthorization {
        from,
        to,
        value,
        validAfter: valid_after,
        validBefore: valid_before,
        nonce,
    }
    .eip712_hash_struct();

    compute_eip712_digest(domain_separator, struct_hash)
}

/// Computes the EIP-712 digest for `cancelAuthorization`.
///
/// # Arguments
///
/// * `domain_separator` - The EIP-712 domain separator from the token contract
/// * `authorizer` - The address that created the original authorization
/// * `nonce` - The nonce of the authorization to cancel
///
/// # Returns
///
/// The 32-byte digest ready to be signed.
pub fn hash_cancel_authorization(
    domain_separator: FixedBytes<32>,
    authorizer: Address,
    nonce: FixedBytes<32>,
) -> FixedBytes<32> {
    let struct_hash = CancelAuthorization { authorizer, nonce }.eip712_hash_struct();

    compute_eip712_digest(domain_separator, struct_hash)
}

/// Computes the final EIP-712 digest from domain separator and struct hash.
///
/// # EIP-712 Specification
///
/// The digest is computed as:
/// ```text
/// keccak256("\x19\x01" ‖ domainSeparator ‖ structHash)
/// ```
fn compute_eip712_digest(
    domain_separator: FixedBytes<32>,
    struct_hash: FixedBytes<32>,
) -> FixedBytes<32> {
    let mut data = Vec::with_capacity(66);
    data.extend_from_slice(b"\x19\x01");
    data.extend_from_slice(domain_separator.as_slice());
    data.extend_from_slice(struct_hash.as_slice());
    keccak256(&data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::address;

    // Test vectors from EIP-3009 reference implementation
    #[test]
    fn test_typehash_constants() {
        // Verify type hashes match expected values
        let transfer_typehash_str =
            "TransferWithAuthorization(address from,address to,uint256 value,uint256 validAfter,uint256 validBefore,bytes32 nonce)";
        let computed = keccak256(transfer_typehash_str.as_bytes());
        assert_eq!(computed, TRANSFER_WITH_AUTHORIZATION_TYPEHASH);

        let receive_typehash_str =
            "ReceiveWithAuthorization(address from,address to,uint256 value,uint256 validAfter,uint256 validBefore,bytes32 nonce)";
        let computed = keccak256(receive_typehash_str.as_bytes());
        assert_eq!(computed, RECEIVE_WITH_AUTHORIZATION_TYPEHASH);

        let cancel_typehash_str = "CancelAuthorization(address authorizer,bytes32 nonce)";
        let computed = keccak256(cancel_typehash_str.as_bytes());
        assert_eq!(computed, CANCEL_AUTHORIZATION_TYPEHASH);
    }

    #[test]
    fn test_hash_transfer_with_authorization() {
        let domain_separator = FixedBytes::<32>::from([1u8; 32]);
        let from = address!("0000000000000000000000000000000000000001");
        let to = address!("0000000000000000000000000000000000000002");
        let value = U256::from(1000);
        let valid_after = U256::from(0);
        let valid_before = U256::from(u64::MAX);
        let nonce = FixedBytes::<32>::from([3u8; 32]);

        let digest = hash_transfer_with_authorization(
            domain_separator,
            from,
            to,
            value,
            valid_after,
            valid_before,
            nonce,
        );

        // Digest should be 32 bytes
        assert_eq!(digest.len(), 32);

        // Digest should be deterministic
        let digest2 = hash_transfer_with_authorization(
            domain_separator,
            from,
            to,
            value,
            valid_after,
            valid_before,
            nonce,
        );
        assert_eq!(digest, digest2);

        // Different inputs should produce different digests
        let digest3 = hash_transfer_with_authorization(
            domain_separator,
            from,
            to,
            U256::from(2000), // Different value
            valid_after,
            valid_before,
            nonce,
        );
        assert_ne!(digest, digest3);
    }

    #[test]
    fn test_hash_receive_with_authorization() {
        let domain_separator = FixedBytes::<32>::from([1u8; 32]);
        let from = address!("0000000000000000000000000000000000000001");
        let to = address!("0000000000000000000000000000000000000002");
        let value = U256::from(1000);
        let valid_after = U256::from(0);
        let valid_before = U256::from(u64::MAX);
        let nonce = FixedBytes::<32>::from([3u8; 32]);

        let digest = hash_receive_with_authorization(
            domain_separator,
            from,
            to,
            value,
            valid_after,
            valid_before,
            nonce,
        );

        assert_eq!(digest.len(), 32);

        // receiveWithAuthorization should produce different hash than transferWithAuthorization
        // (different type hash)
        let transfer_digest = hash_transfer_with_authorization(
            domain_separator,
            from,
            to,
            value,
            valid_after,
            valid_before,
            nonce,
        );
        assert_ne!(digest, transfer_digest);
    }

    #[test]
    fn test_hash_cancel_authorization() {
        let domain_separator = FixedBytes::<32>::from([1u8; 32]);
        let authorizer = address!("0000000000000000000000000000000000000001");
        let nonce = FixedBytes::<32>::from([3u8; 32]);

        let digest = hash_cancel_authorization(domain_separator, authorizer, nonce);

        assert_eq!(digest.len(), 32);

        // Deterministic
        let digest2 = hash_cancel_authorization(domain_separator, authorizer, nonce);
        assert_eq!(digest, digest2);

        // Different nonce produces different digest
        let different_nonce = FixedBytes::<32>::from([4u8; 32]);
        let digest3 = hash_cancel_authorization(domain_separator, authorizer, different_nonce);
        assert_ne!(digest, digest3);
    }
}
