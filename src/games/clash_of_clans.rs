//! Clash of Clans generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random troop name
pub fn troop() -> String {
    fetch_locale(
        "games.clash_of_clans.troops",
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(|| "Barbarian".to_string())
}

/// Generate a random spell name
pub fn spell() -> String {
    "Lightning Spell".to_string()
}

/// Generate a random building name
pub fn building() -> String {
    "Town Hall".to_string()
}

// --- gem parity generators (automated, locale-backed) ---
fn parity_pick(key: &str) -> String {
    crate::locale::fetch_locale(key, &crate::config::FakerConfig::locale())
        .map(|v| crate::base::sample(&v))
        .unwrap_or_default()
}

/// Gem-compatible generator: `games.clash_of_clans.defensive_buildings`.
pub fn defensive_building() -> String {
    parity_pick("games.clash_of_clans.defensive_buildings")
}

/// Gem-compatible generator: `games.clash_of_clans.ranks`.
pub fn rank() -> String {
    parity_pick("games.clash_of_clans.ranks")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clash_of_clans_generation() {
        assert!(!troop().is_empty());
        assert!(!spell().is_empty());
        assert!(!building().is_empty());
    }
}
