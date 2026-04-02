//! Warhammer Fantasy generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Warhammer Fantasy hero
pub fn hero() -> String {
    fetch_locale("warhammer_fantasy.heros", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Karl Franz".to_string())
}

/// Generate a random Warhammer Fantasy quote
pub fn quote() -> String {
    fetch_locale("warhammer_fantasy.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "For the Empire!".to_string())
}

/// Generate a random Warhammer Fantasy location
pub fn location() -> String {
    fetch_locale("warhammer_fantasy.locations", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Altdorf".to_string())
}

/// Generate a random Warhammer Fantasy faction
pub fn faction() -> String {
    fetch_locale("warhammer_fantasy.factions", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "The Empire".to_string())
}

/// Generate a random Warhammer Fantasy creature
pub fn creature() -> String {
    fetch_locale("warhammer_fantasy.creatures", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Griffon".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_warhammer_fantasy_generation() {
        assert!(!hero().is_empty());
        assert!(!quote().is_empty());
        assert!(!location().is_empty());
        assert!(!faction().is_empty());
        assert!(!creature().is_empty());
    }
}
