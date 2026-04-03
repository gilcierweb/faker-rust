//! Finance generator - generates finance-related data

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random stock ticker symbol
pub fn stock_ticker() -> String {
    fetch_locale("finance.stock_tickers", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(generate_ticker)
}

fn generate_ticker() -> String {
    let config = crate::config::FakerConfig::current();
    const LETTERS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let length = config.rand_range(2, 5);
    let mut ticker = String::new();
    for _ in 0..length {
        let idx = config.rand_range(0, 26) as usize;
        ticker.push(LETTERS[idx] as char);
    }
    ticker
}

/// Generate a random market index
pub fn market_index() -> String {
    fetch_locale("finance.market_indices", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_INDICES).to_string())
}

/// Generate a random currency pair
pub fn currency_pair() -> String {
    let base = sample(FALLBACK_CURRENCIES);
    let quote = sample(FALLBACK_CURRENCIES);
    format!("{}/{})", base, quote)
}

// Fallback data
const FALLBACK_INDICES: &[&str] = &[
    "S&P 500", "Dow Jones", "NASDAQ", "Russell 2000", "FTSE 100",
    "DAX", "Nikkei 225", "Hang Seng", "Shanghai Composite", "CAC 40",
    "Euro Stoxx 50", "IBOVESPA", "ASX 200", "TSX Composite",
];

const FALLBACK_CURRENCIES: &[&str] = &[
    "USD", "EUR", "GBP", "JPY", "CHF", "CAD", "AUD", "NZD",
    "CNY", "HKD", "SGD", "MXN", "BRL", "INR", "RUB", "ZAR",
    "KRW", "SEK", "NOK", "DKK", "PLN", "TRY", "AED", "THB",
];

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
