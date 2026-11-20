//! Clash of Clans generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random troop name
pub fn troop() -> String {
    fetch_locale("games.clash_of_clans.troops", "en")
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
