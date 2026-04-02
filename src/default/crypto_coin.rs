//! Crypto coin generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random cryptocurrency coin name
pub fn coin() -> String {
    fetch_locale("crypto_coin.coins", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_COINS).to_string())
}

/// Generate a random coin acronym/symbol
pub fn acronym() -> String {
    fetch_locale("crypto_coin.acronyms", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_ACRONYMS).to_string())
}

// Fallback data
const FALLBACK_COINS: &[&str] = &[
    "Bitcoin", "Ethereum", "Ripple", "Litecoin", "Cardano", "Polkadot",
    "Stellar", "Chainlink", "Dogecoin", "Uniswap", "Solana", "Avalanche",
    "Polygon", "Binance Coin", "Terra", "Cosmos", "Algorand", "VeChain",
];

const FALLBACK_ACRONYMS: &[&str] = &[
    "BTC", "ETH", "XRP", "LTC", "ADA", "DOT", "XLM", "LINK", "DOGE",
    "UNI", "SOL", "AVAX", "MATIC", "BNB", "LUNA", "ATOM", "ALGO", "VET",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coin() {
        assert!(!coin().is_empty());
    }

    #[test]
    fn test_acronym() {
        assert!(!acronym().is_empty());
    }
}
