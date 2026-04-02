//! The Kingkiller Chronicle book generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Kingkiller Chronicle character
pub fn character() -> String {
    fetch_locale("kingkiller_chronicle.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Kingkiller Chronicle location
pub fn location() -> String {
    fetch_locale("kingkiller_chronicle.locations", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_LOCATIONS).to_string())
}

/// Generate a random Kingkiller Chronicle quote
pub fn quote() -> String {
    fetch_locale("kingkiller_chronicle.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Kvothe",
    "Denna",
    "Bast",
    "Chronicler",
    "Ambrose Jakis",
    "Wilem",
    "Simmon",
    "Fela",
    "Devi",
    "Elodin",
    "Hemme",
    "Lorren",
    "Auri",
    "Master Kilvin",
    "Master Elxa Dal",
    "Abenthy",
    "Trapis",
    "Skarpi",
];

const FALLBACK_LOCATIONS: &[&str] = &[
    "The University",
    "Imre",
    "The Archives",
    "The Underthing",
    "Tarbean",
    "The Eolian",
    "Anker's",
    "The Fishery",
    "Modeg",
    "Ademre",
    "The Fae",
];

const FALLBACK_QUOTES: &[&str] = &[
    "There are three things all wise men fear: the sea in storm, a night with no moon, and the anger of a gentle man.",
    "It's like everyone tells a story about themselves inside their own head. Always. All the time.",
    "You have to be a bit of a liar to tell a story the right way.",
    "Words are pale shadows of forgotten names.",
    "The best lies about me are the ones I told.",
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
    fn test_quote() {
        assert!(!quote().is_empty());
    }
}
