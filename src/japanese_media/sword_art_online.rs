//! Sword Art Online anime/manga generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Sword Art Online character
pub fn character() -> String {
    fetch_locale(
        "sword_art_online.characters",
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Sword Art Online game/world
pub fn game() -> String {
    fetch_locale(
        "sword_art_online.games",
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(|| sample(FALLBACK_GAMES).to_string())
}

/// Generate a random Sword Art Online location
pub fn location() -> String {
    fetch_locale(
        "sword_art_online.locations",
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(|| sample(FALLBACK_LOCATIONS).to_string())
}

/// Generate a random Sword Art Online sword skill
pub fn skill() -> String {
    fetch_locale(
        "sword_art_online.skills",
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(|| sample(FALLBACK_SKILLS).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Kirito",
    "Asuna",
    "Leafa",
    "Sinon",
    "Yui",
    "Silica",
    "Lisbeth",
    "Klein",
    "Agil",
    "Heathcliff",
    "Suguha Kirigaya",
    "Yuuki",
    "Eugeo",
    "Alice Zuberg",
    "Administrator",
    "Death Gun",
    "Nobuyuki Sugou",
];

const FALLBACK_GAMES: &[&str] = &[
    "Sword Art Online",
    "ALfheim Online",
    "Gun Gale Online",
    "Project Alicization",
    "Ordinal Scale",
    "Underworld",
];

const FALLBACK_LOCATIONS: &[&str] = &[
    "Aincrad",
    "Floor 1",
    "Floor 50",
    "Floor 75",
    "Floor 100",
    "Swilvane",
    "Alne",
    "World Tree",
    "Gun Gale City",
    "Central Cathedral",
];

const FALLBACK_SKILLS: &[&str] = &[
    "Starburst Stream",
    "The Eclipse",
    "Vertical Square",
    "Horizontal Square",
    "Vorpal Strike",
    "Deadly Sins",
    " Mother's Rosario",
    "Dual Blades",
    "Elucidator",
    "Dark Repulser",
    "Lambent Light",
];

// --- gem parity generators (automated, locale-backed) ---
fn parity_pick(key: &str) -> String {
    crate::locale::fetch_locale(key, &crate::config::FakerConfig::locale())
        .map(|v| crate::base::sample(&v))
        .unwrap_or_default()
}

/// Gem-compatible generator: `sword_art_online.game_name`.
pub fn game_name() -> String {
    parity_pick("sword_art_online.game_name")
}

/// Gem-compatible generator: `sword_art_online.item`.
pub fn item() -> String {
    parity_pick("sword_art_online.item")
}

/// Gem-compatible generator: `sword_art_online.real_name`.
pub fn real_name() -> String {
    parity_pick("sword_art_online.real_name")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character() {
        assert!(!character().is_empty());
    }

    #[test]
    fn test_game() {
        assert!(!game().is_empty());
    }

    #[test]
    fn test_location() {
        assert!(!location().is_empty());
    }

    #[test]
    fn test_skill() {
        assert!(!skill().is_empty());
    }
}
