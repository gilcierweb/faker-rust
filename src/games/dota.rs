//! Dota 2 generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Dota 2 hero
pub fn hero() -> String {
    fetch_locale("dota.hero", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Anti-Mage".to_string())
}

/// Generate a random Dota 2 item
pub fn item() -> String {
    fetch_locale("dota.item", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Blink Dagger".to_string())
}

/// Generate a random Dota 2 team
pub fn team() -> String {
    fetch_locale("dota.team", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "OG".to_string())
}

/// Generate a random Dota 2 player
pub fn player() -> String {
    fetch_locale("dota.player", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "N0tail".to_string())
}

/// Generate a random Dota 2 building name
pub fn building() -> String {
    fetch_locale("dota.building", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Ancient".to_string())
}

/// Generate a random quote from a specific hero
/// If the hero name is not found, it might return a default quote.
pub fn quote(hero_name: &str) -> String {
    let key = format!("dota.{}.quote", hero_name.to_lowercase().replace(" ", "_"));
    fetch_locale(&key, "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "It's killing time.".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dota_generation() {
        assert!(!hero().is_empty());
        assert!(!item().is_empty());
        assert!(!team().is_empty());
        assert!(!player().is_empty());
        assert!(!building().is_empty());
        assert!(!quote("clockwerk").is_empty());
    }
}
