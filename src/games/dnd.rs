//! Dungeons & Dragons generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random D&D alignment
pub fn alignment() -> String {
    fetch_locale("dnd.alignment", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Lawful Good".to_string())
}

/// Generate a random D&D background
pub fn background() -> String {
    fetch_locale("dnd.background", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Acolyte".to_string())
}

/// Generate a random D&D city
pub fn city() -> String {
    fetch_locale("dnd.cities", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Waterdeep".to_string())
}

/// Generate a random D&D class (using 'klass' to avoid keyword conflict)
pub fn klass() -> String {
    fetch_locale("dnd.klasses", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Fighter".to_string())
}

/// Generate a random D&D language
pub fn language() -> String {
    fetch_locale("dnd.languages", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Common".to_string())
}

/// Generate a random D&D monster
pub fn monster() -> String {
    fetch_locale("dnd.monsters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Beholder".to_string())
}

/// Generate a random D&D race
pub fn race() -> String {
    fetch_locale("dnd.races", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Human".to_string())
}

/// Generate a random D&D melee weapon
pub fn melee_weapon() -> String {
    fetch_locale("dnd.melee_weapons", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Longsword".to_string())
}

/// Generate a random D&D ranged weapon
pub fn ranged_weapon() -> String {
    fetch_locale("dnd.ranged_weapons", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Longbow".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dnd_generation() {
        assert!(!alignment().is_empty());
        assert!(!background().is_empty());
        assert!(!city().is_empty());
        assert!(!klass().is_empty());
        assert!(!language().is_empty());
        assert!(!monster().is_empty());
        assert!(!race().is_empty());
        assert!(!melee_weapon().is_empty());
        assert!(!ranged_weapon().is_empty());
    }
}
