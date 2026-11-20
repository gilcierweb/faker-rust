//! Crypto generator - generates random cryptocurrency coins, names, and symbols

use crate::base::sample;
use crate::locale::{fetch_locale_with_context, sample_with_resolve};

/// Generate a random cryptocurrency coin (e.g. "Bitcoin")
pub fn coin() -> String {
    fetch_locale_with_context(
        "crypto_coin.coin",
        &crate::config::FakerConfig::locale(),
        Some("crypto_coin"),
    )
    .map(|v| sample_with_resolve(&v, Some("crypto_coin")))
    .unwrap_or_else(|| sample(FALLBACK_COINS).to_string())
}

/// Generate a random cryptocurrency name (e.g. "Bitcoin")
pub fn name() -> String {
    coin()
}

/// Generate a random cryptocurrency symbol (e.g. "BTC")
pub fn symbol() -> String {
    fetch_locale_with_context(
        "crypto_coin.symbol",
        &crate::config::FakerConfig::locale(),
        Some("crypto_coin"),
    )
    .map(|v| sample_with_resolve(&v, Some("crypto_coin")))
    .unwrap_or_else(|| sample(FALLBACK_SYMBOLS).to_string())
}

// Fallback data (shared with the `crypto_coin` compatibility module)
const FALLBACK_COINS: &[&str] = &[
    "Bitcoin",
    "Ethereum",
    "Ripple",
    "Litecoin",
    "Cardano",
    "Polkadot",
    "Stellar",
    "Chainlink",
    "Dogecoin",
    "Uniswap",
    "Solana",
    "Avalanche",
    "Polygon",
    "Binance Coin",
    "Terra",
    "Cosmos",
    "Algorand",
    "VeChain",
];
const FALLBACK_SYMBOLS: &[&str] = &[
    "BTC", "ETH", "XRP", "LTC", "ADA", "DOT", "XLM", "LINK", "DOGE", "UNI", "SOL", "AVAX", "MATIC",
    "BNB", "LUNA", "ATOM", "ALGO", "VET",
];
// --- gem parity generators (algorithmic) ---
fn random_hex(length: usize) -> String {
    let config = crate::config::FakerConfig::current();
    (0..length)
        .map(|_| config.rand_char(&crate::base::HEX_LOWER))
        .collect()
}

/// Gem-compatible generator: random MD5-shaped hex digest.
pub fn md5() -> String {
    random_hex(32)
}
/// Gem-compatible generator: random SHA1-shaped hex digest.
pub fn sha1() -> String {
    random_hex(40)
}
/// Gem-compatible generator: random SHA256-shaped hex digest.
pub fn sha256() -> String {
    random_hex(64)
}
/// Gem-compatible generator: random SHA512-shaped hex digest.
pub fn sha512() -> String {
    random_hex(128)
}

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
