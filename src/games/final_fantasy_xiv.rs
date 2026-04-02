//! Final Fantasy XIV game generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random FFXIV character
pub fn character() -> String {
    fetch_locale("final_fantasy_xiv.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random FFXIV job/class
pub fn job() -> String {
    fetch_locale("final_fantasy_xiv.jobs", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_JOBS).to_string())
}

/// Generate a random FFXIV race
pub fn race() -> String {
    fetch_locale("final_fantasy_xiv.races", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_RACES).to_string())
}

/// Generate a random FFXIV location
pub fn location() -> String {
    fetch_locale("final_fantasy_xiv.locations", "en")
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
    "Hyur",
    "Elezen",
    "Lalafell",
    "Miqo'te",
    "Roegadyn",
    "Au Ra",
    "Hrothgar",
    "Viera",
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
