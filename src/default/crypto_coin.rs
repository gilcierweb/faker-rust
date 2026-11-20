//! Crypto coin generator (compatibility re-export of [`crate::crypto`]).

/// Generate a random cryptocurrency coin name.
pub use crate::crypto::coin;

/// Generate a random cryptocurrency symbol (also called an acronym).
pub use crate::crypto::symbol;

/// Alias kept for backwards compatibility with the old `crypto_coin` API.
pub use crate::crypto::symbol as acronym;

// --- gem parity generators (automated, locale-backed) ---
fn parity_pick(key: &str) -> String {
    crate::locale::fetch_locale(key, &crate::config::FakerConfig::locale())
        .map(|v| crate::base::sample(&v))
        .unwrap_or_default()
}

/// Gem-compatible generator: `crypto_coin.coin`.
pub fn coin_array() -> String {
    parity_pick("crypto_coin.coin")
}
// --- gem parity generators (gem record fields) ---
fn coin_fields() -> Vec<String> {
    coin_array()
        .split(',')
        .map(|part| part.trim().to_string())
        .collect()
}

/// Gem-compatible generator: crypto coin name.
pub fn coin_name() -> String {
    coin_fields().first().cloned().unwrap_or_default()
}

/// Gem-compatible generator: crypto coin logo URL.
pub fn url_logo() -> String {
    coin_fields().get(2).cloned().unwrap_or_default()
}

/// Gem-compatible generator: a single crypto coin record as an object.
pub fn coin_hash() -> serde_json::Value {
    let fields = coin_fields();
    let value =
        |index: usize| serde_json::Value::String(fields.get(index).cloned().unwrap_or_default());
    let mut record = serde_json::Map::new();
    record.insert("name".into(), value(0));
    record.insert("acronym".into(), value(1));
    record.insert("url_logo".into(), value(2));
    serde_json::Value::Object(record)
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
