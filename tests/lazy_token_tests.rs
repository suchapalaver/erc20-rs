use alloy::primitives::{address, U256};
use alloy::providers::ProviderBuilder;
use alloy_erc20_full::LazyToken;

/// Test reading token metadata (name, symbol, decimals)
/// Uses DAI on Ethereum mainnet as a known-good token
#[tokio::test]
#[ignore] // Requires network access
async fn test_lazy_token_metadata() {
    let rpc_url =
        std::env::var("ETH_MAINNET_RPC").unwrap_or_else(|_| "https://eth.llamarpc.com".to_string());

    let provider = ProviderBuilder::new().connect_http(rpc_url.parse().unwrap());

    // DAI token on Ethereum mainnet
    let dai_address = address!("6B175474E89094C44Da98b954EedeAC495271d0F");
    let dai = LazyToken::new(dai_address, provider);

    // Test symbol (cached)
    let symbol = dai.symbol().await.unwrap();
    assert_eq!(symbol, "DAI");

    // Second call should use cache
    let symbol_again = dai.symbol().await.unwrap();
    assert_eq!(symbol_again, "DAI");

    // Test name (cached)
    let name = dai.name().await.unwrap();
    assert!(name.contains("Dai"));

    // Test decimals (cached)
    let decimals = dai.decimals().await.unwrap();
    assert_eq!(*decimals, 18);
}

/// Test reading balances
#[tokio::test]
#[ignore] // Requires network access
async fn test_lazy_token_balance_of() {
    let rpc_url =
        std::env::var("ETH_MAINNET_RPC").unwrap_or_else(|_| "https://eth.llamarpc.com".to_string());

    let provider = ProviderBuilder::new().connect_http(rpc_url.parse().unwrap());

    let dai_address = address!("6B175474E89094C44Da98b954EedeAC495271d0F");
    let dai = LazyToken::new(dai_address, provider);

    // Test balance_of for vitalik.eth
    let vitalik = address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
    let balance = dai.balance_of(vitalik).await.unwrap();

    // Balance should be a valid U256 (might be 0)
    assert!(balance >= U256::ZERO);
}

/// Test total supply
#[tokio::test]
#[ignore] // Requires network access
async fn test_lazy_token_total_supply() {
    let rpc_url =
        std::env::var("ETH_MAINNET_RPC").unwrap_or_else(|_| "https://eth.llamarpc.com".to_string());

    let provider = ProviderBuilder::new().connect_http(rpc_url.parse().unwrap());

    let dai_address = address!("6B175474E89094C44Da98b954EedeAC495271d0F");
    let dai = LazyToken::new(dai_address, provider);

    let supply = dai.total_supply().await.unwrap();

    // DAI has significant supply
    assert!(supply > U256::from(1_000_000_000_000_000_000u64));
}

/// Test allowance
#[tokio::test]
#[ignore] // Requires network access
async fn test_lazy_token_allowance() {
    let rpc_url =
        std::env::var("ETH_MAINNET_RPC").unwrap_or_else(|_| "https://eth.llamarpc.com".to_string());

    let provider = ProviderBuilder::new().connect_http(rpc_url.parse().unwrap());

    let dai_address = address!("6B175474E89094C44Da98b954EedeAC495271d0F");
    let dai = LazyToken::new(dai_address, provider);

    let owner = address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
    let spender = address!("1111111254EEB25477B68fb85Ed929f73A960582"); // 1inch router

    let allowance = dai.allowance(owner, spender).await.unwrap();

    // Should return valid U256 (might be 0)
    assert!(allowance >= U256::ZERO);
}

/// Test BigDecimal conversion
#[tokio::test]
#[ignore] // Requires network access
async fn test_lazy_token_get_balance() {
    let rpc_url =
        std::env::var("ETH_MAINNET_RPC").unwrap_or_else(|_| "https://eth.llamarpc.com".to_string());

    let provider = ProviderBuilder::new().connect_http(rpc_url.parse().unwrap());

    let dai_address = address!("6B175474E89094C44Da98b954EedeAC495271d0F");
    let dai = LazyToken::new(dai_address, provider);

    // Test with known amount
    let amount = U256::from(1_000_000_000_000_000_000u64); // 1.0 DAI
    let balance_decimal = dai.get_balance(amount).await.unwrap();

    // Should be approximately 1.0
    let balance_str = balance_decimal.to_string();
    assert!(balance_str.starts_with("1"));
}

/// Test that instance field is accessible (compile-time test)
#[tokio::test]
async fn test_instance_field_is_public() {
    // This test verifies the core feature we added: public instance field
    let rpc_url = "https://eth.llamarpc.com";
    let provider = ProviderBuilder::new().connect_http(rpc_url.parse().unwrap());

    let dai_address = address!("6B175474E89094C44Da98b954EedeAC495271d0F");
    let dai = LazyToken::new(dai_address, provider);

    // This should compile - accessing public instance field
    let _instance = &dai.instance;

    // Verify we can access the address from instance
    assert_eq!(dai.instance.address(), &dai_address);
}

/// Test address getter
#[test]
fn test_lazy_token_address() {
    let rpc_url = "https://eth.llamarpc.com";
    let provider = ProviderBuilder::new().connect_http(rpc_url.parse().unwrap());

    let dai_address = address!("6B175474E89094C44Da98b954EedeAC495271d0F");
    let dai = LazyToken::new(dai_address, provider);

    assert_eq!(dai.address(), &dai_address);
}
