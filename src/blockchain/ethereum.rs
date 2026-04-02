//! Ethereum blockchain generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Ethereum address
pub fn address() -> String {
    fetch_locale("ethereum.addresses", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| generate_ethereum_address())
}

fn generate_ethereum_address() -> String {
    // Generate a mock Ethereum address (0x + 40 hex characters)
    let mut address = String::from("0x");
    const HEX: &[u8] = b"0123456789abcdef";
    let config = crate::config::FakerConfig::current();

    for _ in 0..40 {
        let idx = config.rand_range(0, (HEX.len() - 1) as u32) as usize;
        address.push(HEX[idx] as char);
    }

    address
}

// Fallback data (sample real Ethereum addresses)
const FALLBACK_ADDRESSES: &[&str] = &[
    "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb",
    "0xdAC17F958D2ee523a2206206994597C13D831ec7",
    "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address() {
        let addr = address();
        assert!(!addr.is_empty());
        // Ethereum addresses start with 0x and are 42 characters long
        assert!(addr.starts_with("0x"));
        assert_eq!(addr.len(), 42);
    }
}
