//! Finance generator - generates finance-related data

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random stock ticker symbol
pub fn stock_ticker() -> String {
    generate_ticker()
}

fn generate_ticker() -> String {
    let config = crate::config::FakerConfig::current();
    const LETTERS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let length = config.rand_range(2, 5);
    let mut ticker = String::new();
    for _ in 0..length {
        let idx = config.rand_u32(26) as usize;
        ticker.push(LETTERS[idx] as char);
    }
    ticker
}

/// Generate a random market index
pub fn market_index() -> String {
    fetch_locale(
        "finance.market_indices",
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(|| sample(FALLBACK_INDICES).to_string())
}

/// Generate a random currency pair
pub fn currency_pair() -> String {
    let base = sample(FALLBACK_CURRENCIES);
    let quote = sample(FALLBACK_CURRENCIES);
    format!("{}/{}", base, quote)
}

// Fallback data
const FALLBACK_INDICES: &[&str] = &[
    "S&P 500",
    "Dow Jones",
    "NASDAQ",
    "Russell 2000",
    "FTSE 100",
    "DAX",
    "Nikkei 225",
    "Hang Seng",
    "Shanghai Composite",
    "CAC 40",
    "Euro Stoxx 50",
    "IBOVESPA",
    "ASX 200",
    "TSX Composite",
];

const FALLBACK_CURRENCIES: &[&str] = &[
    "USD", "EUR", "GBP", "JPY", "CHF", "CAD", "AUD", "NZD", "CNY", "HKD", "SGD", "MXN", "BRL",
    "INR", "RUB", "ZAR", "KRW", "SEK", "NOK", "DKK", "PLN", "TRY", "AED", "THB",
];

// --- gem parity generators (automated, locale-backed) ---
fn parity_pick(key: &str) -> String {
    crate::locale::fetch_locale(key, &crate::config::FakerConfig::locale())
        .map(|v| crate::base::sample(&v))
        .unwrap_or_default()
}

/// Gem-compatible generator: `finance.stock_market`.
pub fn stock_market() -> String {
    parity_pick("finance.stock_market")
}
// --- gem parity generators (dynamic locale keys) ---
/// Gem-compatible generator: credit card number for the given card types.
pub fn credit_card(types: &[&str]) -> String {
    let default_types = ["visa", "mastercard", "american_express", "discover"];
    let selected = if types.is_empty() {
        &default_types[..]
    } else {
        types
    };
    let card_type = crate::base::sample(selected);
    crate::locale::fetch_locale(
        &format!("finance.credit_card.{card_type}"),
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| crate::base::numerify(crate::base::sample(&v).trim_matches('/')))
    .unwrap_or_default()
}
/// Gem-compatible generator: stock ticker from the given markets.
pub fn ticker(markets: &[&str]) -> String {
    let default_markets = ["NYSE", "NASDAQ"];
    let selected = if markets.is_empty() {
        &default_markets[..]
    } else {
        markets
    };
    let market = crate::base::sample(selected);
    crate::locale::fetch_locale(
        &format!("finance.ticker.{market}"),
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| crate::base::sample(&v))
    .unwrap_or_default()
}
/// Gem-compatible generator: VAT number for a country code.
pub fn vat_number(country: &str) -> String {
    crate::locale::fetch_locale_named(
        "finance.vat_number",
        country,
        &crate::config::FakerConfig::locale(),
    )
    .map(|template| crate::base::numerify(&template))
    .unwrap_or_default()
}

/// Gem-compatible generator: all country codes with a VAT number template.
pub fn vat_number_keys() -> Vec<String> {
    crate::locale::fetch_locale_keys("finance.vat_number", &crate::config::FakerConfig::locale())
}

/// Gem-compatible generator: Italian condominium fiscal code for `country`.
pub fn condominium_fiscal_code(country: &str) -> String {
    crate::locale::fetch_locale_named(
        "finance.condominium_fiscal_code",
        country,
        &crate::config::FakerConfig::locale(),
    )
    .map(|template| crate::base::numerify(&template))
    .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stock_ticker() {
        assert!(!stock_ticker().is_empty());
    }

    #[test]
    fn test_market_index() {
        assert!(!market_index().is_empty());
    }

    #[test]
    fn test_currency_pair() {
        assert!(!currency_pair().is_empty());
    }
}
