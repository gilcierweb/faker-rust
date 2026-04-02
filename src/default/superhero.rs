//! Superhero generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random superhero name
pub fn name() -> String {
    fetch_locale("superhero.names", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_NAMES).to_string())
}

/// Generate a random superhero power
pub fn power() -> String {
    fetch_locale("superhero.powers", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_POWERS).to_string())
}

/// Generate a random superhero prefix
pub fn prefix() -> String {
    fetch_locale("superhero.prefixes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_PREFIXES).to_string())
}

/// Generate a random superhero suffix
pub fn suffix() -> String {
    fetch_locale("superhero.suffixes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_SUFFIXES).to_string())
}

/// Generate a random superhero descriptor
pub fn descriptor() -> String {
    fetch_locale("superhero.descriptors", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_DESCRIPTORS).to_string())
}

// Fallback data
const FALLBACK_NAMES: &[&str] = &[
    "Iron Man", "Captain America", "Thor", "Hulk", "Black Widow", "Hawkeye",
    "Spider-Man", "Doctor Strange", "Black Panther", "Captain Marvel",
    "Ant-Man", "Wasp", "Vision", "Scarlet Witch", "Falcon", "Winter Soldier",
];

const FALLBACK_POWERS: &[&str] = &[
    "Super Strength", "Flight", "Invisibility", "Teleportation", "Telepathy",
    "Shape Shifting", "Time Travel", "Elemental Control", "Healing Factor",
    "Laser Vision", "Super Speed", "Invulnerability", "Mind Control",
];

const FALLBACK_PREFIXES: &[&str] = &[
    "Captain", "Doctor", "Super", "Mega", "Ultra", "Dark", "Silver",
    "Iron", "Green", "Black", "Red", "Blue", "Golden", "The Amazing",
];

const FALLBACK_SUFFIXES: &[&str] = &[
    "Man", "Woman", "Girl", "Boy", "Knight", "Wing", "Storm", "Fire",
    "Ice", "Thunder", "Shadow", "Light", "Guardian", "Protector",
];

const FALLBACK_DESCRIPTORS: &[&str] = &[
    "The Mighty", "The Invincible", "The Unstoppable", "The Brave",
    "The Fearless", "The Legendary", "The Eternal", "The Radiant",
    "The Shadowy", "The Cosmic", "The Atomic", "The Mystic",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert!(!name().is_empty());
    }

    #[test]
    fn test_power() {
        assert!(!power().is_empty());
    }

    #[test]
    fn test_prefix() {
        assert!(!prefix().is_empty());
    }

    #[test]
    fn test_suffix() {
        assert!(!suffix().is_empty());
    }

    #[test]
    fn test_descriptor() {
        assert!(!descriptor().is_empty());
    }
}
