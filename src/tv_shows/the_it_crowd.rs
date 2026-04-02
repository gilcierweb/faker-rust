//! The IT Crowd TV show generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random The IT Crowd character
pub fn character() -> String {
    fetch_locale("the_it_crowd.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random The IT Crowd quote
pub fn quote() -> String {
    fetch_locale("the_it_crowd.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

/// Generate a random The IT Crowd email
pub fn email() -> String {
    fetch_locale("the_it_crowd.emails", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_EMAILS).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Maurice Moss",
    "Roy Trenneman",
    "Jen Barber",
    "Richmond Avenal",
    "Douglas Reynholm",
    "Denholm Reynholm",
    "Noel",
    "Big Julie",
    "Derek Pippen",
];

const FALLBACK_QUOTES: &[&str] = &[
    "Have you tried turning it off and on again?",
    "Hello, IT. Have you tried turning it off and on again?",
    "I'm disabled!",
    "Fire! Fire!",
    "0118 999 881 999 119 725... 3",
    "I came here to drink milk and kick ass... and I've just finished my milk.",
    "The internet is coming!",
    "It's so light!",
];

const FALLBACK_EMAILS: &[&str] = &[
    "roy.trenneman@ Reynholm.co.uk",
    "moss.maurice@reynholm.co.uk",
    "jen.barber@reynholm.co.uk",
    "richmond.avenal@reynholm.co.uk",
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
    fn test_email() {
        assert!(!email().is_empty());
    }
}
