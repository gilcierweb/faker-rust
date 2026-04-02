//! Currency generator - generates random currency names, codes, and symbols

use crate::base::sample;
use crate::locale::{fetch_locale_with_context, sample_with_resolve};

/// Generate a random currency name
pub fn name() -> String {
    fetch_locale_with_context("currency.name", "en", Some("currency"))
        .map(|v| sample_with_resolve(&v, Some("currency")))
        .unwrap_or_else(|| sample(FALLBACK_NAMES).to_string())
}

/// Generate a random currency code
pub fn code() -> String {
    fetch_locale_with_context("currency.code", "en", Some("currency"))
        .map(|v| sample_with_resolve(&v, Some("currency")))
        .unwrap_or_else(|| sample(FALLBACK_CODES).to_string())
}

/// Generate a random currency symbol
pub fn symbol() -> String {
    fetch_locale_with_context("currency.symbol", "en", Some("currency"))
        .map(|v| sample_with_resolve(&v, Some("currency")))
        .unwrap_or_else(|| sample(FALLBACK_SYMBOLS).to_string())
}

// Fallback data
const FALLBACK_NAMES: &[&str] = &[
    "US Dollar",
    "Euro",
    "Japanese Yen",
    "British Pound",
    "Bitcoin",
];
const FALLBACK_CODES: &[&str] = &["USD", "EUR", "JPY", "GBP", "BTC"];
const FALLBACK_SYMBOLS: &[&str] = &["$", "€", "¥", "£", "₿"];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert!(!name().is_empty());
    }

    #[test]
    fn test_code() {
        assert!(!code().is_empty());
    }
}
