//! Breaking Bad TV show generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Breaking Bad character
pub fn character() -> String {
    fetch_locale("breaking_bad.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Breaking Bad quote
pub fn quote() -> String {
    fetch_locale("breaking_bad.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

/// Generate a random Breaking Bad episode
pub fn episode() -> String {
    fetch_locale("breaking_bad.episodes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_EPISODES).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Walter White",
    "Jesse Pinkman",
    "Skyler White",
    "Hank Schrader",
    "Marie Schrader",
    "Walter White Jr.",
    "Saul Goodman",
    "Gus Fring",
    "Mike Ehrmantraut",
    "Tuco Salamanca",
    "Hector Salamanca",
    "Jane Margolis",
    "Todd Alquist",
    "Lydia Rodarte-Quayle",
    "Steven Gomez",
];

const FALLBACK_QUOTES: &[&str] = &[
    "I am the one who knocks!",
    "Say my name.",
    "You're goddamn right.",
    "I did it for me.",
    "Tread lightly.",
    "Science, bitch!",
    "Yeah, Mr. White! Yeah, science!",
    "No more half measures.",
    "Better call Saul!",
];

const FALLBACK_EPISODES: &[&str] = &[
    "Pilot",
    "Cat's in the Bag...",
    "...And the Bag's in the River",
    "Cancer Man",
    "Gray Matter",
    "Crazy Handful of Nothin'",
    "A No-Rough-Stuff-Type Deal",
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
    fn test_episode() {
        assert!(!episode().is_empty());
    }
}
