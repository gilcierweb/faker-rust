//! Fallout generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Fallout character
pub fn character() -> String {
    fetch_locale("fallout.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Vault Boy".to_string())
}

/// Generate a random Fallout faction
pub fn faction() -> String {
    fetch_locale("fallout.factions", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Brotherhood of Steel".to_string())
}

/// Generate a random Fallout location
pub fn location() -> String {
    fetch_locale("fallout.locations", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Vault 101".to_string())
}

/// Generate a random Fallout quote
pub fn quote() -> String {
    fetch_locale("fallout.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "War. War never changes.".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fallout_generation() {
        assert!(!character().is_empty());
        assert!(!faction().is_empty());
        assert!(!location().is_empty());
        assert!(!quote().is_empty());
    }
}
