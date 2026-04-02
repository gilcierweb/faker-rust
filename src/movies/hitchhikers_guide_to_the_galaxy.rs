//! Hitchhiker's Guide to the Galaxy generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Hitchhiker's Guide character
pub fn character() -> String {
    fetch_locale("hitchhikers_guide_to_the_galaxy.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())

}

/// Generate a random Hitchhiker's Guide quote
pub fn quote() -> String {
    fetch_locale("hitchhikers_guide_to_the_galaxy.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

/// Generate a random Hitchhiker's Guide location
pub fn location() -> String {
    fetch_locale("hitchhikers_guide_to_the_galaxy.locations", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_LOCATIONS).to_string())
}

/// Generate a random Hitchhiker's Guide species
pub fn species() -> String {
    fetch_locale("hitchhikers_guide_to_the_galaxy.species", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_SPECIES).to_string())
}

/// Generate a random Hitchhiker's Guide starship
pub fn starship() -> String {
    fetch_locale("hitchhikers_guide_to_the_galaxy.starships", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_STARSHIPS).to_string())
}

/// Generate a random Hitchhiker's Guide planet
pub fn planet() -> String {
    fetch_locale("hitchhikers_guide_to_the_galaxy.planets", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_PLANETS).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Arthur Dent",
    "Ford Prefect",
    "Zaphod Beeblebrox",
    "Trillian",
    "Marvin the Paranoid Android",
    "Slartibartfast",
    "Deep Thought",
    "Eddie",
    "Fenchurch",
    "Prostetnic Vogon Jeltz",
    "Vann Harl",
    "Humma Kavula",
    "Questular Rontok",
];

const FALLBACK_QUOTES: &[&str] = &[
    "Don't Panic!",
    "So long, and thanks for all the fish.",
    "The answer to life, the universe, and everything is 42.",
    "Time is an illusion. Lunchtime doubly so.",
    "I love deadlines. I like the whooshing sound they make as they fly by.",
    "In the beginning the Universe was created. This has made a lot of people very angry and been widely regarded as a bad move.",
    "Space is big. You just won't believe how vastly, hugely, mind-bogglingly big it is.",
];

const FALLBACK_LOCATIONS: &[&str] = &[
    "The Restaurant at the End of the Universe",
    "Milliways",
    "The Total Perspective Vortex",
    "The Infinite Improbability Drive",
    "The Guide's Offices",
    "Arthur's House",
    "The Vogon Constructor Fleet",
    "The Heart of Gold",
];

const FALLBACK_SPECIES: &[&str] = &[
    "Human",
    "Betelgeusian",
    "Vogon",
    "Dolphins",
    "Mice",
    "Babel Fish",
    "Krikkiters",
    "Magratheans",
    "Haggunenons",
    "Bartledanians",
];

const FALLBACK_STARSHIPS: &[&str] = &[
    "Heart of Gold",
    "Vogon Constructor Fleet",
    "The Improbability Drive",
    "The Tanngrisnir",
    "The Heart of Gold",
    "The Krikkit One",
];

const FALLBACK_PLANETS: &[&str] = &[
    "Earth",
    "Magrathea",
    "Vogsphere",
    "Betelgeuse",
    "Krikkit",
    "Nano",
    "Stavromula Beta",
    "Milliways",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character() {
        assert!(!character().is_empty());
    }

    #[test]
    fn test_quote() {
        assert!(!quote().is_empty());
    }

    #[test]
    fn test_location() {
        assert!(!location().is_empty());
    }

    #[test]
    fn test_species() {
        assert!(!species().is_empty());
    }

    #[test]
    fn test_starship() {
        assert!(!starship().is_empty());
    }

    #[test]
    fn test_planet() {
        assert!(!planet().is_empty());
    }
}
