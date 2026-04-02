//! Futurama TV show generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Futurama character
pub fn character() -> String {
    fetch_locale("futurama.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Futurama quote
pub fn quote() -> String {
    fetch_locale("futurama.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

/// Generate a random Futurama location
pub fn location() -> String {
    fetch_locale("futurama.locations", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_LOCATIONS).to_string())
}

/// Generate a random Futurama catchphrase
pub fn catchphrase() -> String {
    fetch_locale("futurama.catchphrases", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CATCHPHRASES).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Philip J. Fry",
    "Turanga Leela",
    "Bender Bending Rodriguez",
    "Professor Hubert Farnsworth",
    "Dr. Zoidberg",
    "Amy Wong",
    "Hermes Conrad",
    "Zapp Brannigan",
    "Kif Kroker",
    "Nibbler",
    "Lord Nibbler",
    "Calculon",
    "Hypnotoad",
    "Robot Devil",
    "Lrrr",
    "Ndnd",
    "Scruffy",
];

const FALLBACK_QUOTES: &[&str] = &[
    "Good news, everyone!",
    "Bite my shiny metal ass!",
    "Shut up and take my money!",
    "I'm walking on sunshine!",
    "Woo-hoo!",
    "Hooray! I'm useful!",
    "Why not Zoidberg?",
    "That is indeed an interesting question.",
    "Kill all humans! (Except Fry)",
    "To shreds, you say?",
];

const FALLBACK_LOCATIONS: &[&str] = &[
    "Planet Express Headquarters",
    "New New York",
    "Omicron Persei 8",
    "Decapod 10",
    "Robot Planet",
    "Earth",
    "Mars",
    "The Moon",
    "Eternium",
    "Amazonia",
    "Omega 3",
    "Neutral Planet",
    "St. Louis",
];

const FALLBACK_CATCHPHRASES: &[&str] = &[
    "Good news, everyone!",
    "Bite my shiny metal ass!",
    "Why not Zoidberg?",
    "I'm back baby!",
    "Hooray!",
    "Sweet zombie Jesus!",
    "Kill all humans!",
    "Shut up and take my money!",
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
    fn test_catchphrase() {
        assert!(!catchphrase().is_empty());
    }
}
