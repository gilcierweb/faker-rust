//! Stargate generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Stargate character
pub fn character() -> String {
    fetch_locale("stargate.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Jack O'Neill".to_string())
}

/// Generate a random Stargate planet
pub fn planet() -> String {
    fetch_locale("stargate.planets", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Abydos".to_string())
}

/// Generate a random Stargate quote
pub fn quote() -> String {
    fetch_locale("stargate.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Chevron seven is locked.".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stargate_generation() {
        assert!(!character().is_empty());
        assert!(!planet().is_empty());
        assert!(!quote().is_empty());
    }
}
