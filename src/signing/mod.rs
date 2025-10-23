//! Utilities for EIP-712 signing and EIP-3009 authorization signatures.
//!
//! This module provides helper functions for creating and verifying EIP-712 typed data
//! signatures used in EIP-3009 transfer authorizations.

pub mod authorization;
pub mod eip712;

pub use authorization::*;
pub use eip712::*;

use alloy_primitives::FixedBytes;
use rand::Rng;

/// Generates a cryptographically secure random 32-byte nonce.
///
/// This nonce should be used for EIP-3009 authorization to prevent replay attacks.
/// Each authorization MUST use a unique nonce.
///
/// # Example
///
/// ```
/// use erc20_rs::signing::generate_nonce;
///
/// let nonce = generate_nonce();
/// println!("Generated nonce: {:?}", nonce);
/// ```
pub fn generate_nonce() -> FixedBytes<32> {
    let mut rng = rand::thread_rng();
    let mut bytes = [0u8; 32];
    rng.fill(&mut bytes);
    FixedBytes::from(bytes)
}

/// Creates time bounds for an authorization with a specified duration.
///
/// Returns `(valid_after, valid_before)` as Unix timestamps.
///
/// # Arguments
///
/// * `duration_seconds` - How long the authorization should remain valid (e.g., 3600 for 1 hour)
///
/// # Example
///
/// ```
/// use erc20_rs::signing::create_time_bounds;
///
/// // Create authorization valid for next hour
/// let (valid_after, valid_before) = create_time_bounds(3600);
/// ```
pub fn create_time_bounds(duration_seconds: u64) -> (u64, u64) {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    (now, now + duration_seconds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_nonce() {
        let nonce1 = generate_nonce();
        let nonce2 = generate_nonce();

        // Nonces should be different
        assert_ne!(nonce1, nonce2);

        // Nonces should be 32 bytes
        assert_eq!(nonce1.len(), 32);
        assert_eq!(nonce2.len(), 32);
    }

    #[test]
    fn test_create_time_bounds() {
        let duration = 3600; // 1 hour
        let (valid_after, valid_before) = create_time_bounds(duration);

        assert!(valid_before > valid_after);
        assert_eq!(valid_before - valid_after, duration);

        // Should be roughly current time
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        assert!((valid_after as i64 - now as i64).abs() < 2); // Within 2 seconds
    }
}
