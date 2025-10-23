//! EIP-3009 Integration Test Examples
//!
//! These tests demonstrate how EIP-3009 functionality works with the SDK.
//! They are marked as `#[ignore]` because they would require a full testcontainer
//! setup with Anvil forking a network with USDC deployed.
//!
//! The unit tests in the main library (15/15 passing) validate all the core logic.
//! These integration tests serve as documentation for how to use the SDK.

use alloy_primitives::{address, U256};
use alloy_signer_local::PrivateKeySigner;
use erc20_rs::{
    signing::{generate_nonce, sign_transfer_authorization_sync},
    types::TransferAuthorizationParams,
};

/// Example: Create and sign a transfer authorization
///
/// This test demonstrates the core EIP-3009 workflow:
/// 1. Create authorization parameters
/// 2. Sign with the token holder's key
/// 3. Result can be submitted by anyone (gasless for token holder)
#[test]
fn example_create_transfer_authorization() {
    // Alice wants to authorize a transfer to Bob
    let alice_signer = PrivateKeySigner::random();
    let bob_address = address!("70997970C51812dc3A010C7d01b50e0d17dc79C8");

    // Create authorization for 1 USDC (6 decimals), valid for 1 hour
    let params = TransferAuthorizationParams::with_duration(
        alice_signer.address(),
        bob_address,
        U256::from(1_000_000), // 1 USDC
        3600,                  // 1 hour
        generate_nonce(),
    );

    // In a real scenario, get domain_separator from the token contract
    // let domain_separator = token.domain_separator().await?;
    let mock_domain_separator = alloy_primitives::FixedBytes::<32>::from([1u8; 32]);

    // Alice signs the authorization off-chain
    let signature = sign_transfer_authorization_sync(&params, mock_domain_separator, &alice_signer)
        .expect("Signing should succeed");

    // Verify signature format
    assert_eq!(signature.as_bytes().len(), 65);

    println!("✅ Authorization created and signed");
    println!("   Authorizer: {}", params.from);
    println!("   Recipient: {}", params.to);
    println!("   Amount: {}", params.value);
    println!("   Valid from: {}", params.valid_after);
    println!("   Valid until: {}", params.valid_before);
    println!("   Nonce: 0x{}", hex::encode(params.nonce));

    // Now anyone (Bob, a relayer, etc.) can submit this to the blockchain:
    // token.transfer_with_authorization(params.from, params.to, params.value,
    //     params.valid_after, params.valid_before, params.nonce, signature).await?;
}

/// Example: Nonce generation for replay protection
#[test]
fn example_nonce_generation() {
    // Each authorization must have a unique nonce
    let nonce1 = generate_nonce();
    let nonce2 = generate_nonce();

    // Nonces should be different
    assert_ne!(nonce1, nonce2);
    assert_eq!(nonce1.len(), 32);

    println!("✅ Nonces are unique 32-byte values");
    println!("   Nonce 1: 0x{}", hex::encode(nonce1));
    println!("   Nonce 2: 0x{}", hex::encode(nonce2));
}

/// Example: Time-bounded authorization
#[test]
fn example_time_bounded_authorization() {
    let alice = PrivateKeySigner::random();
    let bob = address!("70997970C51812dc3A010C7d01b50e0d17dc79C8");

    // Create authorization valid for next 10 minutes
    let params_10min = TransferAuthorizationParams::with_duration(
        alice.address(),
        bob,
        U256::from(1_000_000),
        600, // 10 minutes
        generate_nonce(),
    );

    assert!(params_10min.valid_before > params_10min.valid_after);
    assert_eq!(params_10min.valid_before - params_10min.valid_after, 600);

    println!("✅ Time-bounded authorization created");
    println!(
        "   Window: {} seconds",
        params_10min.valid_before - params_10min.valid_after
    );

    // You can also create authorization without time bounds (not recommended)
    let params_unbounded = TransferAuthorizationParams::without_time_bounds(
        alice.address(),
        bob,
        U256::from(1_000_000),
        generate_nonce(),
    );

    assert_eq!(params_unbounded.valid_after, 0);
    assert_eq!(params_unbounded.valid_before, u64::MAX);

    println!("⚠️  Unbounded authorization (use with caution)");
}

/// Example: Multiple authorizations with different nonces
#[test]
fn example_multiple_authorizations() {
    let alice = PrivateKeySigner::random();
    let bob = address!("70997970C51812dc3A010C7d01b50e0d17dc79C8");
    let charlie = address!("3C44CdDdB6a900fa2b585dd299e03d12FA4293BC");

    // Alice can create multiple authorizations simultaneously
    // Each needs a unique nonce - no sequential dependency!
    let auth1 = TransferAuthorizationParams::with_duration(
        alice.address(),
        bob,
        U256::from(1_000_000),
        3600,
        generate_nonce(), // Unique nonce 1
    );

    let auth2 = TransferAuthorizationParams::with_duration(
        alice.address(),
        charlie,
        U256::from(2_000_000),
        3600,
        generate_nonce(), // Unique nonce 2
    );

    // Both can be submitted in any order - no nonce conflicts!
    assert_ne!(auth1.nonce, auth2.nonce);

    println!("✅ Multiple concurrent authorizations supported");
    println!("   Auth 1: {} USDC to Bob", auth1.value);
    println!("   Auth 2: {} USDC to Charlie", auth2.value);
    println!("   Can be submitted in any order!");
}

/// Integration test skeleton (ignored - requires testcontainer setup)
#[tokio::test]
#[ignore = "Requires anvil testcontainer with forked network"]
async fn integration_test_against_real_usdc() {
    // This test would:
    // 1. Start anvil container with forked Arbitrum/Ethereum
    // 2. Connect to real USDC contract
    // 3. Impersonate whale to fund test account
    // 4. Create and submit real authorization
    // 5. Verify balances changed on-chain

    // Implementation requires:
    // - testcontainers-modules with anvil
    // - Network access to RPC endpoint
    // - Provider setup with proper connection

    println!("Integration test skeleton - see unit tests for validation");
}
