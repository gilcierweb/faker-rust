//! Star Trek TV show generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Star Trek character
pub fn character() -> String {
    fetch_locale("star_trek.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Star Trek location
pub fn location() -> String {
    fetch_locale("star_trek.locations", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_LOCATIONS).to_string())
}

/// Generate a random Star Trek species
pub fn species() -> String {
    fetch_locale("star_trek.species", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_SPECIES).to_string())
}

/// Generate a random Star Trek quote
pub fn quote() -> String {
    fetch_locale("star_trek.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "James T. Kirk",
    "Spock",
    "Leonard McCoy",
    "Montgomery Scott",
    "Hikaru Sulu",
    "Uhura",
    "Pavel Chekov",
    "Jean-Luc Picard",
    "William Riker",
    "Data",
    "Worf",
    "Geordi La Forge",
    "Deanna Troi",
    "Beverly Crusher",
    "Q",
    "Benjamin Sisko",
    "Kathryn Janeway",
];

const FALLBACK_LOCATIONS: &[&str] = &[
    "USS Enterprise",
    "USS Enterprise-D",
    "Deep Space Nine",
    "Vulcan",
    "Romulus",
    "Qo'noS",
    "Earth",
    "Starfleet Academy",
    "Holodeck",
    "Ten Forward",
    "Bridge",
];

const FALLBACK_SPECIES: &[&str] = &[
    "Human",
    "Vulcan",
    "Klingon",
    "Romulan",
    "Bajoran",
    "Cardassian",
    "Borg",
    "Ferengi",
    "Trill",
    "Betazoid",
    "Andorian",
    "Tellarite",
];

const FALLBACK_QUOTES: &[&str] = &[
    "Live long and prosper.",
    "Beam me up, Scotty!",
    "Resistance is futile.",
    "Make it so.",
    "Engage!",
    "I'm a doctor, not a...",
    "It's life, Jim, but not as we know it.",
    "Space: the final frontier.",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character() {
        assert!(!character().is_empty());
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
    fn test_quote() {
        assert!(!quote().is_empty());
    }
}
