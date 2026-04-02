//! New Girl TV show generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random New Girl character
pub fn character() -> String {
    fetch_locale("new_girl.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random New Girl quote
pub fn quote() -> String {
    fetch_locale("new_girl.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Jessica Day",
    "Nick Miller",
    "Winston Bishop",
    "Schmidt",
    "Cece Parekh",
    "Coach",
    "Reagan Lucas",
    "Aly Nelson",
    "Sam Sweeney",
    "Robbie McFerrin",
    "Bob Day",
    "Nadia",
];

const FALLBACK_QUOTES: &[&str] = &[
    "I brake for birds. I rock a lot of polka dots.",
    "I have the focus of a coked-up Russell Crowe.",
    "Schmidt happens!",
    "Youths!",
    "I'm gonna be a model!",
    "I am not a successful adult. I don't eat vegetables or respect my body.",
    "These are my night peanuts.",
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
}
