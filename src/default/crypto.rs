//! Crypto generator - generates random cryptocurrency coins, names, and symbols

use crate::base::sample;
use crate::locale::{fetch_locale_with_context, sample_with_resolve};

/// Generate a random cryptocurrency coin (e.g. "Bitcoin")
pub fn coin() -> String {
    fetch_locale_with_context("crypto_coin.coin", "en", Some("crypto_coin"))
        .map(|v| sample_with_resolve(&v, Some("crypto_coin")))
        .unwrap_or_else(|| sample(FALLBACK_COINS).to_string())
}

/// Generate a random cryptocurrency name (e.g. "Bitcoin")
pub fn name() -> String {
    coin()
}

/// Generate a random cryptocurrency symbol (e.g. "BTC")
pub fn symbol() -> String {
    fetch_locale_with_context("crypto_coin.symbol", "en", Some("crypto_coin"))
        .map(|v| sample_with_resolve(&v, Some("crypto_coin")))
        .unwrap_or_else(|| sample(FALLBACK_SYMBOLS).to_string())
}

// Fallback data
const FALLBACK_COINS: &[&str] = &["Bitcoin", "Ethereum", "Litecoin", "Ripple", "Dogecoin"];
const FALLBACK_SYMBOLS: &[&str] = &["BTC", "ETH", "LTC", "XRP", "DOGE"];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coin() {
        assert!(!coin().is_empty());
    }

    #[test]
    fn test_symbol() {
        assert!(!symbol().is_empty());
    }
}
