//! World of Warcraft generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random World of Warcraft hero
pub fn hero() -> String {
    fetch_locale("world_of_warcraft.heros", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Thrall".to_string())
}

/// Generate a random World of Warcraft quote
pub fn quote() -> String {
    fetch_locale("world_of_warcraft.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "For the Horde!".to_string())
}

/// Generate a random World of Warcraft class name
pub fn class_name() -> String {
    fetch_locale("world_of_warcraft.class_names", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Paladin".to_string())
}

/// Generate a random World of Warcraft race
pub fn race() -> String {
    fetch_locale("world_of_warcraft.races", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Orc".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wow_generation() {
        assert!(!hero().is_empty());
        assert!(!quote().is_empty());
        assert!(!class_name().is_empty());
        assert!(!race().is_empty());
    }
}
