//! Final Fantasy XIV game generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random FFXIV character
pub fn character() -> String {
    fetch_locale(
        "games.final_fantasy_xiv.characters",
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random FFXIV job/class
pub fn job() -> String {
    fetch_locale(
        "games.final_fantasy_xiv.jobs",
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(|| sample(FALLBACK_JOBS).to_string())
}

/// Generate a random FFXIV race
pub fn race() -> String {
    fetch_locale(
        "games.final_fantasy_xiv.races",
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(|| sample(FALLBACK_RACES).to_string())
}

/// Generate a random FFXIV location
pub fn location() -> String {
    fetch_locale(
        "games.final_fantasy_xiv.zones",
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(|| sample(FALLBACK_LOCATIONS).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Warrior of Light",
    "Alphinaud Leveilleur",
    "Alisaie Leveilleur",
    "Thancred Waters",
    "Y'shtola Rhul",
    "Urianger Augurelt",
    "Tataru Taru",
    "Cid Garlond",
    "Nero tol Scaeva",
    "Gaius van Baelsar",
    "Zenos yae Galvus",
    "Hien Rijin",
    "Lyse Hext",
    "Yda Hext",
    "Papalymo Totolymo",
    "Yugiri Mistwalker",
    "Gosetsu Daito",
    "Estinien Varlineau",
    "Aymeric de Borel",
    "Lucia goe Junius",
];

const FALLBACK_JOBS: &[&str] = &[
    "Paladin",
    "Warrior",
    "Dark Knight",
    "Gunbreaker",
    "White Mage",
    "Scholar",
    "Astrologian",
    "Sage",
    "Monk",
    "Dragoon",
    "Ninja",
    "Samurai",
    "Reaper",
    "Viper",
    "Bard",
    "Machinist",
    "Dancer",
    "Black Mage",
    "Summoner",
    "Red Mage",
    "Pictomancer",
    "Blue Mage",
];

const FALLBACK_RACES: &[&str] = &[
    "Hyur", "Elezen", "Lalafell", "Miqo'te", "Roegadyn", "Au Ra", "Hrothgar", "Viera",
];

const FALLBACK_LOCATIONS: &[&str] = &[
    "Limsa Lominsa",
    "Gridania",
    "Ul'dah",
    "Ishgard",
    "Kugane",
    "Sharlayan",
    "Garlemald",
    "The Crystarium",
    "Old Sharlayan",
    "Radz-at-Han",
    "Tuliyollal",
];

// --- gem parity generators (automated, locale-backed) ---
fn parity_pick(key: &str) -> String {
    crate::locale::fetch_locale(key, &crate::config::FakerConfig::locale())
        .map(|v| crate::base::sample(&v))
        .unwrap_or_default()
}

/// Gem-compatible generator: `games.final_fantasy_xiv.data_centers`.
pub fn data_center() -> String {
    parity_pick("games.final_fantasy_xiv.data_centers")
}

/// Gem-compatible generator: `games.final_fantasy_xiv.zones`.
pub fn zone() -> String {
    parity_pick("games.final_fantasy_xiv.zones")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character() {
        assert!(!character().is_empty());
    }

    #[test]
    fn test_job() {
        assert!(!job().is_empty());
    }

    #[test]
    fn test_race() {
        assert!(!race().is_empty());
    }

    #[test]
    fn test_location() {
        assert!(!location().is_empty());
    }
}
