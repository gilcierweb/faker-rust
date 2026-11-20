//! Crypto coin generator (compatibility re-export of [`crate::crypto`]).

/// Generate a random cryptocurrency coin name.
pub use crate::crypto::coin;

/// Generate a random cryptocurrency symbol (also called an acronym).
pub use crate::crypto::symbol;

/// Alias kept for backwards compatibility with the old `crypto_coin` API.
pub use crate::crypto::symbol as acronym;

// --- gem parity generators (automated, locale-backed) ---
fn parity_pick(key: &str) -> String {
    crate::locale::fetch_locale(key, "en")
        .map(|v| crate::base::sample(&v))
        .unwrap_or_default()
}

/// Gem-compatible generator: `crypto_coin.coin`.
pub fn coin_array() -> String {
    parity_pick("crypto_coin.coin")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reexports_are_usable() {
        assert!(!coin().is_empty());
        assert!(!acronym().is_empty());
        assert!(!symbol().is_empty());
    }
}
