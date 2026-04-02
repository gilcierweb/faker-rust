//! Dragon Ball anime/manga generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Dragon Ball character
pub fn character() -> String {
    fetch_locale("dragon_ball.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Dragon Ball race
pub fn race() -> String {
    fetch_locale("dragon_ball.races", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_RACES).to_string())
}

/// Generate a random Dragon Ball planet
pub fn planet() -> String {
    fetch_locale("dragon_ball.planets", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_PLANETS).to_string())
}

/// Generate a random Dragon Ball transformation
pub fn transformation() -> String {
    fetch_locale("dragon_ball.transformations", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_TRANSFORMATIONS).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Goku",
    "Vegeta",
    "Gohan",
    "Piccolo",
    "Krillin",
    "Bulma",
    "Trunks",
    "Frieza",
    "Cell",
    "Majin Buu",
    "Beerus",
    "Whis",
    "Android 18",
    "Android 17",
    "Master Roshi",
    "Tien",
    "Yamcha",
    "Chi-Chi",
    "Goten",
    "Videl",
    "Mr. Satan",
    "Broly",
    "Jiren",
    "Hit",
];

const FALLBACK_RACES: &[&str] = &[
    "Saiyan",
    "Human",
    "Namekian",
    "Frieza Race",
    "Android",
    "Majin",
    "God of Destruction",
    "Angel",
];

const FALLBACK_PLANETS: &[&str] = &[
    "Earth",
    "Planet Vegeta",
    "Namek",
    "Frieza Planet 79",
    "King Kai's Planet",
    "Supreme Kai's World",
    "Beerus' Planet",
    "Vampa",
];

const FALLBACK_TRANSFORMATIONS: &[&str] = &[
    "Super Saiyan",
    "Super Saiyan 2",
    "Super Saiyan 3",
    "Super Saiyan God",
    "Super Saiyan Blue",
    "Ultra Instinct",
    "Super Saiyan 4",
    "Golden Frieza",
    "Ultimate Gohan",
    "Potential Unleashed",
    "Kaio-ken",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character() {
        assert!(!character().is_empty());
    }

    #[test]
    fn test_race() {
        assert!(!race().is_empty());
    }

    #[test]
    fn test_planet() {
        assert!(!planet().is_empty());
    }

    #[test]
    fn test_transformation() {
        assert!(!transformation().is_empty());
    }
}
